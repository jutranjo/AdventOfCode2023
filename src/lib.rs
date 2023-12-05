use std::str::FromStr;

struct almanac_map<'a> {
    name: &'a str,
    entries: Vec<almanac_entry>,
}

struct almanac_entry {
    destination_range: usize,
    source_range: usize,
    range_length: usize,
}


pub fn read_input_to_almanac(input: &str) {
    
    // Remove leading and trailing whitespaces
    let cleaned_input = input.trim();

    // Split the input into lines
    let lines: Vec<&str> = cleaned_input.lines().collect();

    // Parse seeds
    let seed_values: Vec<usize> = lines[0]
        .split(':')
        .nth(1)
        .unwrap()
        .split_whitespace()
        .map(|s| usize::from_str(s).expect("Failed to parse integer"))
        .collect();

    println!("Seeds: {:?}", seed_values);

    let lines_without_seeds = lines[2..].to_vec();

    println!("{:?}",lines_without_seeds);

    let all_almanacs: Vec<almanac_map>;
    let mut current_almanac: Option<almanac_map> = None;

    let sum: usize = lines_without_seeds
                .iter()
                .map(|line| {

                    print!("{} -> ",line);
                    match line.trim() {
                        s if s.ends_with("map:") => {
                            let map_name = &s[..s.len() - " map:".len()];
                            println!("Found a map name! It's: {}", map_name);
                        }
                        s if s.is_empty() => {
                            println!("empty line");
                        }
                        _ => {
                            println!("Found numbers!: {}",line);
                        }
                    }

                    0
                }).sum();

    println!("{:?}",sum);



}

pub fn solve_part1(input: &str) {
    read_input_to_almanac(&input);

    ()
}