use anyhow::Result;
use flatgeobuf::{FgbCrs, FgbWriter, FgbWriterOptions, GeometryType};
use geojson::de::{deserialize_feature_collection_to_vec, deserialize_geometry};
use geojson::ser::{serialize_geometry, to_feature_collection_byte_vec};
use geozero::{geojson::GeoJsonReader, GeozeroDatasource};
use serde::{Deserialize, Serialize};
use std::{
    fs::File,
    io::{BufWriter, Read, Write},
};

static OSM_ADDRESS: &str = "https://github.com/evansiroky/timezone-boundary-builder/releases/latest/download/timezones-with-oceans-now.geojson.zip";
static NED_ADDRESS: &str = "https://raw.githubusercontent.com/nvkelso/natural-earth-vector/master/geojson/ne_10m_time_zones.geojson";

#[derive(Deserialize, Serialize)]
struct NedGeoJson {
    #[serde(
        serialize_with = "serialize_geometry",
        deserialize_with = "deserialize_geometry"
    )]
    geometry: geo_types::Geometry<f64>,
    #[serde(rename(deserialize = "zone"))]
    offset: f64,
    #[serde(rename(deserialize = "tz_name1st"))]
    tzid: Option<String>,
}

fn main() -> Result<()> {
    let ned_bytes = get_ned_bytes()?;
    let osm_bytes = get_osm_bytes()?;

    write_fgb(ned_bytes, "timezones_ned.fgb")?;
    write_fgb(osm_bytes, "timezones_osm.fgb")?;
    Ok(())
}

fn get_ned_bytes() -> Result<Vec<u8>> {
    let mut response = reqwest::blocking::get(NED_ADDRESS)?;
    let mut input_byte_vec = Vec::new();
    response.read_to_end(&mut input_byte_vec)?;
    let geojson_struct: Vec<NedGeoJson> =
        deserialize_feature_collection_to_vec(input_byte_vec.as_slice())?;
    let output_byte_vec = to_feature_collection_byte_vec(&geojson_struct)?;
    Ok(output_byte_vec)
}

fn get_osm_bytes() -> Result<Vec<u8>> {
    let response = reqwest::blocking::get(OSM_ADDRESS)?;
    let download_bytes = response.bytes()?;
    let mut byte_vec = Vec::new();
    let mut zip = zip::ZipArchive::new(std::io::Cursor::new(download_bytes))?;
    let mut file_input = Vec::new();
    zip.by_index(0)?.read_to_end(&mut file_input)?;
    byte_vec.append(&mut file_input);
    Ok(byte_vec)
}

fn write_fgb(byte_vec: Vec<u8>, file_name: &str) -> Result<()> {
    let mut fgb: FgbWriter = FgbWriter::create_with_options(
        "timezones",
        GeometryType::MultiPolygon,
        FgbWriterOptions {
            write_index: true,
            promote_to_multi: true,
            crs: FgbCrs {
                org: Some("EPSG"),
                code: 4326,
                name: None,
                description: None,
                wkt: None,
                code_string: None,
            },
            ..Default::default()
        },
    )?;

    let mut reader = GeoJsonReader(byte_vec.as_slice());
    reader.process(&mut fgb)?;

    let mut byte_vec: Vec<u8> = Vec::new();
    fgb.write(&mut byte_vec)?;
    /*let fgb_file = File::create(format!("../assets/{}", file_name))?;
    let mut fgb_writer = BufWriter::new(fgb_file);
    fgb_writer.write_all(&byte_vec.as_slice())?;*/
    let zstd_file = File::create(format!("../assets/{}.zst", file_name))?;
    zstd::stream::copy_encode(byte_vec.as_slice(), zstd_file, 22)?;

    Ok(())
}
