use anyhow::{anyhow, Result};
use std::env;
use std::fs::rename;
use std::io::Write;
use std::path::PathBuf;
use std::{fs::File, io::Read};

static OSM_ASSET_ADDRESS: &str = "https://raw.githubusercontent.com/moranbw/spatialtime/main/assets/timezones_osm.fgb.zst";
static NED_ASSET_ADDRESS: &str = "https://raw.githubusercontent.com/moranbw/spatialtime/main/assets/timezones_ned.fgb.zst";

fn main() -> Result<()> {
    // if docs.rs, we don't want to build the large assets...
    if std::env::var("DOCS_RS").is_ok() {
        return Ok(());
    }

    let rebuild_assets = cfg!(feature = "rebuild-assets");
    let cwd = env::current_dir()?;
    let parent_path = cwd
        .parent()
        .ok_or_else(|| anyhow!("Could not get parent path?"))?;
    std::fs::create_dir_all(parent_path.join("assets"))?;

    if cfg!(feature = "ned") {
        let asset_path = parent_path.join("assets").join("timezones_ned.fgb.zst");
        if !asset_path.exists() || rebuild_assets {
            save_file(NED_ASSET_ADDRESS, asset_path)?;
        }
    }

    if cfg!(feature = "osm") {
        let asset_path = parent_path.join("assets").join("timezones_osm.fgb.zst");
        if !asset_path.exists() || rebuild_assets {
            save_file(OSM_ASSET_ADDRESS, asset_path)?;
        }
    }

    Ok(())
}

fn save_file(address: &str, asset_path: PathBuf) -> Result<()> {
    let mut response = reqwest::blocking::get(address)?;
    let mut output_byte_vec = Vec::new();
    response.read_to_end(&mut output_byte_vec)?;
    let tmp_path = asset_path.with_extension("tmp");
    let mut file = File::create(&tmp_path)?;
    file.write_all(&output_byte_vec.as_slice())?;
    rename(tmp_path, asset_path)?;
    Ok(())
}