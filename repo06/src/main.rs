use std::fs;
use _06::{solve_part1,solve_part2};

fn main() {
    let file_location = "input.txt";
    let example = fs::read_to_string(file_location).expect(&format!("Couldn't open file: {}",file_location));

    
    let result1 = solve_part1(&example);
    println!("Result for part 1 is {}",result1);

    let result2 = solve_part2(&example);
    println!("Result for part 2 is {}",result2);
    
    
}
