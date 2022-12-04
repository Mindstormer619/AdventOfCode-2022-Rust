#![allow(non_snake_case)]
#![allow(dead_code)]

mod day01;
mod day02;
mod day03;

mod utils {
    use std::{
        fs::File,
        io::{BufRead, BufReader},
        path::Path,
    };

    pub fn linesFromFile(filename: impl AsRef<Path>) -> Vec<String> {
        let file = File::open(filename).expect("No fucking way");
        let buf = BufReader::new(file);

        buf.lines()
            .map(|l| l.expect("Could not parse line"))
            .collect()
    }
}

fn main() {
    println!("Hello, elves!");

    dbg!(day03::sumOfPriorities("test"));
    dbg!(day03::sumOfPriorities("prod"));

    dbg!(day03::elfGroups("test"));
    dbg!(day03::elfGroups("prod"));
}
