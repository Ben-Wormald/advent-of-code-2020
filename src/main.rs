#![allow(dead_code)]
use std::fs;

mod solutions { pub mod day_09; }
use solutions::day_09::solve;
const INPUT: &str = "./input/09";

fn main() {
    let input = fs::read_to_string(INPUT).expect("oh no!");
    let result = solve(&input);
    println!("{}", result)
}
