#![allow(dead_code)]
use std::fs;

mod solutions { pub mod day_03; }
use solutions::day_03::solve;
const INPUT: &str = "./input/03";

fn main() {
    let input = fs::read_to_string(INPUT).expect("oh no!");
    let result = solve(&input);
    println!("{}", result)
}
