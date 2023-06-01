use std::fs::read_to_string;

fn main() {
    let it: Vec<_> = vec![1, 2, 3].iter().map(|i| i + 1).collect();

    println!("{:?}", it);

    let contents = read_to_string("./src/hello.txt").unwrap();
    contents.lines().for_each(|x| println!("{}", x));

    // print every other line
    contents
        .lines()
        .enumerate()
        .filter(|(idx, _)| idx % 2 == 0)
        .for_each(|(_, x)| println!("{}", x));

    // print every other line, skip first 2 lines and takes 2 lines
    contents
        .lines()
        .enumerate()
        .filter(|(idx, _)| idx % 2 == 0)
        .skip(2)
        .take(2)
        .for_each(|(_, x)| println!("{}", x));

    print_color(RGB);

    let col = RGB::Green;

    // use method that was implemented into the ENUM
    // col.is_green()
}

enum RGB {
    Red,
    Green,
    Blue,
    Yellow,
}

impl RGB {
    fn is_green(&self) -> bool {
        if let RGB::Green = self {
            return true;
        }
        return false
    }

    fn is_green_parts(&self) -> bool {
        return match self {
            RGB::Red => false,
            RGB::Green => false,
            RGB::Blue => true,
            RGB::Yellow => true
        }
    }
}

fn print_color(color: RGB) {
    match color: RGB {
        RGB::Red => {
            println!("Red")
        }
        RGB::Green => {
            println!("Green")
        }
        RGB::Blue => {
            println!("Blue")
        }
        RGB::Yellow => {
            println!("Yellow")
        }
    }
}
