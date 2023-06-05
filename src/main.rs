// mod exr_1;
// mod exr_2;
// mod stack_heap;
mod shapes;

use crate::shapes::{ rect::Rect, circle::Circle, area::Area };

pub fn main() {
    let rect = Rect {
        x: 0.0,
        y: 0.0,
        width: 10.0,
        height: 10.0,
    };

    let circle = Circle {
        x: 0.0,
        y: 0.0,
        radius: 10.0,
    };

    println!("rect area {}", rect.area());
    println!("circle area {}", circle.area())
}
