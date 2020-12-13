#![allow(dead_code)]
use std::fs;

mod solutions { pub mod day_13; }
use solutions::day_13::solve;
const INPUT: &str = "./input/13";

fn main() {
    let input = fs::read_to_string(INPUT).expect("oh no!");
    let result = solve(&input);
    println!("{}", result)
}
