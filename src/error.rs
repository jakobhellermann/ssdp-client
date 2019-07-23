use display_attr::DisplayAttr;
use std::{io, str::Utf8Error};

#[derive(Debug, DisplayAttr)]
/// The Error type
pub enum Error {
    /// IO Error
    #[display(fmt = "io error: {}", _0)]
    IO(io::Error),
    /// SSDP is not encoded properly
    #[display(fmt = "utf8 error: {}", _0)]
    Utf8Error(Utf8Error),
    /// Missing header in the SSDP Response
    #[display(fmt = "missing header: {}", _0)]
    MissingHeader(&'static str),
    /// Invalid header in the SSDP Response
    #[display(fmt = "invalid header: {}", _0)]
    InvalidHeader(&'static str),
    /// Malformed search target in SSDP header
    #[display(fmt = "{}", _0)]
    ParseSearchTargetError(ParseSearchTargetError),
}

impl std::error::Error for Error {
    fn cause(&self) -> Option<&dyn std::error::Error> {
        match &self {
            Error::IO(e) => Some(e),
            Error::Utf8Error(e) => Some(e),
            _ => None,
        }
    }
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
#[derive(Debug, Eq, PartialEq, DisplayAttr)]
#[display(fmt = "failed to parse urn")]
pub struct ParseURNError;
impl std::error::Error for ParseURNError {}

/// An error returned when parsing a search target using `from_str` fails
#[derive(Debug, Eq, PartialEq, DisplayAttr)]
pub enum ParseSearchTargetError {
    /// Failed to parse URN in Search Target
    #[display(fmt = "{}", _0)]
    URN(ParseURNError),
    /// Failed to parse Search Target
    #[display(fmt = "failed to parse search target")]
    ST,
}
impl std::error::Error for ParseSearchTargetError {}
