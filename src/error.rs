use std::error::Error;
use std::fmt;
use std::io;
use std::str::Utf8Error;

#[derive(Debug)]
/// The Error type
pub enum SSDPError {
    /// IO Error
    IO(io::Error),
    /// SSDP is not encoded properly
    Utf8Error(Utf8Error),
    /// Missing header in the SSDP Response
    MissingHeader(&'static str),
    /// Malformed search target in SSDP header
    ParseSearchTargetError(ParseSearchTargetError),
}

impl fmt::Display for SSDPError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match &self {
            SSDPError::IO(e) => write!(f, "io error: ").and(e.fmt(f)),
            SSDPError::Utf8Error(e) => e.fmt(f),
            SSDPError::MissingHeader(h) => write!(f, "missing header: {}", h),
            SSDPError::ParseSearchTargetError(e) => e.fmt(f),
        }
    }
}

impl Error for SSDPError {
    fn cause(&self) -> Option<&dyn Error> {
        match &self {
            SSDPError::IO(e) => Some(e),
            SSDPError::Utf8Error(e) => Some(e),
            _ => None,
        }
    }
}

impl From<io::Error> for SSDPError {
    fn from(e: io::Error) -> Self {
        SSDPError::IO(e)
    }
}

impl From<Utf8Error> for SSDPError {
    fn from(e: Utf8Error) -> Self {
        SSDPError::Utf8Error(e)
    }
}
impl From<ParseSearchTargetError> for SSDPError {
    fn from(e: ParseSearchTargetError) -> Self {
        SSDPError::ParseSearchTargetError(e)
    }
}

#[derive(Debug)]
/// An error returned when parsing a search target using from_str fails
pub struct ParseSearchTargetError {
    _priv: (),
}
impl ParseSearchTargetError {
    pub(crate) fn new() -> Self {
        Self { _priv: () }
    }
}
impl fmt::Display for ParseSearchTargetError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        "failed to parse search target".fmt(f)
    }
}
impl Error for ParseSearchTargetError {}
