extern crate reqwest;
use self::reqwest::Url;
use crate::soap;
use async_recursion::async_recursion;
use async_trait::async_trait;
use schema::transport::{Error, Transport};

#[derive(Default, Debug, Clone)]
pub struct Client {
    uri: String,
    client: reqwest::Client,
    credentials: Option<Credentials>,
}

#[derive(Default, Debug, Clone)]
pub struct Credentials {
    pub username: String,
    pub password: String,
}

#[async_trait]
impl Transport for Client {
    async fn request(&self, message: &str) -> Result<String, Error> {
        let uri = Url::parse(&self.uri).map_err(|e| Error::Transport(e.to_string()))?;

        self.request(message, &uri, None, 0).await
    }
}

impl Client {
    pub fn new(uri: &str, credentials: Option<Credentials>) -> Self {
        Client {
            uri: uri.into(),
            client: reqwest::Client::builder()
                .danger_accept_invalid_certs(true)
                .redirect(reqwest::redirect::Policy::none())
                .build()
                .unwrap(),
            credentials,
        }
    }

    pub fn username_token(&self) -> Option<soap::auth::UsernameToken> {
        self.credentials
            .as_ref()
            .map(|c| soap::auth::UsernameToken::new(&c.username, &c.password))
    }

    #[async_recursion]
    async fn request(
        &self,
        message: &str,
        uri: &Url,
        prev_response: Option<&'async_recursion reqwest::Response>,
        redirections: u32,
    ) -> Result<String, Error> {
        let soap_msg = soap::soap(message, &self.username_token())
            .map_err(|e| Error::Transport(format!("{:?}", e)))?;

        let mut request = self
            .client
            .post(uri.as_str())
            .header("Content-Type", "application/soap+xml; charset=utf-8;");

        if let Some(response) = prev_response {
            let creds = self
                .credentials
                .as_ref()
                .ok_or_else(|| Error::Transport("No credentials".to_string()))?;

            request = request.header(
                "Authorization",
                Client::digest_auth(&response, &creds, uri)?,
            )
        }

        let response = request
            .body(soap_msg)
            .send()
            .await
            .map_err(|e| Error::Transport(e.to_string()))?;

        debug!("{} response.status() = {}", uri, response.status());

        if response.status().is_success() {
            response
                .text()
                .await
                .map_err(|e| Error::Transport(e.to_string()))
                .and_then(|text| {
                    soap::unsoap(&text).map_err(|e| Error::Transport(format!("{:?}", e)))
                })
        } else if response.status() == reqwest::StatusCode::UNAUTHORIZED {
            // If we got 401 then we should retry with digest HTTP auth.
            // 401 response has all needed data for authentication.
            self.request(message, &uri, Some(&response), redirections)
                .await
        } else if response.status().is_redirection() {
            // reqwest changes method on 302, so we have to handle redirections ourselves
            // https://github.com/seanmonstar/reqwest/issues/912

            if redirections > 0 {
                return Err(Error::Transport("Redirection limit exceeded".to_string()));
            }

            self.request(
                message,
                &Client::get_redirect_location(&response)?,
                None,
                redirections + 1,
            )
            .await
        } else {
            Err(Error::Transport(response.status().to_string()))
        }
    }

    fn get_redirect_location(response: &reqwest::Response) -> Result<Url, Error> {
        Url::parse(
            response.headers()[reqwest::header::LOCATION]
                .to_str()
                .map_err(|e| Error::Transport(e.to_string()))?,
        )
        .map_err(|e| Error::Transport(e.to_string()))
    }

    fn digest_auth(
        res: &reqwest::Response,
        creds: &Credentials,
        url: &Url,
    ) -> Result<String, Error> {
        let www_authenticate = res.headers()[reqwest::header::WWW_AUTHENTICATE]
            .to_str()
            .map_err(|e| Error::Transport(e.to_string()))?;

        let mut context =
            digest_auth::AuthContext::new(&creds.username, &creds.password, url.path());

        context.method = digest_auth::HttpMethod::POST;

        Ok(digest_auth::parse(www_authenticate)
            .map_err(|e| Error::Transport(e.to_string()))?
            .respond(&context)
            .map_err(|e| Error::Transport(e.to_string()))?
            .to_string())
    }
}
