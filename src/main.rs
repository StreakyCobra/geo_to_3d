#![allow(dead_code)]

#[macro_use]
extern crate clap;

mod core;
mod dem;

use clap::{Arg, ArgMatches, App};
use core::{Location, Coordinate};
use dem::srtm::get_1dem;
use dem::viewfinderpanoramas::Dataset;

/// Parse command line arguments.
fn parse_arguments<'a>() -> ArgMatches<'a> {
    // Get arguments
    let matches = App::new("geo_to_3d")
        .version("0.1.0")
        .author("Fabien Dubosson <fabien.dubosson@gmail.com>")
        .about("A tool to create 3D models from geographic data.")
        .arg(Arg::with_name("DATASET")
                 .help("The dataset to use")
                 .possible_values(&["dem1", "dem3", "dem15"])
                 .required(true))
        .arg(Arg::with_name("LAT_1")
                 .help("Latitude of the first point of the rectangle")
                 .required(true))
        .arg(Arg::with_name("LON_1")
                 .help("Longitude of the first point of the rectangle")
                 .required(true))
        .arg(Arg::with_name("LAT_2")
                 .help("Latitude of the second point of the rectangle")
                 .required(true))
        .arg(Arg::with_name("LON_2")
                 .help("Longitude of the second point of the rectangle")
                 .required(true))
        .get_matches();

    // Check arguments types
    value_t_or_exit!(matches.value_of("LAT_1"), f32);
    value_t_or_exit!(matches.value_of("LON_1"), f32);
    value_t_or_exit!(matches.value_of("LAT_2"), f32);
    value_t_or_exit!(matches.value_of("LON_2"), f32);

    // Return arguments matches
    matches
}

/// Entry point of the program.
fn main() {
    let matches = parse_arguments();

    // Dataset to use
    #[allow(unused_variables)]
    let dataset: Dataset = match matches.value_of("DATASET").unwrap() {
        "dem1" => Dataset::Dem1,
        "dem3" => Dataset::Dem3,
        "dem15" => Dataset::Dem15,
        _ => panic!("The specified dataset is not supported."),
    };

    // Parse coordinate points as float values
    let lat_1: f32 = matches.value_of("LAT_1")
        .unwrap()
        .parse()
        .unwrap();
    let lon_1: f32 = matches.value_of("LON_1")
        .unwrap()
        .parse()
        .unwrap();
    let lat_2: f32 = matches.value_of("LAT_2")
        .unwrap()
        .parse()
        .unwrap();
    let lon_2: f32 = matches.value_of("LON_2")
        .unwrap()
        .parse()
        .unwrap();

    // Create location points from coordinates
    let loc_1 = Location {
        lat: Coordinate::new(&lat_1),
        lon: Coordinate::new(&lon_1),
    };
    let loc_2 = Location {
        lat: Coordinate::new(&lat_2),
        lon: Coordinate::new(&lon_2),
    };

    // Print coordinate points
    println!("{}", loc_1);
    println!("{}", loc_2);

    println!("{:?}", get_1dem(&loc_1, &loc_2));
}
