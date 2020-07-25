use crate::error::{ParseSearchTargetError, ParseURNError};
use std::{borrow::Cow, fmt};

#[derive(Debug, Eq, PartialEq, Clone)]
/// Specify what SSDP control points to search for
pub enum SearchTarget {
    /// Search for all devices and services.
    All,
    /// Search for root devices only.
    RootDevice,
    /// unique identifier for a device
    UUID(String),
    /// e.g. schemas-upnp-org:device:ZonePlayer:1
    /// or schemas-sonos-com:service:Queue:1
    URN(URN),
    /// e.g. roku:ecp
    Custom(String, String),
}
impl fmt::Display for SearchTarget {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            SearchTarget::All => write!(f, "ssdp:all"),
            SearchTarget::RootDevice => write!(f, "upnp:rootdevice"),
            SearchTarget::UUID(uuid) => write!(f, "uuid:{}", uuid),
            SearchTarget::URN(urn) => write!(f, "{}", urn),
            SearchTarget::Custom(key, value) => write!(f, "{}:{}", key, value),
        }
    }
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
            s if s.starts_with("urn") => URN::from_str(s)
                .map(SearchTarget::URN)
                .map_err(ParseSearchTargetError::URN)?,
            s => {
                let split: Vec<&str> = s.split(":").collect();
                if split.len() != 2 {
                    return Err(ParseSearchTargetError::ST);
                }
                SearchTarget::Custom(split[0].into(), split[1].into())
            }
        })
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[allow(missing_docs)]
/// Uniform Resource Name
///
/// e.g. `urn:schemas-upnp-org:service:RenderingControl:1`
pub enum URN {
    Device(Cow<'static, str>, Cow<'static, str>, u32),
    Service(Cow<'static, str>, Cow<'static, str>, u32),
}
impl fmt::Display for URN {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            URN::Device(domain, typ, version) => {
                write!(f, "urn:{}:device:{}:{}", domain, typ, version)
            }
            URN::Service(domain, typ, version) => {
                write!(f, "urn:{}:service:{}:{}", domain, typ, version)
            }
        }
    }
}

impl URN {
    /// Creates an instance of a device URN
    pub const fn device(domain: &'static str, typ: &'static str, version: u32) -> Self {
        URN::Device(Cow::Borrowed(domain), Cow::Borrowed(typ), version)
    }
    /// Creates an instance of a service URN
    pub const fn service(domain: &'static str, typ: &'static str, version: u32) -> Self {
        URN::Service(Cow::Borrowed(domain), Cow::Borrowed(typ), version)
    }

    /// Extracts the `schemas-upnp-org` part of the
    /// `urn:schemas-upnp-org:service:RenderingControl:1`
    pub fn domain_name(&self) -> &str {
        match self {
            URN::Device(domain_name, _, _) => domain_name,
            URN::Service(domain_name, _, _) => domain_name,
        }
    }

    /// Extracts the `RenderingControl` part of the
    /// `urn:schemas-upnp-org:service:RenderingControl:1`
    pub fn typ(&self) -> &str {
        match self {
            URN::Device(_, typ, _) => typ,
            URN::Service(_, typ, _) => typ,
        }
    }

    /// Extracts the `1` part of the
    /// `urn:schemas-upnp-org:service:RenderingControl:1`
    pub fn version(&self) -> u32 {
        match self {
            URN::Device(_, _, v) => *v,
            URN::Service(_, _, v) => *v,
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
    fn from_str(str: &str) -> Result<Self, Self::Err> {
        let mut iter = str.split(':');
        if iter.next() != Some("urn") {
            return Err(ParseURNError);
        }

        let domain = iter.next().ok_or(ParseURNError)?.to_string().into();
        let urn_type = &iter.next().ok_or(ParseURNError)?;
        let typ = iter.next().ok_or(ParseURNError)?.to_string().into();
        let version = iter
            .next()
            .ok_or(ParseURNError)?
            .parse::<u32>()
            .map_err(|_| ParseURNError)?;

        if iter.next() != None {
            return Err(ParseURNError);
        }

        if urn_type.eq_ignore_ascii_case("service") {
            Ok(URN::Service(domain, typ, version))
        } else if urn_type.eq_ignore_ascii_case("device") {
            Ok(URN::Device(domain, typ, version))
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
            Ok(SearchTarget::URN(URN::Device(
                "schemas-upnp-org".into(),
                "ZonePlayer".into(),
                1
            )))
        );
        assert_eq!(
            "urn:schemas-sonos-com:service:Queue:2".parse(),
            Ok(SearchTarget::URN(URN::Service(
                "schemas-sonos-com".into(),
                "Queue".into(),
                2
            )))
        );
        assert_eq!(
            "roku:ecp".parse(),
            Ok(SearchTarget::Custom("roku".into(), "ecp".into()))
        );
    }
}
