# spatialtime
[![Crates.io Version](https://img.shields.io/crates/v/spatialtime)](https://crates.io/crates/spatialtime)
[![Documentation](https://docs.rs/spatialtime/badge.svg)](https://docs.rs/spatialtime)
[![License:MIT](https://img.shields.io/badge/license-MIT-blue.svg)](https://github.com/moranbw/spatialtime/blob/main/LICENSE)

A Rust library to lookup timezone data based on longitude and latitude. Only focused on the offline environment, in which the system clock cannot be trusted at all (thus, no DST adjustments).  Uses the [Natural Earth](https://www.naturalearthdata.com/) (**NED**) and [OpenStreetMap](https://www.openstreetmap.org/) (**OSM**) datasets, pre-processed into [flatgeobufs](https://github.com/flatgeobuf/flatgeobuf) for indexed queries.

### Inspo
The idea and some conventions are heavily influenced by [rtz](https://github.com/twitchax/rtz), which is an awesome library that is probably fine for most people. At the time of writing, however, it only worked with nightly Rust. So I have gone forward with this *much* simpler implementation for stable Rust, that fits the specific use-case I am trying to solve.
## Install
```toml
# use NED
spatialtime = { version = "0.2", features = ["ned"] }
# use OSM
spatialtime = { version = "0.2", features = ["osm"] }
```
## Usage
```rust
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

## OSM or NED?
**OSM** dataset is much larger, coming in at 17.9MB. **NED** is 890KB. **OSM** may be more "accurate" and more "up-to-date", but which one you use is likely case-by-case.

## Data Sources
- **NED**: [natural-earth-vector](https://github.com/nvkelso/natural-earth-vector)
- **OSM**: [timezone-boundary-builder](https://github.com/evansiroky/timezone-boundary-builder)