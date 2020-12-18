#![allow(dead_code)]
use std::fs;

mod solutions { pub mod day_18; }
use solutions::day_18::solve;
const INPUT: &str = "./input/18";

fn main() {
    let input = fs::read_to_string(INPUT).expect("oh no!");
    let result = solve(&input);
    println!("{}", result)
}
