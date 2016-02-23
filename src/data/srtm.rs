use core::Position;
use std::env;
use std::fs::metadata;
use std::path;
use std::process::Command;
use std::fmt;

struct Tile {
    lat: i32,
    lon: i32,
}

impl fmt::Display for Tile {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Tile::[lat: {}, lon: {}]", self.lat, self.lon)
    }
}

const DEM1_BASE_URL: &'static str = "http://viewfinderpanoramas.org/dem1/";

fn dem1_tile_filename(tile: &Tile, ext: &str) -> String {
    format!("N{:02}E{:03}.{}", tile.lat, tile.lon, ext)
}

fn dem1_tile_path(tile: &Tile, ext: &str) -> String {
    let pwd = env::var("PWD").expect("$PWD is not set");
    let filename = dem1_tile_filename(tile, ext);
    path::PathBuf::from(&pwd)
        .join("data")
        .join(filename)
        .to_str()
        .unwrap()
        .to_string()
}

fn dem1_download_tile(tile: &Tile) -> () {
    println!("Downloading tile {}", tile);
    Command::new("wget")
        .arg(format!("{}{}", DEM1_BASE_URL, dem1_tile_filename(tile, "zip")))
        .arg("-O")
        .arg(dem1_tile_path(tile, "zip"))
        .output()
        .unwrap_or_else(|e| panic!("Failed to download a tile with {}", e));
    println!("Downloaded  tile {}", tile);
}

fn dem1_extract_tile(tile: &Tile) -> () {
    println!("Extracting tile {}", tile);
    let tile_path = dem1_tile_path(tile, "zip");
    Command::new("unzip")
        .arg(&tile_path)
        .arg("-d")
        .arg(path::Path::new(&tile_path).parent().unwrap())
        .output()
        .unwrap_or_else(|e| panic!("Failed to extract a tile with {}", e));
    println!("Extracted  tile {}", tile);
}

pub fn get_tile(pos: &Position) {
    let tile = Tile {
        lat: pos.lat as i32,
        lon: pos.lon as i32,
    };

    match metadata(dem1_tile_path(&tile, "hgt")) {
        // The `hgt` file already exists, return
        Ok(_) => {
            println!("Tile {} found", tile);
            return;
        }
        // The `hgt` file doesn't exist, check for the `zip`
        Err(_) => {
            match metadata(dem1_tile_path(&tile, "zip")) {
                // The `zip` file exists, extract it
                Ok(_) => dem1_extract_tile(&tile),
                // The `zip` file doesn't exist, download and extract it
                Err(_) => {
                    dem1_download_tile(&tile);
                    dem1_extract_tile(&tile)
                }
            }
        }
    }
}
