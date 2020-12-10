#![allow(dead_code)]
use std::fs;

mod solutions { pub mod day_10; }
use solutions::day_10::solve;
const INPUT: &str = "./input/10";

fn main() {
    let input = fs::read_to_string(INPUT).expect("oh no!");
    let result = solve(&input);
    println!("{}", result)
}
