use anyhow::{anyhow, Result};
use flatgeobuf::{FgbCrs, FgbWriter, FgbWriterOptions, GeometryType};
use geojson::de::{deserialize_feature_collection_to_vec, deserialize_geometry};
use geojson::ser::{serialize_geometry, to_feature_collection_byte_vec};
use geozero::{geojson::GeoJsonReader, GeozeroDatasource};
use serde::{Deserialize, Serialize};
use std::env;
use std::fs::rename;
use std::path::PathBuf;
use std::{
    fs::File,
    io::Read,
};

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

static OSM_ADDRESS: &str = "https://github.com/evansiroky/timezone-boundary-builder/releases/latest/download/timezones-with-oceans-now.geojson.zip";
static NED_ADDRESS: &str = "https://raw.githubusercontent.com/nvkelso/natural-earth-vector/master/geojson/ne_10m_time_zones.geojson";

fn main() -> Result<()> {
    if cfg!(not(docsrs)) {
        let rebuild_assets = cfg!(feature = "rebuild-assets");
        let cwd = env::current_dir()?;
        let parent_path = cwd
            .parent()
            .ok_or_else(|| anyhow!("Could not get parent path?"))?;
        std::fs::create_dir_all(parent_path.join("assets"))?;

        if cfg!(feature = "ned") {
            let asset_path = parent_path.join("assets").join("timezones_ned.fgb.zst");
            if !asset_path.exists() || rebuild_assets {
                let ned_bytes = get_ned_bytes()?;
                write_fgb(ned_bytes, asset_path)?;
            }
        }

        if cfg!(feature = "osm") {
            let asset_path = parent_path.join("assets").join("timezones_osm.fgb.zst");
            if !asset_path.exists() || rebuild_assets {
                let osm_bytes = get_osm_bytes()?;
                write_fgb(osm_bytes, asset_path)?;
            }
        }
    }

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

fn write_fgb(byte_vec: Vec<u8>, asset_path: PathBuf) -> Result<()> {
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
    let tmp_path = asset_path.with_extension("tmp");
    let zstd_file = File::create(&tmp_path)?;
    zstd::stream::copy_encode(byte_vec.as_slice(), zstd_file, 22)?;
    rename(tmp_path, asset_path)?;
    
    Ok(())
}
