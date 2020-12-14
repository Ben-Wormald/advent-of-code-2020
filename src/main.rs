#![allow(dead_code)]
use std::fs;

mod solutions { pub mod day_14; }
use solutions::day_14::solve;
const INPUT: &str = "./input/14";

fn main() {
    let input = fs::read_to_string(INPUT).expect("oh no!");
    let result = solve(&input);
    println!("{}", result)
}
