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
        .for_each(|(_, x)| println!("{}", x))
}
