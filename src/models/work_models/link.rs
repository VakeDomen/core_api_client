use serde::{Deserialize as Des, Serialize};
use serde::de::{self, Deserialize, Deserializer, MapAccess, Visitor};
use std::fmt;

/// Links are wrapped in an enum since they can either be simple string links or a structured piece of data with a link type.
#[derive(Debug, Serialize)]
pub enum LinkType {
    /// Represents a raw string link.
    Raw(String),

    /// Represents a structured link.
    Structured(Link)  
} 

/// Struct representing a structured link.
#[derive(Debug, Des, Serialize)]
pub struct Link {
    /// Type of the link.
    #[serde(rename = "type")]
    link_type: String,

    /// URL of the link.
    url: String,
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