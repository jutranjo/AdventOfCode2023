use std::fs;

use _04::{solve_part1,solve_part2};

fn main() {
    let file_location = "input.txt";
    let example = fs::read_to_string(file_location).expect(&format!("Couldn't open file: {}",file_location));

    println!("{}",solve_part1(&example));

    println!("{}",solve_part2(&example));
}
