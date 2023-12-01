use std::fs;

fn main() {
    let input = fs::read_to_string("input.txt")
        .expect("input.txt cannot be opened");

    let mut line_sum = 0;

    for line in input.lines(){
        //Find first digit in line
        let first_digit = 'first_loop: loop {
            for symbol in line.chars().into_iter() {
                if symbol.is_ascii_digit() {
                    break 'first_loop symbol;
                }
            }
        };
        //Find last digit in line
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
}
