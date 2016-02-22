use std::fmt;

pub struct Point {
    pub x: f32,
    pub y: f32,
}

pub struct Position(pub Point);

impl fmt::Display for Point {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Point: (x: {}, y: {})", self.x, self.y)
    }
}

impl fmt::Display for Position {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Position: (LAT: {}, LGT: {})", self.0.x, self.0.y)
    }
}
