extern crate nalgebra;

use core::Location;
use std::env;
use std::fmt;
use std::fs::metadata;
use std::path;
use std::process::Command;
use self::nalgebra::DMat;

const BASE_URL: &'static str = "http://viewfinderpanoramas.org/dem1/";

struct Tile {
    pub lat: i8,
    pub lon: i8,
}

impl fmt::Display for Tile {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Tile::[lat: {}, lon: {}]", self.lat, self.lon)
    }
}

fn tile_filename(tile: &Tile, ext: &str) -> String {
    format!("N{:02}E{:03}.{}", tile.lat, tile.lon, ext)
}

fn tile_path(tile: &Tile, ext: &str) -> String {
    let pwd = env::var("PWD").expect("$PWD is not set");
    let filename = tile_filename(tile, ext);
    path::PathBuf::from(&pwd)
        .join("data")
        .join(filename)
        .to_str()
        .unwrap()
        .to_string()
}

fn download_tile(tile: &Tile) -> () {
    println!("Downloading tile {}", tile);
    Command::new("wget")
        .arg(format!("{}{}", BASE_URL, tile_filename(tile, "zip")))
        .arg("-O")
        .arg(tile_path(tile, "zip"))
        .output()
        .unwrap_or_else(|e| panic!("Failed to download a tile with {}", e));
    println!("Downloaded  tile {}", tile);
}

fn extract_tile(tile: &Tile) -> () {
    println!("Extracting tile {}", tile);
    let tile_path = tile_path(tile, "zip");
    Command::new("unzip")
        .arg(&tile_path)
        .arg("-d")
        .arg(path::Path::new(&tile_path).parent().unwrap())
        .output()
        .unwrap_or_else(|e| panic!("Failed to extract a tile with {}", e));
    println!("Extracted  tile {}", tile);
}

fn get_tile(tile: &Tile) {
    match metadata(tile_path(&tile, "hgt")) {
        // The `hgt` file already exists, return
        Ok(_) => {
            println!("Tile {} found", tile);
            return;
       }
        // The `hgt` file doesn't exist, check for the `zip`
        Err(_) => {
            match metadata(tile_path(&tile, "zip")) {
                // The `zip` file exists, extract it
                Ok(_) => extract_tile(&tile),
                // The `zip` file doesn't exist, download and extract it
                Err(_) => {
                    download_tile(&tile);
                    extract_tile(&tile)
                }
            }
        }
    }
}

fn tile_from_coord(location: &Location) -> Tile {
    Tile {
        lat: location.lat.deg,
        lon: location.lon.deg,
    }
}

pub fn get_dem(point_1: &Location, point_2: &Location) -> DMat<f32> {
    DMat::from_row_vec(2, 2, &[0.0, 0.0, 0.0, 0.0])
}
