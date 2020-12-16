#![allow(dead_code)]
use std::fs;

mod solutions { pub mod day_16; }
use solutions::day_16::solve;
const INPUT: &str = "./input/16";

fn main() {
    let input = fs::read_to_string(INPUT).expect("oh no!");
    let result = solve(&input);
    println!("{}", result)
}
