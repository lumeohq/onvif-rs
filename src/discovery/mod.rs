use crate::{schema, soap};
use std::net::{IpAddr, Ipv4Addr, SocketAddr, UdpSocket};

#[derive(Debug)]
pub enum Error {
    Network(std::io::Error),
    Internal(String),
}

pub fn discover_sync(duration: std::time::Duration) -> Result<Vec<String>, Error> {
    let probe = build_probe();
    let probe_xml = yaserde::ser::to_string(&probe).map_err(Error::Internal)?;

    let socket = (|| {
        let local_ipv4_addr = Ipv4Addr::UNSPECIFIED;
        let multi_ipv4_addr = Ipv4Addr::new(239, 255, 255, 250);
        let local_socket_addr = SocketAddr::new(IpAddr::V4(local_ipv4_addr), 0);
        let multi_socket_addr = SocketAddr::new(IpAddr::V4(multi_ipv4_addr), 3702);

        let socket = UdpSocket::bind(local_socket_addr)?;

        socket.set_read_timeout(Some(std::time::Duration::from_millis(10)))?;

        socket.join_multicast_v4(&multi_ipv4_addr, &local_ipv4_addr)?;

        socket.send_to(&probe_xml.as_bytes(), multi_socket_addr)?;

        Ok(socket)
    })()
    .map_err(Error::Network)?;

    std::thread::sleep(duration);

    let xmls = recv_xmls(&socket);

    Ok(extract_xaddrs(
        xmls,
        &probe.header.message_id,
        is_addr_responding,
    ))
}

fn recv_xmls(socket: &UdpSocket) -> Vec<String> {
    let mut buf = [0; 16 * 1024];
    let mut xmls = vec![];

    while let Ok((amt, _src)) = socket.recv_from(&mut buf) {
        xmls.push(String::from_utf8_lossy(&buf[..amt]).to_string());
    }

    xmls
}

fn extract_xaddrs<F>(xmls: Vec<String>, message_id: &str, check_addr: F) -> Vec<String>
where
    F: Fn(&str) -> bool,
{
    xmls.iter()
        .map(|xml| yaserde::de::from_str::<schema::ws_discovery::probe_matches::Envelope>(&xml))
        .filter_map(|de_result| de_result.ok())
        .filter(|envelope| envelope.header.relates_to == message_id)
        .map(|envelope| {
            envelope
                .body
                .probe_matches
                .probe_match
                .iter()
                .flat_map(|probe_match| probe_match.x_addrs.split_whitespace())
                .map(|x| x.to_string())
                .filter(|addr| check_addr(addr))
                .take(1)
                .next()
        })
        .filter_map(|x| x)
        .collect()
}

fn build_probe() -> schema::ws_discovery::probe::Envelope {
    use schema::ws_discovery::probe::*;

    Envelope {
        header: Header {
            message_id: format!("uuid:{}", uuid::Uuid::new_v4()),
            action: "http://schemas.xmlsoap.org/ws/2005/04/discovery/Probe".into(),
            to: "urn:schemas-xmlsoap-org:ws:2005:04:discovery".into(),
        },
        ..Default::default()
    }
}

fn is_addr_responding(uri: &str) -> bool {
    schema::devicemgmt::get_system_date_and_time(
        &soap::client::Client::new(&uri),
        &Default::default(),
    )
    .is_ok()
}

#[test]
fn test_xaddrs_extraction() {
    fn make_xml(relates_to: &str, xaddrs: &str) -> String {
        format!(
            r#"<?xml version="1.0" encoding="UTF-8"?>
            <SOAP-ENV:Envelope
                        xmlns:SOAP-ENV="http://www.w3.org/2003/05/soap-envelope"
                        xmlns:wsa="http://schemas.xmlsoap.org/ws/2004/08/addressing"
                        xmlns:d="http://schemas.xmlsoap.org/ws/2005/04/discovery"
                        xmlns:dn="http://www.onvif.org/ver10/network/wsdl">
                <SOAP-ENV:Header>
                    <wsa:RelatesTo>{relates_to}</wsa:RelatesTo>
                </SOAP-ENV:Header>
                <SOAP-ENV:Body>
                    <d:ProbeMatches>
                        <d:ProbeMatch>
                            <d:XAddrs>http://something.else</d:XAddrs>
                        </d:ProbeMatch>
                        <d:ProbeMatch>
                            <d:XAddrs>{xaddrs}</d:XAddrs>
                        </d:ProbeMatch>
                    </d:ProbeMatches>
                </SOAP-ENV:Body>
            </SOAP-ENV:Envelope>
            "#,
            relates_to = relates_to,
            xaddrs = xaddrs
        )
    }

    let our_uuid = "uuid:84ede3de-7dec-11d0-c360-F01234567890";
    let bad_uuid = "uuid:84ede3de-7dec-11d0-c360-F00000000000";

    let input = vec![
        make_xml(our_uuid, "http://addr_10"),
        make_xml(our_uuid, "http://addr_20 http://addr_21 http://addr_22"),
        make_xml(bad_uuid, "http://addr_30 http://addr_31"),
    ];

    let responding_addrs = vec![
        "http://addr_10",
        "http://addr_21",
        "http://addr_22",
        "http://addr_30",
    ];

    let actual = extract_xaddrs(input, our_uuid, |addr| responding_addrs.contains(&addr));

    assert_eq!(actual.len(), 2);

    // OK: message UUID matches and addr responds
    assert!(actual.contains(&"http://addr_10".to_string()));

    // OK: message UUID matches and addr responds
    assert!(actual.contains(&"http://addr_21".to_string()));

    // BAD: message UUID matches and addr responds but we take only first one
    assert!(!actual.contains(&"http://addr_22".to_string()));

    // BAD: wrong message UUID
    assert!(!actual.contains(&"http://addr_30".to_string()));
}
