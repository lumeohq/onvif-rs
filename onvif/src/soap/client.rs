#![allow(clippy::large_enum_variant)]

use crate::soap::{
    self,
    auth::{digest::Digest, username_token::UsernameToken},
};
use async_recursion::async_recursion;
use async_trait::async_trait;
use schema::transport::{Error, Transport};
use std::{
    fmt::{Debug, Formatter},
    sync::Arc,
    time::Duration,
};
use url::Url;

macro_rules! event {
    ($lvl:expr, $self:ident, $($arg:tt)+) => {
        tracing::event!($lvl, "{}: {}", $self.config.uri, format_args!($($arg)+))
    };
}

macro_rules! debug {
    ($($arg:tt)+) => {
        event!(tracing::Level::DEBUG, $($arg)+)
    }
}

#[derive(Clone)]
pub struct Client {
    client: reqwest::Client,
    config: Config,
}

#[derive(Clone)]
pub struct ClientBuilder {
    client: Option<reqwest::Client>,
    config: Config,
}

impl ClientBuilder {
    pub const DEFAULT_TIMEOUT: Duration = Duration::from_secs(5);

    pub fn new(uri: &Url) -> Self {
        Self {
            client: None,
            config: Config {
                uri: uri.clone(),
                credentials: None,
                response_patcher: None,
                auth_type: AuthType::Any,
                timeout: ClientBuilder::DEFAULT_TIMEOUT,
                fix_time_gap: None,
            },
        }
    }

    pub fn http_client(mut self, client: reqwest::Client) -> Self {
        self.client = Some(client);
        self
    }

    pub fn credentials(mut self, credentials: Option<Credentials>) -> Self {
        self.config.credentials = credentials;
        self
    }

    pub fn response_patcher(mut self, response_patcher: Option<ResponsePatcher>) -> Self {
        self.config.response_patcher = response_patcher;
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

    pub fn fix_time_gap(mut self, time_gap: Option<chrono::Duration>) -> Self {
        self.config.fix_time_gap = time_gap;
        self
    }

    pub fn build(self) -> Client {
        let client = if let Some(client) = self.client {
            client
        } else {
            ClientBuilder::default_http_client_builder()
                .timeout(self.config.timeout)
                .build()
                .unwrap()
        };

        Client {
            client,
            config: self.config,
        }
    }

    pub fn default_http_client_builder() -> reqwest::ClientBuilder {
        #[allow(unused_mut)]
        let mut client_builder =
            reqwest::Client::builder().redirect(reqwest::redirect::Policy::none());

        #[cfg(feature = "tls")]
        {
            // hyper-rustls does not support IP hosts (like https://192.168.1.2) which are
            // very common for IP cameras. So we can use only native-tls for now.
            // https://github.com/ctz/hyper-rustls/issues/56
            client_builder = client_builder
                .use_native_tls()
                .danger_accept_invalid_certs(true);
        }

        client_builder
    }
}

#[derive(Clone)]
struct Config {
    uri: Url,
    credentials: Option<Credentials>,
    response_patcher: Option<ResponsePatcher>,
    auth_type: AuthType,
    timeout: Duration,
    fix_time_gap: Option<chrono::Duration>,
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

impl Debug for Credentials {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("{} [password hidden]", self.username))
    }
}

pub type ResponsePatcher = Arc<dyn Fn(&str) -> Result<String, String> + Send + Sync>;

#[derive(Debug)]
enum RequestAuthType {
    Digest(Digest),
    UsernameToken,
}

#[async_trait]
impl Transport for Client {
    async fn request(&self, message: &str) -> Result<String, Error> {
        match self.config.auth_type {
            AuthType::Any => {
                match self.request_with_digest(message).await {
                    Ok(success) => Ok(success),
                    Err(Error::Authorization(e)) => {
                        debug!(self, "Failed to authorize with Digest auth: {}. Trying UsernameToken auth ...", e);
                        self.request_with_username_token(message).await
                    }
                    Err(e) => Err(e),
                }
            }
            AuthType::Digest => self.request_with_digest(message).await,
            AuthType::UsernameToken => self.request_with_username_token(message).await,
        }
    }
}

