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

/// Retrieve timezone data via the NED dataset for a given longitude + latitude pair.
pub fn lookup(longitude: f64, latitude: f64) -> Result<NedResponse, SpatialtimeError> {
    let intersection_properties = get_intersection(TZ_FGB, Point::new(longitude, latitude))?;
    let tzid = {
        match intersection_properties.get("tzid") {
            Some(tzid) => Some(tzid.to_string()),
            None => None,
        }
    };
    let offset_prop = intersection_properties
        .get("offset")
        .ok_or(SpatialtimeError::Properties("offset".to_string()))?;
    let offset: f64 = offset_prop
        .parse()
        .map_err(|e| SpatialtimeError::Properties(format!("offset conversion: {}", e)))?;

    Ok(NedResponse { offset, tzid })
}

#[test]
fn ned_test() {
    let white_house = Point::new(-77.0365, 38.8977);
    let the_lodge = Point::new(149.1165, -35.3108);

    let response = lookup(the_lodge.x(), the_lodge.y()).unwrap();
    println!("{:?}", response);
    assert_eq!(
        lookup(white_house.x(), white_house.y()).unwrap().offset,
        -5.0
    );
    assert_eq!(lookup(the_lodge.x(), the_lodge.y()).unwrap().offset, 10.0)
}
