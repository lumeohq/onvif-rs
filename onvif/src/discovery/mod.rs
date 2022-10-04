use futures_core::stream::Stream;
use schema::ws_discovery::{probe, probe_matches};
use std::{
    collections::HashSet,
    fmt::{Debug, Formatter},
    net::{IpAddr, Ipv4Addr, SocketAddr},
    sync::Arc,
};
use thiserror::Error;
use tokio::{
    io,
    net::UdpSocket,
    sync::mpsc::channel,
    time::{timeout, Duration},
};
use tokio_stream::wrappers::ReceiverStream;
use tracing::debug;
use url::Url;

use crate::utils::{display_list::DisplayList, hash::calculate_hash};

#[derive(Debug, Error)]
pub enum Error {
    #[error("Network error: {0}")]
    Network(#[from] io::Error),

    #[error("(De)serialization error: {0}")]
    Serde(String),

    #[error("Unsupported feature: {0}")]
    Unsupported(String),
}

#[derive(Clone, Eq, Hash, PartialEq)]
pub struct Device {
    pub name: Option<String>,
    pub urls: Vec<Url>,
}

impl Debug for Device {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Device")
            .field("name", &self.name)
            .field("url", &DisplayList(&self.urls))
            .finish()
    }
}

#[derive(Debug, Clone)]
pub struct DiscoveryBuilder {
    duration: Duration,
    listen_address: IpAddr,
}

impl Default for DiscoveryBuilder {
    fn default() -> Self {
        Self {
            duration: Duration::from_secs(5),
            listen_address: IpAddr::V4(Ipv4Addr::UNSPECIFIED),
        }
    }
}

impl DiscoveryBuilder {
    /// How long to listen for the responses from the network.
    pub fn duration(&mut self, duration: Duration) -> &mut Self {
        self.duration = duration;
        self
    }

    /// Address to listen on.
    ///
    /// By default, it is 0.0.0.0 which is fine for a single-NIC case. With multiple NICs, it's
    /// problematic because 0.0.0.0 is routed to only one NIC, but you may want to run the discovery
    /// on a specific network.
    pub fn listen_address(&mut self, listen_address: IpAddr) -> &mut Self {
        self.listen_address = listen_address;
        self
    }

    /// Discovers devices on a local network asynchronously using WS-discovery.
    ///
    /// Internally it sends a multicast probe and waits for responses for a specified amount of time.
    /// The result is a stream of discovered devices.
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
    ///     discovery::DiscoveryBuilder::default().run()
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
    ///     let devices = discovery::DiscoveryBuilder::default().run()
    ///         .await
    ///         .unwrap()
    ///         .collect::<HashSet<_>>()
    ///         .await;
    ///
    ///     println!("Devices found: {:?}", devices);
    /// };
    /// ```
    pub async fn run(&self) -> Result<impl Stream<Item = Device>, Error> {
        let Self {
            duration,
            listen_address,
        } = self;

        let probe = Arc::new(build_probe());
        let probe_xml = yaserde::ser::to_string(probe.as_ref()).map_err(Error::Serde)?;

        debug!("Probe XML: {}", probe_xml);

        let socket = {
            const LOCAL_PORT: u16 = 0;

            const MULTI_IPV4_ADDR: Ipv4Addr = Ipv4Addr::new(239, 255, 255, 250);
            const MULTI_PORT: u16 = 3702;

            let local_socket_addr = SocketAddr::new(*listen_address, LOCAL_PORT);
            let multi_socket_addr = SocketAddr::new(IpAddr::V4(MULTI_IPV4_ADDR), MULTI_PORT);

            let socket = UdpSocket::bind(local_socket_addr).await?;

            match listen_address {
                IpAddr::V4(addr) => socket.join_multicast_v4(MULTI_IPV4_ADDR, *addr)?,
                IpAddr::V6(_) => return Err(Error::Unsupported("Discovery with IPv6".to_owned())),
            }

            socket
                .send_to(probe_xml.as_bytes(), multi_socket_addr)
                .await?;

            socket
        };

        let (device_sender, device_receiver) = channel(32);
        let device_receiver = ReceiverStream::new(device_receiver);

        let mut known_responses = HashSet::new();

        let produce_devices = async move {
            while let Ok((xml, src)) = recv_string(&socket).await {
                if !known_responses.insert(calculate_hash(&xml)) {
                    debug!("Duplicate response from {src}, skipping ...");
                    continue;
                }

                debug!("Probe match XML: {}", xml,);

                let envelope = match yaserde::de::from_str::<probe_matches::Envelope>(&xml) {
                    Ok(envelope) => envelope,
                    Err(e) => {
                        debug!("Deserialization failed: {e}");
                        continue;
                    }
                };

                if envelope.header.relates_to != probe.header.message_id {
                    debug!("Unrelated message");
                    continue;
                }

                if let Some(device) = device_from_envelope(envelope) {
                    debug!("Found device {device:?}");
                    // It's ok to ignore the sending error as user can drop the receiver soon
                    // (for example, after the first device discovered).
                    let _ = device_sender.send(device).await;
                } else {
                    debug!("No devices found");
                }
            }
        };

        tokio::spawn(timeout(*duration, produce_devices));

        Ok(device_receiver)
    }
}

async fn recv_string(s: &UdpSocket) -> io::Result<(String, SocketAddr)> {
    let mut buf = vec![0; 16 * 1024];
    let (len, src) = s.recv_from(&mut buf).await?;

    Ok((String::from_utf8_lossy(&buf[..len]).to_string(), src))
}

fn device_from_envelope(envelope: probe_matches::Envelope) -> Option<Device> {
    let onvif_probe_match = envelope
        .body
        .probe_matches
        .probe_match
        .iter()
        .find(|probe_match| {
            probe_match
                .find_in_scopes("onvif://www.onvif.org")
                .is_some()
        })?;

    let name = onvif_probe_match.name();
    let urls = onvif_probe_match.x_addrs();

    Some(Device { name, urls })
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
        make_xml(our_uuid, "http://addr_20 http://addr_21 http://addr_22"),
        make_xml(bad_uuid, "http://addr_30 http://addr_31"),
    ];

    let actual = input
        .iter()
        .filter_map(|xml| yaserde::de::from_str::<probe_matches::Envelope>(xml).ok())
        .filter(|envelope| envelope.header.relates_to == our_uuid)
        .filter_map(device_from_envelope)
        .collect::<Vec<_>>();

    assert_eq!(actual.len(), 1);

    // OK: message UUID matches and addr responds
    assert_eq!(
        actual,
        &[Device {
            urls: vec![
                Url::parse("http://addr_20").unwrap(),
                Url::parse("http://addr_21").unwrap(),
                Url::parse("http://addr_22").unwrap()
            ],
            name: Some("MyCamera2000".to_string())
        }]
    );
}
