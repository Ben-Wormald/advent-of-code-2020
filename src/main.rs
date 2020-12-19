#![allow(dead_code)]
use std::fs;

mod solutions { pub mod day_19; }
use solutions::day_19::solve;
const INPUT: &str = "./input/19";

fn main() {
    let input = fs::read_to_string(INPUT).expect("oh no!");
    let result = solve(&input);
    println!("{}", result)
}
