use crate::soap;
use async_stream::stream;
use futures_core::stream::Stream;
use futures_util::{future::ready, stream::FuturesUnordered, StreamExt};
use schema::{
    transport::Error as TransportError,
    ws_discovery::{probe, probe_matches},
};
use std::{
    future::Future,
    iter,
    net::{IpAddr, Ipv4Addr, SocketAddr},
};
use thiserror::Error;
use tokio::{
    io,
    net::UdpSocket,
    time::{self, Duration, Instant},
};
use tracing::debug;
use url::Url;

#[derive(Debug, Error)]
pub enum Error {
    #[error("Network error: {0}")]
    Network(#[from] io::Error),

    #[error("(De)serialization error: {0}")]
    Serde(String),
}

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub struct Device {
    pub name: Option<String>,
    pub url: Url,
}

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
/// You can access each element on the stream concurrently as soon as devices respond:
///
/// ```
/// use onvif::discovery;
/// use futures_util::stream::StreamExt; // to use for_each_concurrent
///
/// const MAX_CONCURRENT_JUMPERS: usize = 100;
///
/// async {
///     discovery::discover(std::time::Duration::from_secs(1))
///         .await
///         .unwrap()
///         .for_each_concurrent(MAX_CONCURRENT_JUMPERS, |addr| {
///             async move {
///                 println!("Device found: {:?}", addr);
///             }
///         })
///         .await;
/// };
/// ```
///
/// Or you can await on a collection of unique devices found in one second:
///
/// ```
/// use onvif::discovery;
/// use futures_util::stream::StreamExt; // to use collect
/// use std::collections::HashSet;
///
/// async {
///     let devices = discovery::discover(std::time::Duration::from_secs(1))
///         .await
///         .unwrap()
///         .collect::<HashSet<_>>()
///         .await;
///
///     println!("Devices found: {:?}", devices);
/// };
/// ```
pub async fn discover(duration: Duration) -> Result<impl Stream<Item = Device>, Error> {
    let probe = build_probe();
    let probe_xml = yaserde::ser::to_string(&probe).map_err(Error::Serde)?;

    debug!("Probe XML: {}", probe_xml);

    let socket = {
        const LOCAL_IPV4_ADDR: Ipv4Addr = Ipv4Addr::UNSPECIFIED;
        const LOCAL_PORT: u16 = 0;

        const MULTI_IPV4_ADDR: Ipv4Addr = Ipv4Addr::new(239, 255, 255, 250);
        const MULTI_PORT: u16 = 3702;

        let local_socket_addr = SocketAddr::new(IpAddr::V4(LOCAL_IPV4_ADDR), LOCAL_PORT);
        let multi_socket_addr = SocketAddr::new(IpAddr::V4(MULTI_IPV4_ADDR), MULTI_PORT);

        let socket = UdpSocket::bind(local_socket_addr).await?;
        socket.join_multicast_v4(MULTI_IPV4_ADDR, LOCAL_IPV4_ADDR)?;
        socket
            .send_to(probe_xml.as_bytes(), multi_socket_addr)
            .await?;

        socket
    };

    Ok(stream! {
        let probe = &probe;
        let socket = &socket;

        let start = Instant::now();
        loop {
            let elapsed = start.elapsed();
            if elapsed >= duration {
                break;
            }

            let timeout = duration - elapsed;

            // Separate async block to be able to short-circuit on `None`.
            let try_produce_item = async move {
                let xml = recv_string(socket, timeout).await.ok()?;
                debug!("Probe match XML: {}", xml);
                let envelope = yaserde::de::from_str::<probe_matches::Envelope>(&xml).ok()?;
                if envelope.header.relates_to != probe.header.message_id {
                    return None;
                }
                get_responding_addr(envelope, is_addr_responding).await
            };

            if let Some(item) = try_produce_item.await {
                yield item;
            }
        }
    })
}

async fn recv_string(s: &UdpSocket, timeout: Duration) -> io::Result<String> {
    let mut buf = vec![0; 16 * 1024];
    let (len, _src) = time::timeout(timeout, s.recv_from(&mut buf)).await??;

    Ok(String::from_utf8_lossy(&buf[..len]).to_string())
}

async fn get_responding_addr<F, Fut>(
    envelope: probe_matches::Envelope,
    check_addr: F,
) -> Option<Device>
where
    F: Fn(Url) -> Fut + Copy,
    Fut: Future<Output = bool>,
{
    envelope
        .body
        .probe_matches
        .probe_match
        .iter()
        .filter(|probe_match| {
            probe_match
                .find_in_scopes("onvif://www.onvif.org")
                .is_some()
        })
        .flat_map(|probe_match| {
            probe_match
                .x_addrs()
                .into_iter()
                .zip(iter::repeat(probe_match.name()))
        })
        .map(|(url, name)| async move {
            check_addr(url.clone()).await.then(|| {
                debug!("Responding addr: {:?}", url);
                Device { name, url }
            })
        })
        .collect::<FuturesUnordered<_>>()
        .filter_map(ready)
        .next()
        .await
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

async fn is_addr_responding(uri: Url) -> bool {
    matches!(
        schema::devicemgmt::get_system_date_and_time(
            &soap::client::ClientBuilder::new(&uri)
                .timeout(Duration::from_millis(500))
                .build(),
            &Default::default(),
        )
        .await,
        Ok(_) | Err(TransportError::Authorization(_))
    )
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
                            <d:Scopes>onvif://www.onvif.org/name/MyCamera2000</d:Scopes>
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

    async fn is_addr_responding(uri: Url) -> bool {
        let responding_addrs = vec![
            "http://addr_10".parse().unwrap(),
            "http://addr_21".parse().unwrap(),
            "http://addr_30".parse().unwrap(),
        ];

        responding_addrs.contains(&uri)
    }

    let actual = input
        .iter()
        .filter_map(|xml| yaserde::de::from_str::<probe_matches::Envelope>(xml).ok())
        .filter(|envelope| envelope.header.relates_to == our_uuid)
        .filter_map(|envelope| {
            tokio::runtime::Runtime::new()
                .unwrap()
                .block_on(get_responding_addr(envelope, is_addr_responding))
        })
        .collect::<Vec<_>>();

    assert_eq!(actual.len(), 2);

    // OK: message UUID matches and addr responds
    assert!(actual.contains(&Device {
        url: Url::parse("http://addr_10").unwrap(),
        name: Some("MyCamera2000".to_string())
    }));

    // OK: message UUID matches and one of addresses responds
    assert!(
        actual.contains(&Device {
            url: Url::parse("http://addr_21").unwrap(),
            name: Some("MyCamera2000".to_string())
        }) || actual.contains(&Device {
            url: Url::parse("http://addr_22").unwrap(),
            name: Some("MyCamera2000".to_string())
        })
    );

    // BAD: wrong message UUID
    assert!(!actual.contains(&Device {
        url: Url::parse("http://addr_30").unwrap(),
        name: Some("MyCamera2000".to_string())
    }));
}
