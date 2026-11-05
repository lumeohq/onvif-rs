use super::*;
use crate::soap::client::{AuthType, ClientBuilder, Credentials};
use crate::utils::xml_eq::assert_xml_eq;
use schema::{
    soap_envelope::{FaultcodeEnum, Reasontext, Subcode},
    transport::{self, Transport},
};
use std::sync::{
    atomic::{AtomicUsize, Ordering},
    Arc,
};
use tokio::{
    io::{AsyncReadExt, AsyncWriteExt},
    net::{TcpListener, TcpStream},
};
use url::Url;

#[test]
fn test_soap() {
    let app_data = r#"
        <my:Book xmlns:my="http://www.example.my/schema">
            <my:Title>Such book</my:Title>
            <my:Pages>42</my:Pages>
        </my:Book>
        "#;

    let expected = r#"<?xml version="1.0" encoding="UTF-8"?>
        <s:Envelope xmlns:s="http://www.w3.org/2003/05/soap-envelope"
                    xmlns:my="http://www.example.my/schema">
            <s:Body>
                <my:Book>
                    <my:Title>Such book</my:Title>
                    <my:Pages>42</my:Pages>
                </my:Book>
            </s:Body>
        </s:Envelope>
        "#;

    let actual = soap(app_data, &None).unwrap();

    println!("{}", actual);
    println!("{}", expected);

    assert_xml_eq(actual.as_str(), expected);
}

#[test]
#[expect(non_local_definitions)]
fn test_unsoap() {
    #[derive(Default, Eq, PartialEq, Debug, YaDeserialize)]
    #[yaserde(prefix = "my", namespace = "my: http://www.example.my/schema")]
    pub struct Book {
        #[yaserde(prefix = "my", rename = "Title")]
        pub title: String,

        #[yaserde(prefix = "my", rename = "Pages")]
        pub pages: i32,
    }

    let input = r#"<?xml version="1.0" encoding="utf-8"?>
        <s:Envelope xmlns:s="http://www.w3.org/2003/05/soap-envelope"
                    xmlns:my="http://www.example.my/schema">
            <s:Body>
                <my:Book>
                    <my:Title>Such book</my:Title>
                    <my:Pages>42</my:Pages>
                </my:Book>
            </s:Body>
        </s:Envelope>
        "#;

    let actual = unsoap(input).unwrap();

    println!("{:?}", actual);

    let parsed: Book = yaserde::de::from_str(&actual).unwrap();

    assert_eq!(parsed.title, "Such book");
    assert_eq!(parsed.pages, 42);
}

#[test]
fn test_get_fault() {
    let response = r#"<?xml version="1.0" ?>
        <soapenv:Fault
            xmlns:soapenv="http://www.w3.org/2003/05/soap-envelope"
            xmlns:ter="http://www.onvif.org/ver10/error"
            xmlns:xs="http://www.w3.org/2000/10/XMLSchema">
            <soapenv:Code>
                <soapenv:Value>tns:DataEncodingUnknown</soapenv:Value>
                <soapenv:Subcode>
                    <soapenv:Value>ter:fault subcode</soapenv:Value>
                    <soapenv:Subcode>
                        <soapenv:Value>ter:fault subcode</soapenv:Value>
                    </soapenv:Subcode>
                </soapenv:Subcode>
            </soapenv:Code>
            <soapenv:Reason>
                <soapenv:Text xml:lang="en">fault reason 1</soapenv:Text>
                <soapenv:Text xml:lang="en">fault reason 2</soapenv:Text>
            </soapenv:Reason>
            <soapenv:Node>http://www.w3.org/2003/05/soap-envelope/node/ultimateReceiver</soapenv:Node>
            <soapenv:Role>http://www.w3.org/2003/05/soap-envelope/role/ultimateReceiver</soapenv:Role>
            <soapenv:Detail>
                <soapenv:Text>fault detail</soapenv:Text>
            </soapenv:Detail>
        </soapenv:Fault>
        "#;

    let envelope = xmltree::Element::parse(response.as_bytes()).unwrap();

    let fault = deserialize_fault(&envelope).unwrap();

    assert_eq!(
        fault.code.value,
        FaultcodeEnum("tns:DataEncodingUnknown".to_string())
    );
    assert_eq!(
        fault.code.subcode,
        Some(Subcode {
            value: "ter:fault subcode".to_string(),
            // subcode: Vec::new()
        })
    );
    assert_eq!(
        fault.reason.text,
        vec![
            Reasontext {
                lang: "en".to_string()
            },
            Reasontext {
                lang: "en".to_string()
            }
        ]
    );
}

