use std::fs;

pub(crate) fn main() {
    let file_name = std::env::args().nth(1).expect("this file name to be passed correctly.");

    let file = fs::read_to_string(file_name).expect("unable to read the file to string.");

    file.lines().for_each(|line| {
        if let Ok(value) = line.parse::<usize>() {
            println!("{}", value);
        }
        println!("Line not a number")
    })
}

// Ok() for errors
// Some() for options
