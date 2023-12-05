use std::fs;
use _05::solve_part1;

fn main() {
    let file_location = "example.txt";
    let example = fs::read_to_string(file_location).expect(&format!("Couldn't open file: {}",file_location));

    solve_part1(&example);
}
