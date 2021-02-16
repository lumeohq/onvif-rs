use crate::soap;
use async_recursion::async_recursion;
use async_trait::async_trait;
use reqwest::Url;
use schema::transport::{Error, Transport};

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
    pub fn new(uri: &str) -> Self {
        Self {
            config: Config {
                uri: uri.to_string(),
                credentials: None,
                auth_type: AuthType::Any,
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

    pub fn build(self) -> Client {
        Client {
            client: reqwest::Client::builder()
                .danger_accept_invalid_certs(true)
                .redirect(reqwest::redirect::Policy::none())
                .build()
                .unwrap(),
            config: self.config,
        }
    }
}

#[derive(Clone)]
struct Config {
    uri: String,
    credentials: Option<Credentials>,
    auth_type: AuthType,
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

#[derive(Clone, Copy)]
enum RequestAuthType {
    Digest,
    UsernameToken,
}

#[async_trait]
impl Transport for Client {
    async fn request(&self, message: &str) -> Result<String, Error> {
        let uri = Url::parse(&self.config.uri).map_err(|e| Error::Transport(e.to_string()))?;

        match self.config.auth_type {
            AuthType::Any => match self.request_with_digest(message, &uri).await {
                Ok(success) => Ok(success),
                Err(_) => self.request_with_username_token(message, &uri).await,
            },
            AuthType::Digest => self.request_with_digest(message, &uri).await,
            AuthType::UsernameToken => self.request_with_username_token(message, &uri).await,
        }
    }
}

impl Client {
    async fn request_with_digest(&self, message: &str, uri: &Url) -> Result<String, Error> {
        self.request_recursive(message, &uri, RequestAuthType::Digest, None, 0)
            .await
    }

    async fn request_with_username_token(&self, message: &str, uri: &Url) -> Result<String, Error> {
        self.request_recursive(message, &uri, RequestAuthType::UsernameToken, None, 0)
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

        let soap_msg = soap::soap(message, &username_token)
            .map_err(|e| Error::Transport(format!("{:?}", e)))?;

        let mut request = self
            .client
            .post(uri.as_str())
            .header("Content-Type", "application/soap+xml; charset=utf-8;");

        if let Some(response) = prev_response {
            let creds = self
                .config
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
        } else if response.status() == reqwest::StatusCode::UNAUTHORIZED
            && matches!(auth_type, RequestAuthType::Digest)
        {
            // If we got 401 then we should retry with digest HTTP auth.
            // 401 response has all needed data for authentication.
            self.request_recursive(message, &uri, auth_type, Some(&response), redirections)
                .await
        } else if response.status().is_redirection() {
            // reqwest changes method on 302, so we have to handle redirections ourselves
            // https://github.com/seanmonstar/reqwest/issues/912

            if redirections > 0 {
                return Err(Error::Transport("Redirection limit exceeded".to_string()));
            }

            self.request_recursive(
                message,
                &Client::get_redirect_location(&response)?,
                auth_type,
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

    pub fn username_token_auth(&self) -> Option<soap::auth::UsernameToken> {
        self.config
            .credentials
            .as_ref()
            .map(|c| soap::auth::UsernameToken::new(&c.username, &c.password))
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
