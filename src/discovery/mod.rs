use crate::{
    schema::{
        self,
        ws_discovery::{probe, probe_matches},
    },
    soap,
};
use async_std::{pin::Pin, stream::Stream};
use std::net::{IpAddr, Ipv4Addr, SocketAddr};

#[derive(Debug)]
pub enum Error {
    Network(std::io::Error),
    Internal(String),
}

type StringStream = Pin<Box<dyn Stream<Item = String>>>;

/// Discovers devices on a local network asynchronously using WS-discovery.
///
/// Internally it sends a multicast probe and waits for responses for a specified amount of time.
/// The result is a stream of discovered devices one address per device.
/// The stream is terminated after provided amount of time.
///
/// There are many different ways to iterate over and process the values in a `Stream`
/// https://rust-lang.github.io/async-book/05_streams/02_iteration_and_concurrency.html
///
/// # Examples
///
/// You can access each element on the stream concurrently as elements become available:
///
/// ```
/// use onvif_rs::discovery;
/// use futures::stream::StreamExt; // to use for_each_concurrent
///
/// const MAX_CONCURRENT_JUMPERS: usize = 100;
///
/// async {
///     discovery::discover(std::time::Duration::from_secs(1))
///         .await
///         .unwrap()
///         .for_each_concurrent(MAX_CONCURRENT_JUMPERS, |addr| {
///             async move {
///                 println!("Device found at address: {}", addr);
///             }
///         })
///         .await;
/// };
/// ```
///
/// Or you can await on a collection of devices found in one second:
///
/// ```
/// use onvif_rs::discovery;
/// use futures::stream::StreamExt; // to use collect
///
/// async {
///     let addrs = discovery::discover(std::time::Duration::from_secs(1))
///         .await
///         .unwrap()
///         .collect::<Vec<_>>()
///         .await;
///
///     println!("Devices found: {:?}", addrs);
/// };
/// ```
pub async fn discover(duration: std::time::Duration) -> Result<StringStream, Error> {
    let probe = build_probe();
    let probe_xml = yaserde::ser::to_string(&probe).map_err(Error::Internal)?;

    let socket = (|| {
        async {
            const LOCAL_IPV4_ADDR: Ipv4Addr = Ipv4Addr::UNSPECIFIED;
            const LOCAL_PORT: u16 = 0;

            const MULTI_IPV4_ADDR: Ipv4Addr = Ipv4Addr::new(239, 255, 255, 250);
            const MULTI_PORT: u16 = 3702;

            let local_socket_addr = SocketAddr::new(IpAddr::V4(LOCAL_IPV4_ADDR), LOCAL_PORT);
            let multi_socket_addr = SocketAddr::new(IpAddr::V4(MULTI_IPV4_ADDR), MULTI_PORT);

            let socket = async_std::net::UdpSocket::bind(local_socket_addr).await?;
            socket.join_multicast_v4(MULTI_IPV4_ADDR, LOCAL_IPV4_ADDR)?;
            socket
                .send_to(&probe_xml.as_bytes(), multi_socket_addr)
                .await?;

            Ok(socket)
        }
    })()
    .await
    .map_err(Error::Network)?;

    let stream = {
        use async_std::stream::StreamExt;

        // Make an async stream of XML's
        futures::stream::unfold(socket, |s| async { Some((recv_string(&s).await, s)) })
            .filter_map(|string| string.ok())
            .filter_map(|xml| yaserde::de::from_str::<probe_matches::Envelope>(&xml).ok())
            .filter(move |envelope| envelope.header.relates_to == probe.header.message_id)
            .filter_map(|envelope| get_responding_addr(&envelope, is_addr_responding))
            // Blend in an interval stream to implement timeout
            // Let our payload stream contain Some's and timeout stream contain None's
            .map(|payload| Some(payload))
            .merge(async_std::stream::interval(duration).map(|_| None))
            // Terminate stream when the first None is received
            .take_while(|event| event.is_some())
            .filter_map(|event| event)
    };

    {
        use futures::stream::StreamExt;

        Ok(stream.boxed())
    }
}

async fn recv_string(s: &async_std::net::UdpSocket) -> std::io::Result<String> {
    let mut buf = vec![0; 16 * 1024];

    let (len, _src) = s.recv_from(&mut buf).await?;

    Ok(String::from_utf8_lossy(&buf[..len]).to_string())
}

fn get_responding_addr<F>(envelope: &probe_matches::Envelope, check_addr: F) -> Option<String>
where
    F: Fn(&str) -> bool,
{
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
}

fn build_probe() -> probe::Envelope {
    use probe::*;

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

    let actual = input
        .iter()
        .filter_map(|xml| yaserde::de::from_str::<probe_matches::Envelope>(&xml).ok())
        .filter(|envelope| envelope.header.relates_to == our_uuid)
        .filter_map(|envelope| {
            get_responding_addr(&envelope, |addr| responding_addrs.contains(&addr))
        })
        .collect::<Vec<_>>();

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
