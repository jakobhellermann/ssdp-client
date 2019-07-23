use crate::error::{ParseSearchTargetError, ParseURNError};
use std::borrow::Cow;
use std::fmt;

#[derive(Debug, Eq, PartialEq, Clone)]
/// Specify what SSDP control points to search for
pub enum SearchTarget<'s> {
    /// Search for all devices and services.
    All,
    /// Search for root devices only.
    RootDevice,
    /// Search for a particular device. device-UUID specified by UPnP vendor.
    UUID(String),
    /// e.g. schemas-upnp-org:device:ZonePlayer:1
    /// or schemas-sonos-com:service:Queue:1
    URN(URN<'s>),
}

#[derive(Clone, Debug, Eq, PartialEq)]
#[allow(missing_docs)]
/// Uniform Resource Name
pub struct URN<'s> {
    pub domain: Cow<'s, str>,
    pub urn_type: URNType,
    pub type_: Cow<'s, str>,
    pub version: u32,
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
#[allow(missing_docs)]
pub enum URNType {
    Service,
    Device,
}

impl fmt::Display for URN<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "urn:{}:{}:{}:{}",
            self.domain, self.urn_type, self.type_, self.version
        )
    }
}
impl fmt::Display for URNType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            URNType::Service => "service".fmt(f),
            URNType::Device => "device".fmt(f),
        }
    }
}
impl fmt::Display for SearchTarget<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            SearchTarget::All => "ssdp:all".fmt(f),
            SearchTarget::RootDevice => "upnp:rootdevice".fmt(f),
            SearchTarget::UUID(uuid) => write!(f, "uuid:{}", uuid),
            SearchTarget::URN(urn) => write!(f, "{}", urn),
        }
    }
}

impl std::str::FromStr for URN<'_> {
    type Err = ParseURNError;

    fn from_str(str: &str) -> Result<Self, Self::Err> {
        let mut iter = str.split(':');
        if iter.next() != Some("urn") {
            return Err(ParseURNError);
        }

        let domain = iter.next().ok_or(ParseURNError)?.to_string();
        let device_or_service = iter.next().ok_or(ParseURNError)?;
        let type_ = iter.next().ok_or(ParseURNError)?.to_string();
        let version = iter
            .next()
            .ok_or(ParseURNError)?
            .parse::<u32>()
            .map_err(|_| ParseURNError)?;

        if iter.next() != None {
            return Err(ParseURNError);
        }

        let urn_type = match device_or_service {
            "device" => Ok(URNType::Device),
            "service" => Ok(URNType::Service),
            _ => Err(ParseURNError),
        }?;

        Ok(URN {
            domain: Cow::Owned(domain),
            urn_type,
            type_: Cow::Owned(type_),
            version,
        })
    }
}

impl std::str::FromStr for SearchTarget<'_> {
    type Err = ParseSearchTargetError;

    fn from_str(str: &str) -> Result<Self, Self::Err> {
        if str == "ssdp:all" {
            return Ok(SearchTarget::All);
        }
        if str == "upnp:rootdevice" {
            return Ok(SearchTarget::RootDevice);
        }
        if str.starts_with("uuid:") {
            return Ok(SearchTarget::UUID(
                str.trim_start_matches("uuid:").to_string(),
            ));
        }

        URN::from_str(str)
            .map(SearchTarget::URN)
            .map_err(ParseSearchTargetError::URN)
    }
}

#[cfg(test)]
mod tests {
    use super::{SearchTarget, URNType, URN};

    #[test]
    fn parse_search_target() {
        assert_eq!("ssdp:all".parse(), Ok(SearchTarget::All));
        assert_eq!("upnp:rootdevice".parse(), Ok(SearchTarget::RootDevice));

        assert_eq!(
            "uuid:some-uuid".parse(),
            Ok(SearchTarget::UUID("some-uuid".to_string()))
        );

        assert_eq!(
            "urn:schemas-upnp-org:device:ZonePlayer:1".parse(),
            Ok(SearchTarget::URN(URN {
                domain: "schemas-upnp-org".into(),
                urn_type: URNType::Device,
                type_: "ZonePlayer".into(),
                version: 1,
            }))
        );
        assert_eq!(
            "urn:schemas-sonos-com:service:Queue:2".parse(),
            Ok(SearchTarget::URN(URN {
                domain: "schemas-sonos-com".into(),
                urn_type: URNType::Service,
                type_: "Queue".into(),
                version: 2
            }))
        );
    }
}
