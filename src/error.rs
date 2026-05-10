use std::fmt;

#[derive(Debug)]
pub struct ApiError {
    pub message: String,
    pub status: u16,
    pub code: Option<String>,
}

impl fmt::Display for ApiError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match &self.code {
            Some(code) => write!(f, "laddro: {} (status {}, code {})", self.message, self.status, code),
            None => write!(f, "laddro: {} (status {})", self.message, self.status),
        }
    }
}

impl std::error::Error for ApiError {}

impl ApiError {
    pub fn is_auth_error(&self) -> bool {
        self.status == 401
    }

    pub fn is_usage_limit_error(&self) -> bool {
        self.status == 402
    }

    pub fn is_not_found(&self) -> bool {
        self.status == 404
    }
}

#[derive(Debug)]
pub enum Error {
    Api(ApiError),
    Request(reqwest::Error),
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Error::Api(e) => write!(f, "{}", e),
            Error::Request(e) => write!(f, "request failed: {}", e),
        }
    }
}

impl std::error::Error for Error {}

impl From<reqwest::Error> for Error {
    fn from(err: reqwest::Error) -> Self {
        Error::Request(err)
    }
}
