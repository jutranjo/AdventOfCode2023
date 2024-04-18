use std::fs;
use _07::solve_part1;

fn main() {
    let file_location = "input.txt";
    let input = fs::read_to_string(file_location).expect(&format!("Couldn't open file: {}",file_location));

    let part_1_solution = solve_part1(&input);
    println!("part 1 sum is {}",part_1_solution);
}
