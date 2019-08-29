use crate::error::{ParseSearchTargetError, ParseURNError};
use err_derive::Error;
use std::borrow::Cow;

#[derive(Debug, Eq, PartialEq, Clone, Error)]
/// Specify what SSDP control points to search for
pub enum SearchTarget<'s> {
    /// Search for all devices and services.
    #[error(display = "ssdp:all")]
    All,
    /// Search for root devices only.
    #[error(display = "upnp:rootdevice")]
    RootDevice,
    /// Search for a particular device. device-UUID specified by UPnP vendor.
    #[error(display = "uuid:{}", _0)]
    UUID(String),
    /// e.g. schemas-upnp-org:device:ZonePlayer:1
    /// or schemas-sonos-com:service:Queue:1
    #[error(display = "{}", _0)]
    URN(URN<'s>),
}

impl std::str::FromStr for SearchTarget<'_> {
    type Err = ParseSearchTargetError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(match s {
            "ssdp:all" => SearchTarget::All,
            "upnp:rootdevice" => SearchTarget::RootDevice,
            s if s.starts_with("uuid") => {
                SearchTarget::UUID(s.trim_start_matches("uuid:").to_string())
            }
            s => URN::from_str(s)
                .map(SearchTarget::URN)
                .map_err(ParseSearchTargetError::URN)?,
        })
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Error)]
#[error(display = "urn:{}:{}:{}:{}", domain, urn_type, type_, version)]
#[allow(missing_docs)]
/// Uniform Resource Name
/// urn:$domain:$urn_type:$type_:$version
/// urn:schemas-upnp-org:service:RenderingControl:1
pub struct URN<'s> {
    pub domain: Cow<'s, str>,
    pub urn_type: URNType,
    pub type_: Cow<'s, str>,
    pub version: u32,
}

impl std::str::FromStr for URN<'_> {
    type Err = ParseURNError;
    #[rustfmt::skip]
    fn from_str(str: &str) -> Result<Self, Self::Err> {
        let mut iter = str.split(':');
        if iter.next() != Some("urn") { return Err(ParseURNError); }

        let domain = iter.next().ok_or(ParseURNError)?.to_string();
        let urn_type = iter.next().ok_or(ParseURNError)?.parse()?;
        let type_ = iter.next().ok_or(ParseURNError)?.to_string();
        let version = iter.next().ok_or(ParseURNError)?
            .parse::<u32>().map_err(|_| ParseURNError)?;

        if iter.next() != None { return Err(ParseURNError); }

        Ok(URN { domain: domain.into(), urn_type, type_: type_.into(), version })
    }
}

#[derive(Debug, Copy, Clone, Eq, PartialEq, Error)]
#[allow(missing_docs)]
pub enum URNType {
    #[error(display = "service")]
    Service,
    #[error(display = "device")]
    Device,
}

impl std::str::FromStr for URNType {
    type Err = ParseURNError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "device" => Ok(URNType::Device),
            "service" => Ok(URNType::Service),
            _ => Err(ParseURNError),
        }
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
