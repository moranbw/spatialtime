use std::{collections::HashMap, io::BufReader};

use flatgeobuf::{FallibleStreamingIterator, FeatureProperties, FgbReader};
use geo::{Geometry, Intersects, Point};
use geozero::ToGeo;

use crate::SpatialtimeError;

/// Decompress data, read as flatgeobuf, then determine if it intersects with input point. If so, return properties.
pub fn get_intersection(
    bytes: &[u8],
    point: Point,
) -> Result<HashMap<String, String>, SpatialtimeError> {
    let mut fgb_bytes = Vec::new();
    zstd::stream::copy_decode(bytes, &mut fgb_bytes).map_err(|e| SpatialtimeError::Zst(e))?;
    let mut reader = BufReader::new(fgb_bytes.as_slice());
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
