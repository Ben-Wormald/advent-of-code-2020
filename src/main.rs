#![allow(dead_code)]
use std::fs;

mod solutions { pub mod day_04; }
use solutions::day_04::solve;
const INPUT: &str = "./input/04";

fn main() {
    let input = fs::read_to_string(INPUT).expect("oh no!");
    let result = solve(&input);
    println!("{}", result)
}
