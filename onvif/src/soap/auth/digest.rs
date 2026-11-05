use crate::soap::client::Credentials;
use reqwest::{RequestBuilder, Response};
use std::fmt::{Debug, Formatter};
use thiserror::Error;
use url::Url;

#[derive(Debug, Error)]
pub enum Error {
    #[error("Invalid state")]
    InvalidState,
    #[error("No credentials")]
    NoCredentials,
    #[error("Digest {0}")]
    Digest(String),
}

pub struct Digest {
    creds: Option<Credentials>,
    uri: Url,
    reuse_headers: bool,
    /// The last 401 response, whose challenge the next request answers.
    challenge: Option<Response>,
    /// 401 responses to requests that answered a challenge, since the last success.
    rejections: u8,
}

impl Digest {
    pub fn new(uri: &Url, creds: &Option<Credentials>, reuse_headers: bool) -> Self {
        Self {
            creds: creds.clone(),
            uri: uri.clone(),
            reuse_headers,
            challenge: None,
            rejections: 0,
        }
    }
}

impl Digest {
    /// Call this when the authentication was successful.
    pub fn set_success(&mut self) {
        self.rejections = 0;

        if !self.reuse_headers {
            self.challenge = None;
        }
    }

    /// Call this when a response other than success or 401 answered the challenge. The challenge
    /// is spent, but the credentials were not accepted either, so the rejections stay.
    pub fn discard_challenge(&mut self) {
        self.challenge = None;
    }

    /// Call this when received 401 Unauthorized.
    pub fn set_401(&mut self, response: Response) {
        if self.challenge.is_some() {
            self.rejections = self.rejections.saturating_add(1);
        }

        self.challenge = Some(response);
    }

    pub fn is_failed(&self) -> bool {
        // Possible scenarios:
        // - A request without credentials gets a 401 with a challenge, we answer it and get 200 OK.
        //   So, a 401 to a request without credentials is never a failure.
        // - The caller decided to reuse the same challenge for multiple requests. At some point,
        //   the answer gets a 401 with a new challenge and `stale=true`. This rejection is not a
        //   failure either: the answer to the new challenge gets 200 OK, and `set_success()`
        //   clears the rejections.
        // - A third rejected answer since the last success is certainly a failure.
        self.challenge.is_some() && self.rejections >= 2
    }

    pub fn add_headers(&self, mut request: RequestBuilder) -> Result<RequestBuilder, Error> {
        match &self.challenge {
            None => Ok(request),
            Some(response) => {
                let creds = self.creds.as_ref().ok_or(Error::NoCredentials)?;

                request = request.header("Authorization", digest_auth(response, creds, &self.uri)?);

                Ok(request)
            }
        }
    }
}

fn digest_auth(res: &reqwest::Response, creds: &Credentials, url: &Url) -> Result<String, Error> {
    let www_authenticate = res
        .headers()
        .get(reqwest::header::WWW_AUTHENTICATE)
        .ok_or_else(|| Error::Digest("No www-authenticate header".to_string()))?
        .to_str()
        .map_err(|e| Error::Digest(e.to_string()))?;

    let mut context = digest_auth::AuthContext::new(&creds.username, &creds.password, url.path());

    context.method = digest_auth::HttpMethod::POST;

    Ok(digest_auth::parse(www_authenticate)
        .map_err(|e| Error::Digest(e.to_string()))?
        .respond(&context)
        .map_err(|e| Error::Digest(e.to_string()))?
        .to_string())
}

impl Debug for Digest {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Digest")
            .field("creds", &self.creds)
            .field("challenge", &self.challenge.is_some())
            .field("rejections", &self.rejections)
            .finish()
    }
}
