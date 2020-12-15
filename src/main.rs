#![allow(dead_code)]
use std::fs;

mod solutions { pub mod day_15; }
use solutions::day_15::solve;
const INPUT: &str = "./input/15";

fn main() {
    let input = fs::read_to_string(INPUT).expect("oh no!");
    let result = solve(&input);
    println!("{}", result)
}
