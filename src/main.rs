use shapes::collisions::Collidable;

use crate::shapes::{area::Area, circle::Circle, rect::Rect};

mod shapes;

fn main() {
    let rect = Rect::default();
    let rect2 = Rect::default();
    let circle = Circle {
        x: 0.5,
        y: 0.5,
        radius: 4.0,
    };

    let circle2 = Circle {
        x: 1.5,
        y: 1.5,
        radius: 4.0,
    };

    rect.collide(&circle);
}
