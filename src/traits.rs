mod shapes;

use std::f64::consts::PI;

use shapes::{ Rect, Circle };

trait Area {
    fn area(&self) -> f64;
}

impl Area for Rect {
    fn area(&self) -> f64 {
        return self.height * self.width;
    }
}

impl Area for Circle {
    fn area(&self) -> f64 {
        return self.radius * self.radius * PI;
    }
}

pub(crate) fn main() {
    let rect = Rect {
        x: 0.0,
        y: 0.0,
        width: 10.0,
        height: 10.0,
    };

    let cirle = Circle {
        x: 0.0,
        y: 0.0,
        radius: 10.0,
    };

    println!("rect area {}", rect.area());
    println!("circle area {}", cirle.area())
}
