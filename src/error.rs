use std::{fmt, io, str::Utf8Error};

#[derive(Debug)]
/// The Error type
pub enum Error {
    /// IO Error
    IO(io::Error),
    /// SSDP is not encoded properly
    Utf8(Utf8Error),
    /// Missing header in the SSDP Response
    MissingHeader(&'static str),
    /// Invalid header in the SSDP Response
    InvalidHeader(&'static str),
    /// Malformed search target in SSDP header
    ParseSearchTargetError(ParseSearchTargetError),
    /// Failed to parse HTTP response
    InvalidHTTP(&'static str),
    /// Non-200 HTTP Status Code
    HTTPError(u32),
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Error::IO(err) => write!(f, "io error: {err}"),
            Error::Utf8(err) => write!(f, "utf8 decoding error: {err}"),
            Error::MissingHeader(err) => write!(f, "missing header: {err}"),
            Error::InvalidHeader(err) => write!(f, "invalid header: {err}"),
            Error::ParseSearchTargetError(err) => write!(f, "{err}"),
            Error::InvalidHTTP(err) => write!(f, "failed to parse http response: {err}"),
            Error::HTTPError(err) => write!(
                f,
                "control point responded with non-zero exit code: {err}"
            ),
        }
    }
}

impl std::error::Error for Error {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Error::IO(err) => Some(err),
            Error::Utf8(err) => Some(err),
            Error::ParseSearchTargetError(err) => Some(err),
            _ => None,
        }
    }
}

impl From<io::Error> for Error {
    fn from(err: io::Error) -> Self {
        Error::IO(err)
    }
}
impl From<Utf8Error> for Error {
    fn from(err: Utf8Error) -> Self {
        Error::Utf8(err)
    }
}
impl From<ParseSearchTargetError> for Error {
    fn from(err: ParseSearchTargetError) -> Self {
        Error::ParseSearchTargetError(err)
    }
}

/// An error returned when parsing a search target using `from_str` fails
#[derive(Debug, Eq, PartialEq)]
pub struct ParseURNError;
impl std::error::Error for ParseURNError {}
impl fmt::Display for ParseURNError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "failed to parse URN")
    }
}

/// An error returned when parsing a search target using `from_str` fails
#[derive(Debug, Eq, PartialEq)]
pub enum ParseSearchTargetError {
    /// Failed to parse URN in Search Target
    #[allow(clippy::upper_case_acronyms)]
    URN(ParseURNError),
    /// Failed to parse Search Target
    ST,
}

impl fmt::Display for ParseSearchTargetError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ParseSearchTargetError::URN(_) => write!(f, "invalid urn supplied"),
            ParseSearchTargetError::ST => write!(f, "invalid search target format"),
        }
    }
}
impl std::error::Error for ParseSearchTargetError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        if let ParseSearchTargetError::URN(err) = self {
            Some(err)
        } else {
            None
        }
    }
}
