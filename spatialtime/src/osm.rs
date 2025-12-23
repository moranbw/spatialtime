use geo::Point;

use crate::{shared::get_intersection, SpatialtimeError};

#[cfg(not(docsrs))]
static TZ_FGB: &[u8] = include_bytes!("../../assets/timezones_osm.fgb.zst");

#[cfg(docsrs)]
static TZ_FGB: &[u8] = &[];

/// OSM payload
#[derive(Clone, Debug)]
pub struct OsmResponse {
    /// TZID string such as `America/New_York`. Will exist in OSM, may not in NED.
    pub tzid: String,
}

/// Reusable reader for OSM dataset lookups. Decompresses data once on creation.
pub struct OsmReader {
    fgb_bytes: Vec<u8>,
}

impl OsmReader {
    /// Creates a new OSM reader.
    pub fn new() -> Result<Self, SpatialtimeError> {
        let mut fgb_bytes = Vec::new();
        zstd::stream::copy_decode(TZ_FGB, &mut fgb_bytes)?;
        Ok(Self { fgb_bytes })
    }

    /// Retrieve timezone data for a given longitude + latitude pair.
    pub fn lookup(&self, longitude: f64, latitude: f64) -> Result<OsmResponse, SpatialtimeError> {
        let intersection_properties =
            get_intersection(&self.fgb_bytes, Point::new(longitude, latitude))?;
        let tzid: String = intersection_properties
            .get("tzid")
            .ok_or(SpatialtimeError::Properties("tzid".to_string()))?
            .to_string();

        Ok(OsmResponse { tzid })
    }
}

#[test]
fn osm_test() {
    let white_house = Point::new(-77.0365, 38.8977);
    let the_lodge = Point::new(149.1165, -35.3108);

    let osm_reader = OsmReader::new().unwrap();
    assert_eq!(
        osm_reader.lookup(white_house.x(), white_house.y()).unwrap().tzid,
        "America/New_York"
    );
    assert_eq!(
        osm_reader.lookup(the_lodge.x(), the_lodge.y()).unwrap().tzid,
        "Australia/Sydney"
    )
}
