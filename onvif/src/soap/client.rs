use crate::soap::{self, auth::username_token::UsernameToken};
use async_recursion::async_recursion;
use async_trait::async_trait;
use schema::transport::{Error, Transport};
use std::time::Duration;
use url::Url;

macro_rules! log {
    ($lvl:expr, $self:ident, $($arg:tt)+) => {
        log::log!($lvl, "{}: {}", $self.config.uri, format_args!($($arg)+))
    };
}

macro_rules! debug {
    ($($arg:tt)+) => {
        log!(log::Level::Debug, $($arg)+)
    }
}

macro_rules! warn {
    ($($arg:tt)+) => {
        log!(log::Level::Warn, $($arg)+)
    };
}

#[derive(Clone)]
pub struct Client {
    client: reqwest::Client,
    config: Config,
}

#[derive(Clone)]
pub struct ClientBuilder {
    config: Config,
}

impl ClientBuilder {
    pub fn new(uri: &Url) -> Self {
        Self {
            config: Config {
                uri: uri.clone(),
                credentials: None,
                auth_type: AuthType::Any,
                timeout: Duration::from_secs(5),
            },
        }
    }

    pub fn credentials(mut self, credentials: Option<Credentials>) -> Self {
        self.config.credentials = credentials;
        self
    }

    pub fn auth_type(mut self, auth_type: AuthType) -> Self {
        self.config.auth_type = auth_type;
        self
    }

    pub fn timeout(mut self, timeout: Duration) -> Self {
        self.config.timeout = timeout;
        self
    }

    pub fn build(self) -> Client {
        Client {
            client: reqwest::Client::builder()
                .danger_accept_invalid_certs(true)
                .redirect(reqwest::redirect::Policy::none())
                .timeout(self.config.timeout)
                .build()
                .unwrap(),
            config: self.config,
        }
    }
}

#[derive(Clone)]
struct Config {
    uri: Url,
    credentials: Option<Credentials>,
    auth_type: AuthType,
    timeout: Duration,
}

#[derive(Clone, Debug)]
pub enum AuthType {
    /// First try to authorize with Digest and in case of error try UsernameToken auth
    Any,
    /// Use only Digest auth
    Digest,
    /// Use only UsernameToken auth
    UsernameToken,
}

#[derive(Clone)]
pub struct Credentials {
    pub username: String,
    pub password: String,
}

#[derive(Clone, Copy, Debug)]
enum RequestAuthType {
    Digest,
    UsernameToken,
}

#[async_trait]
impl Transport for Client {
    async fn request(&self, message: &str) -> Result<String, Error> {
        let result = match self.config.auth_type {
            AuthType::Any => {
                match self.request_with_digest(message).await {
                    Ok(success) => Ok(success),
                    Err(Error::Authorization(e)) => {
                        warn!(self, "Failed to authorize with Digest auth: {}. Trying UsernameToken auth ...", e);
                        self.request_with_username_token(message).await
                    }
                    Err(e) => Err(e),
                }
            }
            AuthType::Digest => self.request_with_digest(message).await,
            AuthType::UsernameToken => self.request_with_username_token(message).await,
        };

        match &result {
            Ok(response) => debug!(self, "Request succeeded: {}", response),
            Err(e) => warn!(self, "Request failed: {:?}", e),
        }

        result
    }
}

impl Client {
    async fn request_with_digest(&self, message: &str) -> Result<String, Error> {
        self.request_recursive(message, &self.config.uri, RequestAuthType::Digest, None, 0)
            .await
    }

    async fn request_with_username_token(&self, message: &str) -> Result<String, Error> {
        self.request_recursive(
            message,
            &self.config.uri,
            RequestAuthType::UsernameToken,
            None,
            0,
        )
        .await
    }