impl Client {
    async fn request_with_digest(&self, message: &str) -> Result<String, Error> {
        let mut auth_type =
            RequestAuthType::Digest(Digest::new(&self.config.uri, &self.config.credentials));

        self.request_recursive(message, &self.config.uri, &mut auth_type, 0)
            .await
    }

    async fn request_with_username_token(&self, message: &str) -> Result<String, Error> {
        let mut auth_type = RequestAuthType::UsernameToken;

        self.request_recursive(message, &self.config.uri, &mut auth_type, 0)
            .await
    }

    #[async_recursion]
    async fn request_recursive(
        &self,
        message: &str,
        uri: &Url,
        auth_type: &mut RequestAuthType,
        redirections: u32,
    ) -> Result<String, Error> {
        let username_token = match auth_type {
            RequestAuthType::UsernameToken => self.username_token_auth(),
            _ => None,
        };

        debug!(
            self,
            "About to make request. auth_type={:?}, redirections={}", auth_type, redirections
        );

        let soap_msg = soap::soap(message, &username_token)
            .map_err(|e| Error::Protocol(format!("{:?}", e)))?;

        let mut request = self
            .client
            .post(uri.as_str())
            .header("Content-Type", "application/soap+xml; charset=utf-8;");

        if let RequestAuthType::Digest(digest) = auth_type {
            request = digest
                .add_headers(request)
                .map_err(|e| Error::Authorization(e.to_string()))?;

            debug!(self, "Digest headers added");
        }

        debug!(self, "Request body: {}", soap_msg);

        let response = request.body(soap_msg).send().await.map_err(|e| match e {
            e if e.is_connect() => Error::Connection(e.to_string()),
            e if e.is_timeout() => Error::Timeout(e.to_string()),
            e if e.is_redirect() => Error::Redirection(e.to_string()),
            e if e.is_decode() || e.is_body() => Error::Protocol(e.to_string()),
            e => Error::Other(e.to_string()),
        })?;

        let status = response.status();

        debug!(self, "Response status: {}", status);

        if status.is_success() {
            response
                .text()
                .await
                .map_err(|e| Error::Protocol(e.to_string()))
                .and_then(|text| {
                    debug!(self, "Response body: {}", text);
                    let response =
                        soap::unsoap(&text).map_err(|e| Error::Protocol(format!("{:?}", e)))?;
                    if let Some(response_patcher) = &self.config.response_patcher {
                        match response_patcher(&response) {
                            Ok(patched) => {
                                debug!(self, "Response (SOAP unwrapped, patched): {}", patched);
                                Ok(patched)
                            }
                            Err(e) => Err(Error::Protocol(format!("Patching failed: {}", e))),
                        }
                    } else {
                        Ok(response)
                    }
                })
        } else if status == reqwest::StatusCode::UNAUTHORIZED {
            match auth_type {
                RequestAuthType::Digest(digest) if !digest.is_failed() => {
                    digest.set_401(response);
                }
                _ => {
                    if let Ok(text) = response.text().await {
                        debug!(self, "Got Unauthorized with body: {}", text);
                    }

                    return Err(Error::Authorization("Unauthorized".to_string()));
                }
            }

            self.request_recursive(message, uri, auth_type, redirections)
                .await
        } else if status.is_redirection() {
            // reqwest changes method on 302, so we have to handle redirections ourselves
            // https://github.com/seanmonstar/reqwest/issues/912

            if redirections > 0 {
                return Err(Error::Redirection("Redirection limit exceeded".to_string()));
            }

            let new_url = Client::get_redirect_location(&response)?;

            debug!(self, "Redirecting to {} ...", new_url);

            self.request_recursive(message, &new_url, auth_type, redirections + 1)
                .await
        } else {
            if let Ok(text) = response.text().await {
                debug!(self, "Got HTTP error with body: {}", text);
                if let Err(soap::Error::Fault(f)) = soap::unsoap(&text) {
                    if f.is_unauthorized() {
                        return Err(Error::Authorization("Unauthorized".to_string()));
                    }
                }
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
            .map(|c| UsernameToken::new(&c.username, &c.password, self.config.fix_time_gap))
    }

    pub fn set_fix_time_gap(&mut self, time_gap: Option<chrono::Duration>) {
        self.config.fix_time_gap = time_gap;
    }
}
