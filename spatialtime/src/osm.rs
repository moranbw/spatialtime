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

/// Retrieve timezone data via the OSM dataset for a given longitude + latitude pair.
pub fn lookup(longitude: f64, latitude: f64) -> Result<OsmResponse, SpatialtimeError> {
    let intersection_properties = get_intersection(TZ_FGB, Point::new(longitude, latitude))?;
    let tzid: String = intersection_properties
        .get("tzid")
        .ok_or(SpatialtimeError::Properties("tzid".to_string()))?
        .to_string();

    Ok(OsmResponse { tzid })
}

#[test]
fn osm_test() {
    let white_house = Point::new(-77.0365, 38.8977);
    let the_lodge = Point::new(149.1165, -35.3108);

    assert_eq!(
        lookup(white_house.x(), white_house.y()).unwrap().tzid,
        "America/New_York"
    );
    assert_eq!(
        lookup(the_lodge.x(), the_lodge.y()).unwrap().tzid,
        "Australia/Sydney"
    )
}
