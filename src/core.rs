use std::fmt;

pub struct Coordinate {
    pub deg: i8,
    pub min: i8,
    pub sec: f32,
}

impl Coordinate {
    pub fn new(val: &f32) -> Coordinate {
        let deg = *val as i8;
        let min = (60. * (*val - deg as f32)) as i8;
        let sec = 3600. * ((*val - deg as f32) - (min as f32) / 60.);
        Coordinate {deg: deg, min: min, sec: sec}
    }
}

pub struct Location {
    pub lat: Coordinate,
    pub lon: Coordinate,
}

impl fmt::Display for Coordinate {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{: >2}° {: >2}’ {: >6.2}”", self.deg, self.min, self.sec)
    }
}

impl fmt::Display for Location {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Lat: {} // Lon: {}", self.lat, self.lon)
    }
}
