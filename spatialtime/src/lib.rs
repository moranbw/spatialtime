//! A Rust library to lookup timezone data based on longitude and latitude.
//! Only focused on the offline environment, in which the system clock cannot be trusted at all (thus, no DST adjustments).
//! Uses the [Natural Earth](https://www.naturalearthdata.com/) (**NED**) and [OpenStreetMap](https://www.openstreetmap.org/) (**OSM**) datasets, 
//! pre-processed into [flatgeobufs](https://github.com/flatgeobuf/flatgeobuf) for indexed queries.

#![warn(missing_docs)]

use thiserror::Error;


/// Shared functionlaity and structs. used internally
mod shared;
/// Lookup using OSM dataset
pub mod osm;
/// Lookup using the NED dataset
pub mod ned;

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

/// Data that is returned
#[derive(Clone, Debug)]
pub struct SpatialtimeResponse {
    /// Actual offset from UTC. Only available in NED.
    pub offset: Option<f64>,
    /// TZID string such as `America/New_York`. Will exist in OSM, may not in NED.
    pub tzid: Option<String>,
}