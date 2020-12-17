#![allow(dead_code)]
use std::fs;

mod solutions { pub mod day_17; }
use solutions::day_17::solve;
const INPUT: &str = "./input/17";

fn main() {
    let input = fs::read_to_string(INPUT).expect("oh no!");
    let result = solve(&input);
    println!("{}", result)
}
