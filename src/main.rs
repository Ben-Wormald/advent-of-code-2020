use std::fs;

mod solutions {
    pub mod day_01;
}

fn main() {
    let input = fs::read_to_string("./input/01")
        .expect("oh no");
    let result = solutions::day_01::solve(&input);
    println!("{}", result)
}
