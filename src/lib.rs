use std::str::FromStr;

#[derive(Clone,Debug)]
pub struct AlmanacMap {
    name: String,
    entries: Vec<AlmanacEntry>,
}
#[derive(Clone,Debug)]
pub struct AlmanacEntry {
    destination_start: usize,
    source_start: usize,
    range_length: usize,
}


pub fn read_input_to_almanac(input: &str) -> Vec<AlmanacMap> {
    
    let cleaned_input = input.trim();

    let lines: Vec<&str> = cleaned_input.lines().collect();


    let lines_without_seeds = lines[2..].to_vec();

    let mut all_almanacs: Vec<AlmanacMap> = Vec::new();
    let mut current_almanac: Option<AlmanacMap> = None;

    for line in lines_without_seeds.iter(){
                    match line.trim() {
                        s if s.ends_with("map:") => {
                            let map_name = &s[..s.len() - " map:".len()];

                            let entry_list: Vec<AlmanacEntry> = vec![];
                            current_almanac = Some(AlmanacMap { name: map_name.to_string(), entries: entry_list })
                        }
                        s if s.is_empty() => {
                            if let Some(valid_almanac) = &current_almanac {
                                all_almanacs.push(valid_almanac.clone())
                            }
                        }
                        _ => {
                            let numbers: Vec<usize> = line.split_whitespace().map(|s| s.parse::<usize>().expect("Failed to parse integer")).collect();
                            let new_entry:AlmanacEntry = AlmanacEntry { destination_start: numbers[0], source_start:numbers[1], range_length:numbers[2] };
                            if let Some(valid_almanac) = &mut current_almanac {
                                valid_almanac.entries.push(new_entry);
                            }
                            
                        }
                    }
                }
    if let Some(valid_almanac) = &current_almanac {
        all_almanacs.push(valid_almanac.clone())
    }
    
    all_almanacs

}

fn read_input_to_seeds(input: &str) -> Vec<usize>{
    let cleaned_input = input.trim();

    let lines: Vec<&str> = cleaned_input.lines().collect();
    
    lines[0]
        .split(':')
        .nth(1)
        .unwrap()
        .split_whitespace()
        .map(|s| usize::from_str(s).expect("Failed to parse integer"))
        .collect()
}

fn map_with_string(all_almanacs: Vec<AlmanacMap>, name: String, seed: isize) -> isize {
    if let Some(using_almanac) = all_almanacs.iter().find(|almanac| almanac.name==name) {
        map_using_almanac(seed as isize,using_almanac)
    } else {
        panic!();
    }
}

pub fn solve_part1(input: &str) {
    let all_almanacs = read_input_to_almanac(&input);
    let seed_values = read_input_to_seeds(&input);

    let mut list_of_locations: Vec<isize> = vec![];

    for seed in seed_values {
        let soil = map_with_string(all_almanacs.clone(), String::from("seed-to-soil"), seed as isize);
        let fertilizer = map_with_string(all_almanacs.clone(), String::from("soil-to-fertilizer"), soil as isize);
        let water = map_with_string(all_almanacs.clone(), String::from("fertilizer-to-water"), fertilizer as isize);
        let light = map_with_string(all_almanacs.clone(), String::from("water-to-light"), water as isize);
        let temperature = map_with_string(all_almanacs.clone(), String::from("light-to-temperature"), light as isize);
        let humidity = map_with_string(all_almanacs.clone(), String::from("temperature-to-humidity"), temperature as isize);
        let location = map_with_string(all_almanacs.clone(), String::from("humidity-to-location"), humidity as isize);
        
        list_of_locations.push(location);
    }

    println!("Minimum is {:?}",list_of_locations.iter().min().unwrap());

    ()
}

fn map_using_almanac(start: isize, almanac: &AlmanacMap) -> isize {

    for entry in almanac.entries.iter() {
        let source_start = entry.source_start;
        let source_end = entry.source_start+entry.range_length - 1;
        
        if start>=source_start as isize && start<=source_end as isize {
            let shift:isize = entry.source_start as isize-entry.destination_start as isize;
            let output = start-shift;
            return output;
        }
    }
    start
}

#[cfg(test)]
mod tests {
    use std::fs;
    use crate::{read_input_to_almanac, map_using_almanac};

    #[test]
    fn mapping_seed_to_soil() {
        let file_location = "example.txt";
        let example = fs::read_to_string(file_location).expect(&format!("Couldn't open file: {}",file_location));
        let all_almanacs = read_input_to_almanac(&example);


        if let Some(using_almanac) = all_almanacs.iter().find(|almanac| almanac.name=="seed-to-soil") {
            let start_seed = 79;    
            let start_seed_mapped = map_using_almanac(start_seed,using_almanac);
            assert_eq!(start_seed_mapped, 81);

            let start_seed = 14;    
            let start_seed_mapped = map_using_almanac(start_seed,using_almanac);
            assert_eq!(start_seed_mapped, 14);

            let start_seed = 55;    
            let start_seed_mapped = map_using_almanac(start_seed,using_almanac);
            assert_eq!(start_seed_mapped, 57);

            let start_seed = 13;    
            let start_seed_mapped = map_using_almanac(start_seed,using_almanac);
            assert_eq!(start_seed_mapped, 13);
        } else {
            panic!()
        }
    }

    #[test]
    fn mapping_soil_to_fertilizer() {
        let file_location = "example.txt";
        let example = fs::read_to_string(file_location).expect(&format!("Couldn't open file: {}",file_location));
        let all_almanacs = read_input_to_almanac(&example);


        if let Some(using_almanac) = all_almanacs.iter().find(|almanac| almanac.name=="soil-to-fertilizer") {
            let start_seed = 81;    
            let start_seed_mapped = map_using_almanac(start_seed,using_almanac);
            assert_eq!(start_seed_mapped, 81);

            let start_seed = 14;    
            let start_seed_mapped = map_using_almanac(start_seed,using_almanac);
            assert_eq!(start_seed_mapped, 53);

            let start_seed = 57;    
            let start_seed_mapped = map_using_almanac(start_seed,using_almanac);
            assert_eq!(start_seed_mapped, 57);

            let start_seed = 13;    
            let start_seed_mapped = map_using_almanac(start_seed,using_almanac);
            assert_eq!(start_seed_mapped, 52);
        } else {
            panic!("testfailed");
        }
    }
}

    