use std::fmt::{ Display, Formatter };
use super::area::Area;

pub struct Rect {
    pub x: f64,
    pub y: f64,
    pub width: f64,
    pub height: f64,
}

// we can implement method on something we don't have it here "Rect"
impl Area for Rect {
    fn area(&self) -> f64 {
        self.height * self.width
    }
}

impl Default for Rect {
    fn default() -> Self {
        Rect {
            x: 0.0,
            y: 0.0,
            width: 10.0,
            height: 10.0,
        }
    }
}

impl Display for Rect {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "Rectangle({},{}), {} x {}", self.x, self.y, self.width, self.height)
    }
}
