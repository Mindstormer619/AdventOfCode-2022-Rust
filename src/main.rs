#![allow(non_snake_case)]

use crate::{day01::solve, day02::{strat1, strat2}};

pub mod day01;
pub mod day02;

pub mod utils {
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

    solve("test");
    solve("prod");

    dbg!(strat1("test"));
    dbg!(strat1("prod"));

    dbg!(strat2("test"));
    dbg!(strat2("prod"));
}
