#![allow(dead_code)]
use std::fs;

mod solutions { pub mod day_12; }
use solutions::day_12::solve;
const INPUT: &str = "./input/12";

fn main() {
    let input = fs::read_to_string(INPUT).expect("oh no!");
    let result = solve(&input);
    println!("{}", result)
}
