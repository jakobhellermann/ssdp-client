use std::error::Error;
use std::fmt;
use std::io;
use std::str::Utf8Error;

#[derive(Debug)]
pub enum SSDPError {
    IO(io::Error),
    Utf8Error(Utf8Error),
    MissingHeader(&'static str),
}

impl fmt::Display for SSDPError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match &self {
            SSDPError::IO(e) => write!(f, "io error: ").and(e.fmt(f)),
            SSDPError::Utf8Error(e) => e.fmt(f),
            SSDPError::MissingHeader(h) => write!(f, "missing header: {}", h),
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
