use geo::Point;

use crate::{shared::get_intersection, SpatialtimeError};

#[cfg(not(docsrs))]
static TZ_FGB: &[u8] = include_bytes!("../../assets/timezones_ned.fgb.zst");

#[cfg(docsrs)]
static TZ_FGB: &[u8] = &[];

/// NED payload
#[derive(Clone, Debug)]
pub struct NedResponse {
    /// Actual offset from UTC. Only available in NED.
    pub offset: f64,
    /// TZID string such as `America/New_York`. Will exist in OSM, may not in NED.
    pub tzid: Option<String>,
}

/// Reusable reader for NED dataset lookups. Decompresses data once on creation.
pub struct NedReader {
    fgb_bytes: Vec<u8>,
}

impl NedReader {
    /// Creates a new NED reader.
    pub fn new() -> Result<Self, SpatialtimeError> {
        let mut fgb_bytes = Vec::new();
        zstd::stream::copy_decode(TZ_FGB, &mut fgb_bytes)?;
        Ok(Self { fgb_bytes })
    }

    /// Retrieve timezone data for a given longitude + latitude pair.
    pub fn lookup(&self, longitude: f64, latitude: f64) -> Result<NedResponse, SpatialtimeError> {
        let intersection_properties = get_intersection(&self.fgb_bytes, Point::new(longitude, latitude))?;
        let tzid = intersection_properties.get("tzid").map(|s| s.to_string());
        let offset_prop = intersection_properties
            .get("offset")
            .ok_or(SpatialtimeError::Properties("offset".to_string()))?;
        let offset: f64 = offset_prop
            .parse()
            .map_err(|e| SpatialtimeError::Properties(format!("offset conversion: {}", e)))?;

        Ok(NedResponse { offset, tzid })
    }
}

#[test]
fn ned_test() {
    let white_house = Point::new(-77.0365, 38.8977);
    let the_lodge = Point::new(149.1165, -35.3108);

    let ned_reader = NedReader::new().unwrap();
    let response = ned_reader.lookup(the_lodge.x(), the_lodge.y()).unwrap();
    println!("{:?}", response);
    assert_eq!(
        ned_reader.lookup(white_house.x(), white_house.y()).unwrap().offset,
        -5.0
    );
    assert_eq!(ned_reader.lookup(the_lodge.x(), the_lodge.y()).unwrap().offset, 10.0)
}
