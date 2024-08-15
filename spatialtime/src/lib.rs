
/*!
 A very simple library to lookup timezone data based on longitude and latitude (offline).

## Usage
```
let response = spatialtime::osm::lookup(-77.0365, 38.8977).unwrap();
/***
 *  OSM dataset does not include offset, just tzid
 *  OsmResponse { tzid: "America/New_York" }
 ***/
let response = spatialtime::ned::lookup(149.1165, -35.3108).unwrap();
/***
 *  NED dataset will always contain offset, but might not have a tzid
 *  NedResponse { offset: 10.0, tzid: Some("Australia/Sydney") }
 ***/
```
## Crate features
By default, this crate will not include any functionality.  You need to explicitly enable the 
**ned** and/or **osm** features.
* **ned** -
  Uses the [Natural Earth](https://www.naturalearthdata.com/) dataset. Much smaller at 890KB.
* **osm** -
  Uses the [OpenStreetMap](https://www.openstreetmap.org/) dataset. Much larger at 17.9MB, but 
  potentially more "accurate" and "up-to-date"

*/
 


#![warn(missing_docs)]
use thiserror::Error;

/// Lookup using the NED dataset
#[cfg(feature = "ned")]
pub mod ned;
/// Lookup using OSM dataset
#[cfg(feature = "osm")]
pub mod osm;
/// Shared functionlaity and structs. used internally
mod shared;

/// Custom errors
#[derive(Error, Debug)]
pub enum SpatialtimeError {
    /// Error decompressing Zst file
    #[error("Error decompressing: {0}")]
    Zst(#[from] std::io::Error),
    /// Error while reading flatgeobuf
    #[error("Error reading flatgeobuf: {0}")]
    Fgb(#[from] flatgeobuf::Error),
    /// Error while manipulating data with geozero
    #[error("Error parsing with geozero: {0}")]
    Geozero(#[from] geozero::error::GeozeroError),
    /// Error while retrieving properties that may have been "required"
    #[error("Error fetching properties: {0}")]
    Properties(String),
    /// No instersection found for longitude + latitude pair
    #[error("No intersection found!")]
    NoIntersection,
}
