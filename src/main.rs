#![allow(dead_code)]
use std::fs;

mod solutions { pub mod day_06; }
use solutions::day_06::solve;
const INPUT: &str = "./input/06";

fn main() {
    let input = fs::read_to_string(INPUT).expect("oh no!");
    let result = solve(&input);
    println!("{}", result)
}
