use serde::{Deserialize as Des, Serialize};
use serde::de::{self, Deserialize, Deserializer, MapAccess, Visitor};
use std::fmt;

/// Links are wrapped in an enum since they can either be simple string links or a structured piece of data with a link type.
#[derive(Debug, Serialize, PartialEq, PartialOrd, Hash)]
pub enum LinkType {
    /// Represents a raw string link.
    Raw(String),

    /// Represents a structured link.
    Structured(Link)  
}

/// Struct representing a structured link.
#[derive(Debug, Des, Serialize, PartialEq, PartialOrd, Hash, Default)]
pub struct Link {
    /// Type of the link.
    #[serde(rename = "type")]
    link_type: String,

    /// URL of the link.
    url: String,
}

impl Default for LinkType {
    fn default() -> Self {
        LinkType::Raw("".to_string())
    }
} 

impl<'de> Deserialize<'de> for LinkType {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct LinkTypeVisitor;

        impl<'de> Visitor<'de> for LinkTypeVisitor {
            type Value = LinkType;

            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                formatter.write_str("string or object")
            }

            fn visit_str<E>(self, value: &str) -> Result<LinkType, E>
            where
                E: de::Error,
            {
                Ok(LinkType::Raw(value.to_owned()))
            }

            fn visit_map<A>(self, map: A) -> Result<LinkType, A::Error>
            where
                A: MapAccess<'de>,
            {
                Ok(LinkType::Structured(Deserialize::deserialize(
                    de::value::MapAccessDeserializer::new(map),
                )?))
            }
        }

        deserializer.deserialize_any(LinkTypeVisitor)
    }
}

impl fmt::Display for LinkType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            LinkType::Raw(s) => write!(f, "Raw Link: {}", s),
            LinkType::Structured(link) => write!(f, "Structured Link: {}", link),
        }
    }
}

impl fmt::Display for Link {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Link {{ type: {}, url: {} }}", self.link_type, self.url)
    }
}