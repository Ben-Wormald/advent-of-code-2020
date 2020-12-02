#![allow(dead_code)]
use std::fs;

mod solutions { pub mod day_02; }
use solutions::day_02::solve;
const INPUT: &str = "./input/02";

fn main() {
    let input = fs::read_to_string(INPUT).expect("oh no!");
    let result = solve(&input);
    println!("{}", result)
}
