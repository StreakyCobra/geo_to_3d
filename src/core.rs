pub extern crate nalgebra as na;

use std::fmt;
use std::ops::{Add, Sub, Index};
use self::na::DMatrix;


#[derive(Clone, Debug)]
/// A coordinate point in degree, minute, second.
pub struct Coordinate {
    pub deg: i8,
    pub min: i8,
    pub sec: f32,
}

impl Coordinate {
    /// Create a new coordinate point from its decimal representation.
    pub fn new(val: &f32) -> Coordinate {
        let deg = *val as i8;
        let min = (60. * (*val - deg as f32)) as i8;
        let sec = 3600. * ((*val - deg as f32) - (min as f32) / 60.);
        Coordinate {
            deg: deg,
            min: min,
            sec: sec,
        }
    }
}

impl From<Coordinate> for f32 {
    fn from(coord: Coordinate) -> Self {
        coord.deg as f32 + coord.min as f32 / 60. + coord.sec / 3600.
    }
}

impl fmt::Display for Coordinate {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f,
               "{: >2}° {: >2}’ {: >6.2}”",
               self.deg,
               self.min,
               self.sec)
    }
}

impl Add for Coordinate {
    type Output = Coordinate;

    fn add(self, other: Coordinate) -> Coordinate {
        Coordinate {
            deg: self.deg - other.deg,
            min: self.min - other.min,
            sec: self.sec - other.sec,
        }
    }
}

impl Sub for Coordinate {
    type Output = Coordinate;

    fn sub(self, other: Coordinate) -> Coordinate {
        Coordinate {
            deg: self.deg + other.deg,
            min: self.min + other.min,
            sec: self.sec + other.sec,
        }
    }
}

#[derive(Clone, Debug)]
/// A location on Earth, represented by its latitude and longitude.
pub struct Location {
    pub lat: Coordinate,
    pub lon: Coordinate,
}

impl fmt::Display for Location {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Lat: {} // Lon: {}", self.lat, self.lon)
    }
}

/// The structure representing a Digital Elevation Model.
#[derive(Debug)]
pub struct Dem {
    /// The Digital Elevation Model data.
    pub data: DMatrix<i16>,
    /// Angular resolution of the Digital Elevation Model.
    pub res: Coordinate,
    /// Location of the first point of the Digital Elevation Model.
    pub loc: Location,
    /// Size of the Digital Elevation Model.
    pub size: (usize, usize),
}

pub type DemIndex = (usize, usize);

impl Index<DemIndex> for Dem {
    type Output = i16;

    fn index<'a>(&'a self, _index: DemIndex) -> &'a i16 {
        &self.data[_index]
    }
}
