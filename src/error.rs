use std::{fmt, io, str::Utf8Error};

#[derive(Debug)]
/// The Error type
pub enum Error {
    /// IO Error
    IO(io::Error),
    /// SSDP is not encoded properly
    Utf8Error(Utf8Error),
    /// Missing header in the SSDP Response
    MissingHeader(&'static str),
    /// Invalid header in the SSDP Response
    InvalidHeader(&'static str),
    /// Malformed search target in SSDP header
    ParseSearchTargetError(ParseSearchTargetError),
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match &self {
            Error::IO(e) => write!(f, "io error: ").and(e.fmt(f)),
            Error::Utf8Error(e) => e.fmt(f),
            Error::MissingHeader(h) => write!(f, "missing header: {}", h),
            Error::InvalidHeader(h) => write!(f, "invalid header: {}", h),
            Error::ParseSearchTargetError(e) => e.fmt(f),
        }
    }
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

#[derive(Debug, Eq, PartialEq)]
/// An error returned when parsing a search target using `from_str` fails
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
impl std::error::Error for ParseSearchTargetError {}
