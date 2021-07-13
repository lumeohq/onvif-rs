pub mod probe {
    #[derive(Default, PartialEq, Debug, YaSerialize)]
    #[yaserde(
        prefix = "d",
        namespace = "d: http://schemas.xmlsoap.org/ws/2005/04/discovery"
    )]
    pub struct Probe {
        #[yaserde(prefix = "d", rename = "Types")]
        pub types: String,
    }

    #[derive(Default, PartialEq, Debug, YaSerialize)]
    #[yaserde(
        prefix = "s",
        namespace = "s: http://www.w3.org/2003/05/soap-envelope",
        namespace = "w: http://schemas.xmlsoap.org/ws/2004/08/addressing"
    )]
    pub struct Header {
        #[yaserde(prefix = "w", rename = "MessageID")]
        pub message_id: String,

        #[yaserde(prefix = "w", rename = "To")]
        pub to: String,

        #[yaserde(prefix = "w", rename = "Action")]
        pub action: String,
    }

    #[derive(Default, PartialEq, Debug, YaSerialize)]
    #[yaserde(
        prefix = "s",
        namespace = "s: http://www.w3.org/2003/05/soap-envelope",
        namespace = "d: http://schemas.xmlsoap.org/ws/2005/04/discovery"
    )]
    pub struct Body {
        #[yaserde(prefix = "d", rename = "Probe")]
        pub probe: Probe,
    }

    #[derive(Default, PartialEq, Debug, YaSerialize)]
    #[yaserde(
        prefix = "s",
        namespace = "s: http://www.w3.org/2003/05/soap-envelope",
        namespace = "d: http://schemas.xmlsoap.org/ws/2005/04/discovery",
        namespace = "w: http://schemas.xmlsoap.org/ws/2004/08/addressing"
    )]
    pub struct Envelope {
        #[yaserde(prefix = "s", rename = "Header")]
        pub header: Header,

        #[yaserde(prefix = "s", rename = "Body")]
        pub body: Body,
    }
}

pub mod probe_matches {
    use percent_encoding::percent_decode_str;
    use url::Url;

    #[derive(Default, PartialEq, Debug, YaDeserialize)]
    #[yaserde(
        prefix = "d",
        namespace = "d: http://schemas.xmlsoap.org/ws/2005/04/discovery"
    )]
    pub struct ProbeMatch {
        #[yaserde(prefix = "d", rename = "Types")]
        pub types: String,

        #[yaserde(prefix = "d", rename = "Scopes")]
        pub scopes: String,

        #[yaserde(prefix = "d", rename = "XAddrs")]
        pub x_addrs: String,
    }

    #[derive(Default, PartialEq, Debug, YaDeserialize)]
    #[yaserde(
        prefix = "d",
        namespace = "d: http://schemas.xmlsoap.org/ws/2005/04/discovery"
    )]
    pub struct ProbeMatches {
        #[yaserde(prefix = "d", rename = "ProbeMatch")]
        pub probe_match: Vec<ProbeMatch>,
    }

    #[derive(Default, PartialEq, Debug, YaDeserialize)]
    #[yaserde(
        prefix = "s",
        namespace = "s: http://www.w3.org/2003/05/soap-envelope",
        namespace = "w: http://schemas.xmlsoap.org/ws/2004/08/addressing"
    )]
    pub struct Header {
        #[yaserde(prefix = "w", rename = "RelatesTo")]
        pub relates_to: String,
    }

    #[derive(Default, PartialEq, Debug, YaDeserialize)]
    #[yaserde(
        prefix = "s",
        namespace = "s: http://www.w3.org/2003/05/soap-envelope",
        namespace = "d: http://schemas.xmlsoap.org/ws/2005/04/discovery"
    )]
    pub struct Body {
        #[yaserde(prefix = "d", rename = "ProbeMatches")]
        pub probe_matches: ProbeMatches,
    }

    #[derive(Default, PartialEq, Debug, YaDeserialize)]
    #[yaserde(prefix = "s", namespace = "s: http://www.w3.org/2003/05/soap-envelope")]
    pub struct Envelope {
        #[yaserde(prefix = "s", rename = "Header")]
        pub header: Header,

        #[yaserde(prefix = "s", rename = "Body")]
        pub body: Body,
    }

    impl ProbeMatch {
        pub fn types(&self) -> Vec<&str> {
            self.types.split_whitespace().collect()
        }

        pub fn scopes(&self) -> Vec<Url> {
            Self::split_string_to_urls(&self.scopes)
        }

        pub fn x_addrs(&self) -> Vec<Url> {
            Self::split_string_to_urls(&self.x_addrs)
        }

        pub fn name(&self) -> Option<String> {
            self.find_in_scopes("onvif://www.onvif.org/name/")
        }

        pub fn hardware(&self) -> Option<String> {
            self.find_in_scopes("onvif://www.onvif.org/hardware/")
        }

        pub fn find_in_scopes(&self, prefix: &str) -> Option<String> {
            self.scopes().iter().find_map(|url| {
                url.as_str()
                    .strip_prefix(prefix)
                    .map(|s| percent_decode_str(s).decode_utf8_lossy().to_string())
            })
        }

        fn split_string_to_urls(s: &str) -> Vec<Url> {
            s.split_whitespace()
                .filter_map(|addr| Url::parse(addr).ok())
                .collect()
        }
    }

    #[test]
    fn probe_match() {
        let ser = r#"
        <?xml version="1.0" encoding="utf-8"?>
        <wsd:ProbeMatch xmlns:wsd="http://schemas.xmlsoap.org/ws/2005/04/discovery"
                        xmlns:dn="http://www.onvif.org/ver10/network/wsdl"
                        xmlns:tds="http://www.onvif.org/ver10/device/wsdl">
            <wsd:Types>
                dn:NetworkVideoTransmitter
                tds:Device
            </wsd:Types>
            <wsd:Scopes>
                onvif://www.onvif.org/name/My%20Camera%202000
                onvif://www.onvif.org/hardware/My-HW-2000
                onvif://www.onvif.org/type/audio_encoder
                onvif://www.onvif.org/type/video_encoder
                onvif://www.onvif.org/type/ptz
                onvif://www.onvif.org/Profile/G
                onvif://www.onvif.org/Profile/Streaming
            </wsd:Scopes>
            <wsd:XAddrs>
                http://192.168.0.100:80/onvif/device_service
                http://10.0.0.200:80/onvif/device_service
            </wsd:XAddrs>
        </wsd:ProbeMatch>
        "#;

        let de: ProbeMatch = yaserde::de::from_str(ser).unwrap();

        assert_eq!(de.name(), Some("My Camera 2000".to_string()));
        assert_eq!(de.hardware(), Some("My-HW-2000".to_string()));
        assert!(de
            .find_in_scopes("onvif://www.onvif.org/type/video_encoder")
            .is_some());
        assert!(de
            .find_in_scopes("onvif://www.onvif.org/type/video_analytics")
            .is_none());
        assert_eq!(
            de.x_addrs(),
            vec![
                Url::parse("http://192.168.0.100:80/onvif/device_service").unwrap(),
                Url::parse("http://10.0.0.200:80/onvif/device_service").unwrap()
            ]
        );
    }
}