#[tokio::test]
async fn error_does_not_restore_rejected_digest_attempts() {
    let listener = TcpListener::bind("127.0.0.1:0").await.unwrap();
    let uri = device_uri(&listener);
    let logins = Arc::new(AtomicUsize::new(0));
    tokio::spawn(serve_rejecting_digest_with_one_503(
        listener,
        logins.clone(),
    ));

    let client = ClientBuilder::new(&uri)
        .credentials(Some(credentials()))
        .auth_type(AuthType::Digest)
        .build();

    let result = client.request("<Good/>").await;
    assert!(
        matches!(result, Err(transport::Error::Other(_))),
        "{:?}",
        result
    );
    assert_eq!(logins.load(Ordering::SeqCst), 2);

    client.request("<Good/>").await.unwrap_err();
    assert_eq!(logins.load(Ordering::SeqCst), 4);
}

#[tokio::test]
async fn digest_auth_survives_requests_failing_with_400() {
    let listener = TcpListener::bind("127.0.0.1:0").await.unwrap();
    let uri = device_uri(&listener);
    tokio::spawn(serve_digest_with_single_use_nonces(listener));

    let client = ClientBuilder::new(&uri)
        .credentials(Some(credentials()))
        .auth_type(AuthType::Digest)
        .build();

    for _ in 0..3 {
        let result = client.request("<Bad/>").await;
        assert!(
            matches!(result, Err(transport::Error::Other(_))),
            "{:?}",
            result
        );
    }

    let result = client.request("<Good/>").await;
    assert!(result.is_ok(), "{:?}", result);
}

fn device_uri(listener: &TcpListener) -> Url {
    let address = listener.local_addr().unwrap();
    format!("http://{address}/onvif/device_service")
        .parse()
        .unwrap()
}

fn credentials() -> Credentials {
    Credentials {
        username: "admin".to_string(),
        password: "password".to_string(),
    }
}

/// Accepts each digest nonce once, like cameras that reject replayed nonces, and answers requests
/// for `<Bad/>` with 400 Bad Request.
async fn serve_digest_with_single_use_nonces(listener: TcpListener) {
    let mut issued_nonces = 0;
    let mut unused_nonce = None;

    loop {
        let (mut stream, _) = listener.accept().await.unwrap();
        let request = read_request(&mut stream).await;

        let response = match unused_nonce.take() {
            Some(nonce) if request.contains(&format!("nonce=\"{nonce}\"")) => {
                if request.contains("<Bad") {
                    http_response("400 Bad Request", "", "")
                } else {
                    let envelope = r#"<s:Envelope xmlns:s="http://www.w3.org/2003/05/soap-envelope"><s:Body><Good/></s:Body></s:Envelope>"#;
                    http_response("200 OK", "", envelope)
                }
            }
            _ => {
                issued_nonces += 1;
                let nonce = format!("nonce-{issued_nonces}");
                let challenge = format!(
                    "WWW-Authenticate: Digest realm=\"test\", qop=\"auth\", nonce=\"{nonce}\"\r\n"
                );
                unused_nonce = Some(nonce);
                http_response("401 Unauthorized", &challenge, "")
            }
        };

        stream.write_all(response.as_bytes()).await.unwrap();
    }
}

/// Rejects every digest login except the second, which gets 503 Service Unavailable.
async fn serve_rejecting_digest_with_one_503(listener: TcpListener, logins: Arc<AtomicUsize>) {
    loop {
        let (mut stream, _) = listener.accept().await.unwrap();
        let request = read_request(&mut stream).await;

        let is_second_login =
            has_digest_credentials(&request) && logins.fetch_add(1, Ordering::SeqCst) == 1;
        let response = if is_second_login {
            http_response("503 Service Unavailable", "", "")
        } else {
            let challenge =
                "WWW-Authenticate: Digest realm=\"test\", qop=\"auth\", nonce=\"nonce\"\r\n";
            http_response("401 Unauthorized", challenge, "")
        };

        stream.write_all(response.as_bytes()).await.unwrap();
    }
}

fn has_digest_credentials(request: &str) -> bool {
    request
        .to_ascii_lowercase()
        .contains("\r\nauthorization: digest ")
}

async fn read_request(stream: &mut TcpStream) -> String {
    let mut request = Vec::new();
    let mut buffer = [0; 4096];

    loop {
        let read = stream.read(&mut buffer).await.unwrap();
        assert_ne!(read, 0, "Connection closed before the request was complete");
        request.extend_from_slice(&buffer[..read]);

        let text = String::from_utf8_lossy(&request);
        let Some(headers_end) = text.find("\r\n\r\n") else {
            continue;
        };
        let content_length = text[..headers_end]
            .lines()
            .filter_map(|line| line.split_once(':'))
            .find(|(name, _)| name.eq_ignore_ascii_case("content-length"))
            .map_or(0, |(_, value)| value.trim().parse().unwrap());

        if request.len() >= headers_end + 4 + content_length {
            return text.into_owned();
        }
    }
}

fn http_response(status: &str, headers: &str, body: &str) -> String {
    format!(
        "HTTP/1.1 {status}\r\n{headers}Content-Length: {}\r\nConnection: close\r\n\r\n{body}",
        body.len()
    )
}
