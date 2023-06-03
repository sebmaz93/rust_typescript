use std::fs::read_to_string;

fn main() {
    let it: Vec<_> = vec![1, 2, 3]
        .iter()
        .map(|i| i + 1)
        .collect();

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

    // let col = RGB::Green

    // use method that was implemented into the ENUM
    // col.is_green()
}

enum Color {
    Red,
    Green,
    Blue,
    Yellow,
}

impl Color {
    fn is_green(&self) -> bool {
        if let Color::Green = self {
            return true;
        }
        return false;
    }

    fn is_green_parts(&self) -> bool {
        return match self {
            Color::Red => false,
            Color::Green => false,
            Color::Blue => true,
            Color::Yellow => true,
        };
    }
}

fn print_color(color: Color) {
    match color {
        Color::Red => { println!("Red") }
        Color::Green => { println!("Green") }
        Color::Blue => { println!("Blue") }
        Color::Yellow => { println!("Yellow") }
    }
}

struct Custom {
    age: usize,
    name: String,
}

enum Item {
    Number(usize),
    String(String),
    MyCustom(Custom),
}

fn append(items: &mut Vec<Item>) {
    items.push(Item::String("Hello folks".into()))
}

fn practice(num: Option<usize>) -> usize {
    return num.unwrap_or(0) * 5;
}

fn practice2(list: Vec<usize>, idx: usize) -> usize {
    return list.get(idx).unwrap_or(&idx) * 5; // &idx because get returns ref to idx so must keep the types same
}
