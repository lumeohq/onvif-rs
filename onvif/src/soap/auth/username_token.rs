#[derive(Default, Debug, Clone)]
pub struct UsernameToken {
    pub username: String,
    pub nonce: String,
    pub digest: String,
    pub created: String,
}

impl UsernameToken {
    pub fn new(
        username: &str,
        password: &str,
        fix_time_gap: Option<chrono::Duration>,
    ) -> UsernameToken {
        let uuid = uuid::Uuid::new_v4();
        let nonce = uuid.as_bytes();

        let mut created = chrono::Utc::now();
        if let Some(time_gap) = fix_time_gap {
            created = match created.checked_add_signed(time_gap) {
                Some(t) => t,
                None => chrono::Utc::now(),
            };
        }
        let created = created.to_rfc3339_opts(chrono::SecondsFormat::Millis, true);

        let mut concat = Vec::with_capacity(nonce.len() + created.len() + password.len());

        concat.extend_from_slice(nonce);
        concat.extend_from_slice(created.as_bytes());
        concat.extend_from_slice(password.as_bytes());

        let digest = {
            let mut hasher = sha1::Sha1::new();
            hasher.update(&concat);
            hasher.digest().bytes()
        };

        UsernameToken {
            username: username.to_string(),
            nonce: base64::encode(nonce),
            digest: base64::encode(digest),
            created,
        }
    }

    pub fn to_xml(&self) -> String {
        format!(
            r##"<?xml version="1.0" encoding="UTF-8"?>
            <wsse:Security
                    xmlns:wsse="http://docs.oasis-open.org/wss/2004/01/oasis-200401-wss-wssecurity-secext-1.0.xsd"
                    xmlns:wsu="http://docs.oasis-open.org/wss/2004/01/oasis-200401-wss-wssecurity-utility-1.0.xsd">
                <wsse:UsernameToken>
                    <wsse:Username>{}</wsse:Username>
                    <wsse:Password Type="http://docs.oasis-open.org/wss/2004/01/oasis-200401-wss-username-token-profile-1.0#PasswordDigest">{}</wsse:Password>
                    <wsse:Nonce EncodingType="http://docs.oasis-open.org/wss/2004/01/oasis-200401-wss-soap-message-security-1.0#Base64Binary">{}</wsse:Nonce>
                    <wsu:Created>{}</wsu:Created>
                </wsse:UsernameToken>
            </wsse:Security>"##,
            self.username, self.digest, self.nonce, self.created
        )
    }
}

#[test]
fn ws_username_token_example() {
    // Example from App guide (6.1.1.3 ONVIF::AuthenticatingByWS-UsernameToken)
    // https://www.onvif.org/wp-content/uploads/2016/12/ONVIF_WG-APG-Application_Programmers_Guide-1.pdf

    let nonce = base64::decode("LKqI6G/AikKCQrN0zqZFlg==").unwrap();
    let date = "2010-09-16T07:50:45Z";
    let password = "userpassword";

    let mut concat = Vec::new();

    concat.extend_from_slice(&nonce);
    concat.extend_from_slice(date.as_bytes());
    concat.extend_from_slice(password.as_bytes());

    let digest = {
        let mut hasher = sha1::Sha1::new();
        hasher.update(&concat);
        hasher.digest().bytes()
    };

    assert_eq!(
        base64::encode(digest),
        "tuOSpGlFlIXsozq4HFNeeGeFLEI=".to_string()
    )
}
