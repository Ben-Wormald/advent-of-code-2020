#![allow(dead_code)]
use std::fs;

mod solutions { pub mod day_05; }
use solutions::day_05::solve;
const INPUT: &str = "./input/05";

fn main() {
    let input = fs::read_to_string(INPUT).expect("oh no!");
    let result = solve(&input);
    println!("{}", result)
}