    #[async_recursion]
    async fn request_recursive(
        &self,
        message: &str,
        uri: &Url,
        auth_type: RequestAuthType,
        prev_response: Option<&'async_recursion reqwest::Response>,
        redirections: u32,
    ) -> Result<String, Error> {
        let username_token = match auth_type {
            RequestAuthType::UsernameToken => self.username_token_auth(),
            _ => None,
        };

        debug!(
            self,
            "About to make request. auth_type={:?}, prev_response={}, redirections={}",
            auth_type,
            prev_response.is_some(),
            redirections
        );

        let soap_msg = soap::soap(message, &username_token)
            .map_err(|e| Error::Protocol(format!("{:?}", e)))?;

        let mut request = self
            .client
            .post(uri.as_str())
            .header("Content-Type", "application/soap+xml; charset=utf-8;");

        if let Some(response) = prev_response {
            let creds = self
                .config
                .credentials
                .as_ref()
                .ok_or_else(|| Error::Authorization("No credentials".to_string()))?;

            request = request.header(
                "Authorization",
                Client::digest_auth(&response, &creds, uri)?,
            );

            debug!(self, "Digest headers added");
        }

        debug!(self, "Request body: {}", soap_msg);

        let response = request
            .body(soap_msg)
            .send()
            .await
            .map_err(|e| Error::Protocol(e.to_string()))?;

        let status = response.status();

        debug!(self, "Response status: {}", status);

        if status.is_success() {
            response
                .text()
                .await
                .map_err(|e| Error::Protocol(e.to_string()))
                .and_then(|text| {
                    debug!(self, "Response body: {}", text);
                    soap::unsoap(&text).map_err(|e| Error::Protocol(format!("{:?}", e)))
                })
        } else if status == reqwest::StatusCode::UNAUTHORIZED {
            match auth_type {
                RequestAuthType::Digest => {
                    // If we got 401 then we should retry with digest HTTP auth.
                    // 401 response has all the needed data for authentication.
                    self.request_recursive(message, &uri, auth_type, Some(&response), redirections)
                        .await
                }
                _ => Err(Error::Authorization("Unauthorized".to_string())),
            }
        } else if status.is_redirection() {
            // reqwest changes method on 302, so we have to handle redirections ourselves
            // https://github.com/seanmonstar/reqwest/issues/912

            if redirections > 0 {
                return Err(Error::Redirection("Redirection limit exceeded".to_string()));
            }

            let new_url = Client::get_redirect_location(&response)?;

            debug!(self, "Redirecting to {} ...", new_url);

            self.request_recursive(message, &new_url, auth_type, None, redirections + 1)
                .await
        } else {
            if let Ok(text) = response.text().await {
                debug!(self, "Got HTTP error with body: {}", text);
            }

            Err(Error::Other(status.to_string()))
        }
    }

    fn get_redirect_location(response: &reqwest::Response) -> Result<Url, Error> {
        response.headers()[reqwest::header::LOCATION]
            .to_str()
            .map_err(|e| Error::Redirection(e.to_string()))?
            .parse::<Url>()
            .map_err(|e| Error::Redirection(e.to_string()))
    }

    pub fn username_token_auth(&self) -> Option<UsernameToken> {
        self.config
            .credentials
            .as_ref()
            .map(|c| UsernameToken::new(&c.username, &c.password))
    }

    fn digest_auth(
        res: &reqwest::Response,
        creds: &Credentials,
        url: &Url,
    ) -> Result<String, Error> {
        let www_authenticate = res
            .headers()
            .get(reqwest::header::WWW_AUTHENTICATE)
            .ok_or_else(|| Error::Authorization("No www-authenticate header".to_string()))?
            .to_str()
            .map_err(|e| Error::Authorization(e.to_string()))?;

        let mut context =
            digest_auth::AuthContext::new(&creds.username, &creds.password, url.path());

        context.method = digest_auth::HttpMethod::POST;

        Ok(digest_auth::parse(www_authenticate)
            .map_err(|e| Error::Authorization(e.to_string()))?
            .respond(&context)
            .map_err(|e| Error::Authorization(e.to_string()))?
            .to_string())
    }
}
