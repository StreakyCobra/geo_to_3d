use std::fmt;

pub struct Position {
    pub lat: f32,
    pub lon: f32,
}

impl fmt::Display for Position {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Position::[lat: {}, lon: {}]", self.lat, self.lon)
    }
}
