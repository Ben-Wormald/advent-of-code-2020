#![allow(dead_code)]
use std::fs;

mod solutions { pub mod day_08; }
use solutions::day_08::solve;
const INPUT: &str = "./input/08";

fn main() {
    let input = fs::read_to_string(INPUT).expect("oh no!");
    let result = solve(&input);
    println!("{}", result)
}
