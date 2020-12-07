#![allow(dead_code)]
use std::fs;

mod solutions { pub mod day_07; }
use solutions::day_07::solve;
const INPUT: &str = "./input/07";

fn main() {
    let input = fs::read_to_string(INPUT).expect("oh no!");
    let result = solve(&input);
    println!("{}", result)
}
