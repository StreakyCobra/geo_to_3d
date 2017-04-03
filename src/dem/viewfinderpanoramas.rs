use std::fmt;

use core::{Coordinate, Location};

#[derive(Clone, Debug)]
struct Tile {
    pub dataset: Dataset,
    pub lat: i8,
    pub lon: i8,
}

impl fmt::Display for Tile {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Tile::[lat: {}, lon: {}]", self.lat, self.lon)
    }
}

fn tile_filename(tile: &Tile, ext: &str) -> String {
    match tile.dataset {
        Dataset::Dem1 => {
            format!("{}{:02}{}{:03}.{}",
                    if tile.lat > 0 { "N" } else { "S" },
                    tile.lat,
                    if tile.lon > 0 { "E" } else { "W" },
                    tile.lon,
                    ext)
        }
        Dataset::Dem3 => "".to_string(),
        Dataset::Dem15 => "".to_string(),
    }
}

fn tile_from_location(location: &Location, dataset: Dataset) -> Tile {
    Tile {
        dataset: dataset,
        lat: location.lat.deg,
        lon: location.lon.deg,
    }
}

#[derive(Clone, Debug)]
pub enum Dataset {
    Dem1,
    Dem3,
    Dem15,
}

fn dataset_url(dataset: &Dataset) -> String {
    let base_url = "http://viewfinderpanoramas.org/";
    let end_url = match *dataset {
        Dataset::Dem1 => "dem1/",
        Dataset::Dem3 => "dem3/",
        Dataset::Dem15 => "DEM/TIF15/",
    };
    format!("{}{}", base_url, end_url)
}

fn dataset_resolution(dataset: &Dataset) -> Coordinate {
    match *dataset {
        Dataset::Dem1 => {
            Coordinate {
                deg: 0,
                min: 0,
                sec: 1.,
            }
        }
        Dataset::Dem3 => {
            Coordinate {
                deg: 0,
                min: 0,
                sec: 3.,
            }
        }
        Dataset::Dem15 => {
            Coordinate {
                deg: 0,
                min: 0,
                sec: 15.,
            }
        }
    }
}

/// The list of tiles available in 1" format, with their optional archive name.
const DEM1_TILES: [((i8, i8), Option<String>); 39] = [// Central Spain
                                                      ((40, -6), None),
                                                      ((40, -5), None),
                                                      ((40, -4), None),
                                                      // Alps
                                                      ((43, 5), None),
                                                      ((43, 6), None),
                                                      ((43, 7), None),
                                                      ((44, 5), None),
                                                      ((44, 6), None),
                                                      ((44, 7), None),
                                                      ((45, 5), None),
                                                      ((45, 6), None),
                                                      ((45, 7), None),
                                                      ((45, 8), None),
                                                      ((45, 9), None),
                                                      ((45, 10), None),
                                                      ((45, 11), None),
                                                      ((46, 5), None),
                                                      ((46, 6), None),
                                                      ((46, 7), None),
                                                      ((46, 8), None),
                                                      ((45, 9), None),
                                                      ((46, 10), None),
                                                      ((46, 11), None),
                                                      ((45, 12), None),
                                                      ((46, 13), None),
                                                      ((46, 14), None),
                                                      ((46, 15), None),
                                                      ((47, 6), None),
                                                      ((47, 7), None),
                                                      ((47, 8), None),
                                                      ((46, 9), None),
                                                      ((47, 10), None),
                                                      ((47, 11), None),
                                                      ((46, 12), None),
                                                      ((47, 13), None),
                                                      ((47, 14), None),
                                                      ((47, 15), None),
                                                      // Slovakia/Poland
                                                      ((49, 19), None),
                                                      ((49, 20), None)];
