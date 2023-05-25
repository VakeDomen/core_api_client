use serde::Deserialize as Des;
use serde::de::{self, Deserialize, Deserializer, MapAccess, Visitor};
use serde::ser::{Serialize, Serializer};
use std::fmt;


#[derive(Debug)]
pub enum LinkType {
    Raw(String),
    Structured(Link)  
} 


#[derive(Debug, Des)]
pub struct Link {
    #[serde(rename = "type")]
    link_type: String,
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