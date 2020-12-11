#![allow(dead_code)]
use std::fs;

mod solutions { pub mod day_11; }
use solutions::day_11::solve;
const INPUT: &str = "./input/11";

fn main() {
    let input = fs::read_to_string(INPUT).expect("oh no!");
    let result = solve(&input);
    println!("{}", result)
}
