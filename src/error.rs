use err_derive::Error;
use std::{io, str::Utf8Error};

#[derive(Debug, Error)]
/// The Error type
pub enum Error {
    /// IO Error
    #[error(display = "io error: {}", _0)]
    IO(#[error(cause)] io::Error),
    /// SSDP is not encoded properly
    #[error(display = "utf8 error: {}", _0)]
    Utf8Error(#[error(cause)] Utf8Error),
    /// Missing header in the SSDP Response
    #[error(display = "missing header: {}", _0)]
    MissingHeader(&'static str),
    /// Invalid header in the SSDP Response
    #[error(display = "invalid header: {}", _0)]
    InvalidHeader(&'static str),
    /// Malformed search target in SSDP header
    #[error(display = "{}", _0)]
    ParseSearchTargetError(#[error(cause)] ParseSearchTargetError),
    #[error(display = "failed to parse http response")]
    /// Failed to parse HTTP response
    ParseHTTPError,
    #[error(display = "control point responded with '{}' exit code", _0)]
    /// Non-200 HTTP Status Code
    HTTPError(u32),
}

impl From<io::Error> for Error {
    fn from(e: io::Error) -> Self {
        Error::IO(e)
    }
}

impl From<Utf8Error> for Error {
    fn from(e: Utf8Error) -> Self {
        Error::Utf8Error(e)
    }
}
impl From<ParseSearchTargetError> for Error {
    fn from(e: ParseSearchTargetError) -> Self {
        Error::ParseSearchTargetError(e)
    }
}

/// An error returned when parsing a search target using `from_str` fails
#[derive(Debug, Eq, PartialEq, Error)]
#[error(display = "failed to parse urn")]
pub struct ParseURNError;

/// An error returned when parsing a search target using `from_str` fails
#[derive(Debug, Eq, PartialEq, Error)]
pub enum ParseSearchTargetError {
    /// Failed to parse URN in Search Target
    #[error(display = "{}", _0)]
    URN(#[error(cause)] ParseURNError),
    /// Failed to parse Search Target
    #[error(display = "failed to parse search target")]
    ST,
}
