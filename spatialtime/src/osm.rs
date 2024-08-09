use geo::Point;

use crate::{shared::get_intersection, SpatialtimeError, SpatialtimeResponse};

/// Retrieve timezone data via the OSM dataset for a given longitude + latitude pair. 
pub fn lookup(longitude: f64, latitude: f64) -> Result<SpatialtimeResponse, SpatialtimeError> {
    let intersection_properties = get_intersection(
        include_bytes!("../../assets/timezones_osm.fgb.zst"),
        Point::new(longitude, latitude),
    )?;
    let tzid: String = intersection_properties
        .get("tzid")
        .ok_or(SpatialtimeError::Properties("tzid".to_string()))?
        .to_string();

    Ok(SpatialtimeResponse {
        offset: None,
        tzid: Some(tzid),
    })
}

#[test]
fn osm_test() {
    let white_house = Point::new(-77.0365, 38.8977);
    let the_lodge = Point::new(149.1165, -35.3108);

    assert_eq!(lookup(white_house.x(), white_house.y()).unwrap().tzid.unwrap(), "America/New_York");
    assert_eq!(lookup(the_lodge.x(), the_lodge.y()).unwrap().tzid.unwrap(), "Australia/Sydney")
}