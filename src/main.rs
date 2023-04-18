enum Color {
    Red,
    Green,
    Blue,
}

impl Color {
    fn is_green(&self) -> bool {
        if let Color::Green = &self {
            return true;
        }
        return false;
    }

    fn is_green_parts(&self) -> bool {
        match self {
            Color::Red => false,
            Color::Green => false,
            Color::Blue => true,
        }
    }
}

fn print_color(color: Color) {
    match color {
        Color::Red => print!("red"),
        Color::Blue => println!("blue"),
        Color::Green => println!("green"),
    }
}

fn main() {
    let foo: Color = Color::Green;

    println!("{}", foo.is_green());
}
