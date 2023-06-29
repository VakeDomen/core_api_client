use std::hash::{Hash, Hasher};
use serde::{Deserialize, Serialize};

/// Represents the geographical location of a data provider.
#[derive(Debug, Deserialize, Serialize, PartialEq, PartialOrd,  Default)]
pub struct Location {
    /// Country code of the location
    #[serde(rename = "countryCode")]
    pub country_code: Option<String>,
    
    /// Latitude of the location
    pub latitude: Option<f32>,
    
    /// Longitude of the location
    pub longitude: Option<f32>,
}


impl Hash for Location {
    fn hash<H: Hasher>(&self, state: &mut H) {
        // The `hash` method takes a mutable reference to a `Hasher`
        // and does not return anything. We will use `Hash::hash` to 
        // hash the inner values of `country_code`, `latitude`, and `longitude`.

        // `Hash::hash` for `Option<String>` is already implemented
        self.country_code.hash(state);

        // `f32` doesn't implement `Hash` directly. We will use `to_bits` 
        // to get a binary representation of the float and hash that 
        // representation. `Option::map` is used to apply `to_bits` to 
        // the value inside the `Option`, if there is one. 
        // `unwrap_or_default` is used to get the `u32` to hash, 
        // defaulting to `0` if the `Option` is `None`.
        self.latitude.map(f32::to_bits).unwrap_or_default().hash(state);
        self.longitude.map(f32::to_bits).unwrap_or_default().hash(state);
    }
}