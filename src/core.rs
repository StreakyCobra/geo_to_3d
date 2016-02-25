use std::fmt;

pub struct Coord {
    pub lat: f32,
    pub lon: f32,
}

impl fmt::Display for Coord {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Coord::[lat: {}, lon: {}]", self.lat, self.lon)
    }
}
