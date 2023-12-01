use std::fs;

fn main() {
    let input = fs::read_to_string("input.txt").expect("input.txt cannot be opened");

    let mut line_sum = 0;

    for line in input.lines(){
        let first_digit = 'first_loop: loop {
            for symbol in line.chars().into_iter() {
                if symbol.is_ascii_digit() {
                    break 'first_loop symbol;
                }
            }
        };
        let last_digit = 'first_loop: loop {
            for symbol in line.chars().into_iter().rev() {
                if symbol.is_ascii_digit() {
                    break 'first_loop symbol;
                }
            }
        };
        line_sum += last_digit.to_digit(10).unwrap_or(0)+first_digit.to_digit(10).unwrap_or(0)*10;
    }
    println!("{}",line_sum);

    //let input = String::from("");
    let line_sum: u32 = input
            .lines()
            .filter_map(|line| {
                let first_digit = line.chars().find(|c| c.is_ascii_digit())?;
                let last_digit = line.chars().rev().find(|c| c.is_ascii_digit())?;

                Some(last_digit.to_digit(10)? + first_digit.to_digit(10)? * 10)
            })
            .sum();

    println!("{}",line_sum);
}
