use crate::error::{ParseSearchTargetError, ParseURNError};
use err_derive::Error;
use std::borrow::Cow;

#[derive(Debug, Eq, PartialEq, Clone, Error)]
/// Specify what SSDP control points to search for
pub enum SearchTarget {
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
    URN(URN),
}

impl std::str::FromStr for SearchTarget {
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
#[allow(missing_docs)]
/// Uniform Resource Name
/// urn:$domain:$urn_type:$type_:$version
/// urn:schemas-upnp-org:service:RenderingControl:1
pub enum URN {
    #[error(display = "urn:{}:device:{}:{}", domain, typ, version)]
    Device { domain: Cow<'static, str>, typ: Cow<'static, str>, version: u32 },
    #[error(display = "urn:{}:service:{}:{}", domain, typ, version)]
    Service{ domain: Cow<'static, str>, typ: Cow<'static, str>, version: u32 },
}

impl URN {
    /// Creates an instance of a service URN
    pub const fn service(domain: &'static str, typ: &'static str, version: u32) -> Self {
        URN::Service {
            domain: Cow::Borrowed(domain),
            typ: Cow::Borrowed(typ),
            version,
        }
    }

    /// Creates an instance of a device URN
    pub const fn device(domain: &'static str, typ: &'static str, version: u32) -> Self {
        URN::Device {
            domain: Cow::Borrowed(domain),
            typ: Cow::Borrowed(typ),
            version,
        }
    }
}

impl Into<SearchTarget> for URN {
    fn into(self) -> SearchTarget {
        SearchTarget::URN(self)
    }
}

impl std::str::FromStr for URN {
    type Err = ParseURNError;
    #[rustfmt::skip]
    fn from_str(str: &str) -> Result<Self, Self::Err> {
        let mut iter = str.split(':');
        if iter.next() != Some("urn") { return Err(ParseURNError); }

        let domain = iter.next().ok_or(ParseURNError)?.to_string().into();
        let urn_type = &iter.next().ok_or(ParseURNError)?;
        let typ = iter.next().ok_or(ParseURNError)?.to_string().into();
        let version = iter.next().ok_or(ParseURNError)?
            .parse::<u32>().map_err(|_| ParseURNError)?;

        if iter.next() != None { return Err(ParseURNError); }


        if urn_type.eq_ignore_ascii_case("service") {
            Ok(URN::Service { domain , typ, version })
        } else if urn_type.eq_ignore_ascii_case("device") {
            Ok(URN::Device { domain , typ, version })
        } else {
            Err(ParseURNError)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{SearchTarget, URN};

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
            Ok(SearchTarget::URN(URN::Device {
                domain: "schemas-upnp-org".into(),
                typ: "ZonePlayer".into(),
                version: 1,
            }))
        );
        assert_eq!(
            "urn:schemas-sonos-com:service:Queue:2".parse(),
            Ok(SearchTarget::URN(URN::Service {
                domain: "schemas-sonos-com".into(),
                typ: "Queue".into(),
                version: 2
            }))
        );
    }
}
