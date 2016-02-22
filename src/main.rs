#[macro_use]
extern crate clap;

mod core;

use clap::{Arg, ArgMatches, App};
use core::{Point, Position};

fn parse_arguments<'a>() -> ArgMatches<'a> {
    // Get arguments matches
    let matches = App::new("geo_to_3d")
                      .version("0.1.0")
                      .author("Fabien Dubosson <fabien.dubosson@gmail.com>")
                      .about("A tool to create 3D models from geographic data.")
                      .arg(Arg::with_name("LAT_1")
                               .help("Latitude of the first point of the rectangle")
                               .required(true))
                      .arg(Arg::with_name("LGT_1")
                               .help("Longitude of the first point of the rectangle")
                               .required(true))
                      .arg(Arg::with_name("LAT_2")
                               .help("Latitude of the second point of the rectangle")
                               .required(true))
                      .arg(Arg::with_name("LGT_2")
                               .help("Longitude of the second point of the rectangle")
                               .required(true))
                      .get_matches();

    // Check arguments types
    value_t_or_exit!(matches.value_of("LAT_1"), f32);
    value_t_or_exit!(matches.value_of("LGT_1"), f32);
    value_t_or_exit!(matches.value_of("LAT_2"), f32);
    value_t_or_exit!(matches.value_of("LGT_2"), f32);

    // Return arguments matches
    matches
}

fn main() {
    let matches = parse_arguments();

    let point_1 = Position(Point {
        x: matches.value_of("LAT_1").unwrap().parse().unwrap(),
        y: matches.value_of("LGT_1").unwrap().parse().unwrap(),
    });

    let point_2 = Position(Point {
        x: matches.value_of("LAT_2").unwrap().parse().unwrap(),
        y: matches.value_of("LGT_2").unwrap().parse().unwrap(),
    });

    println!("{}", point_1);
    println!("{}", point_2);
}
