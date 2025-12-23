use std::{collections::HashMap, io::BufReader};

use flatgeobuf::{FallibleStreamingIterator, FeatureProperties, FgbReader};
use geo::{Geometry, Intersects, Point};
use geozero::ToGeo;

use crate::SpatialtimeError;

/// Takes in uncompressed flatgeobuf, creates reader, then determine if it intersects with input point. If so, return properties.
pub fn get_intersection(
    bytes: &[u8],
    point: Point,
) -> Result<HashMap<String, String>, SpatialtimeError> {
    let mut reader = BufReader::new(bytes);
    let fgb = FgbReader::open(&mut reader).map_err(|e| SpatialtimeError::Fgb(e))?;
    let mut fgp_seq = fgb
        .select_bbox_seq(point.x(), point.y(), point.x(), point.y())
        .map_err(|e| SpatialtimeError::Fgb(e))?;

    let mut props = None;
    while let Some(feature) = fgp_seq.next().map_err(|e| SpatialtimeError::Fgb(e))? {
        if let Ok(Geometry::MultiPolygon(multi_polygon)) = feature.to_geo() {
            if multi_polygon.intersects(&point) {
                props = Some(
                    feature
                        .properties()
                        .map_err(|e| SpatialtimeError::Geozero(e))?,
                );
                break;
            }
        }
    }

    props.ok_or(SpatialtimeError::NoIntersection)
}
