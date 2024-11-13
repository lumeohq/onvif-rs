use crate::soap::client::Credentials;
use nonzero_ext::nonzero;
use reqwest::{RequestBuilder, Response};
use std::fmt::{Debug, Formatter};
use std::num::NonZeroU8;
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
    state: State,
    reuse_headers: bool,
}

enum State {
    Default,
    Got401 {
        response: Response,
        count: NonZeroU8,
    },
}

impl Digest {
    pub fn new(uri: &Url, creds: &Option<Credentials>, reuse_headers: bool) -> Self {
        Self {
            creds: creds.clone(),
            uri: uri.clone(),
            state: State::Default,
            reuse_headers,
        }
    }
}

impl Digest {
    /// Call this when the authentication was successful.
    pub fn set_success(&mut self) {
        if !self.reuse_headers {
            // Since we don't need to preserve the headers, reset all the state to default.
            *self = Self::new(&self.uri, &self.creds, self.reuse_headers);
            return;
        }

        if let State::Got401 { count, .. } = &mut self.state {
            // We always store at least one request, so it's never zero.
            *count = nonzero!(1_u8);
        }
    }

    /// Call this when received 401 Unauthorized.
    pub fn set_401(&mut self, response: Response) {
        self.state = match self.state {
            State::Default => State::Got401 {
                response,
                count: nonzero!(1_u8),
            },
            State::Got401 { count, .. } => State::Got401 {
                response,
                count: count.saturating_add(1),
            },
        }
    }

    pub fn is_failed(&self) -> bool {
        match &self.state {
            State::Default => false,
            // Possible scenarios:
            // - We've got 401 with a challenge for the first time, we calculate the answer, then
            //   we get 200 OK. So, a single 401 is never a failure.
            // - After successful auth the count is 1 because we always store at least one request,
            //   and the caller decided to reuse the same challenge for multiple requests. But at
            //   some point, we'll get a 401 with a new challenge and `stale=true`.
            //   So, we'll get a second 401, and this is also not a failure because after
            //   calculating the answer to the challenge, we'll get a 200 OK, and will reset the
            //   counter in `set_success()`.
            // - Three 401's in a row is certainly a failure.
            State::Got401 { count, .. } => count.get() >= 3,
        }
    }

    pub fn add_headers(&self, mut request: RequestBuilder) -> Result<RequestBuilder, Error> {
        match &self.state {
            State::Default => Ok(request),
            State::Got401 { response, .. } => {
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
            .field("state", &self.state)
            .finish()
    }
}

impl Debug for State {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            State::Default => write!(f, "FirstRequest")?,
            State::Got401 { count, .. } => write!(f, "Got401({count})")?,
        };

        Ok(())
    }
}
