use std::io;

use thiserror::Error;

#[derive(Error, Debug)]
pub enum ApiError {
    #[error("API signals to wait (429 or 503)")]
    // if available, returns the Retry-After header value
    ApiRequestsWait(Option<u64>),
    #[error("reqwest error")]
    Reqwest(reqwest::Error),
    #[error("received unexpted response code `{0}`")]
    UnexpectedStatusCode(reqwest::StatusCode),
    #[error("deserialization error")]
    Deserialize(reqwest::Error),
    #[error("conversion error")]
    Conversion(String),
    #[error("backoff overflow")]
    BackoffOverflow(String),
    #[error("could not clone RequestBuilder")]
    RequestBuilderClone(),
}

#[derive(Error, Debug)]
pub enum DownloadError {
    #[error("URL parse error")]
    UrlParse(url::ParseError),
    #[error("IO error")]
    Io(io::Error),
    #[error("reqwest error")]
    Reqwest(reqwest::Error),
}

#[derive(Error, Debug)]
pub enum UnzipError {
    #[error("IO error")]
    Io(io::Error),
    #[error("zip error")]
    Zip(zip::result::ZipError),
}
