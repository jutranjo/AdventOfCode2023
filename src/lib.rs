use std::{str::FromStr, vec};

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

#[derive(Clone,Debug,PartialEq)]
pub struct SeedRange {
    start: isize,
    end: isize,
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

fn read_input_to_seeds(input: &str) -> Vec<isize>{
    let cleaned_input = input.trim();

    let lines: Vec<&str> = cleaned_input.lines().collect();
    
    lines[0]
        .split(':')
        .nth(1)
        .unwrap()
        .split_whitespace()
        .map(|s| isize::from_str(s).expect("Failed to parse integer"))
        .collect()
}


fn map_with_string(all_almanacs: Vec<AlmanacMap>, name: String, seed: isize) -> isize {
    if let Some(using_almanac) = all_almanacs.iter().find(|almanac| almanac.name==name) {
        map_using_almanac(seed as isize,using_almanac)
    } else {
        panic!();
    }
}

fn map_seeds_to_location(all_almanacs: Vec<AlmanacMap>, seed_values: Vec<isize>) -> isize {
    let mut minimum: Option<isize> = None;

    for seed in seed_values {
        let soil = map_with_string(all_almanacs.clone(), String::from("seed-to-soil"), seed as isize);
        let fertilizer = map_with_string(all_almanacs.clone(), String::from("soil-to-fertilizer"), soil as isize);
        let water = map_with_string(all_almanacs.clone(), String::from("fertilizer-to-water"), fertilizer as isize);
        let light = map_with_string(all_almanacs.clone(), String::from("water-to-light"), water as isize);
        let temperature = map_with_string(all_almanacs.clone(), String::from("light-to-temperature"), light as isize);
        let humidity = map_with_string(all_almanacs.clone(), String::from("temperature-to-humidity"), temperature as isize);
        let location = map_with_string(all_almanacs.clone(), String::from("humidity-to-location"), humidity as isize);
        
        if let Some(existing_minimum) = minimum {
            if location<existing_minimum {
                minimum = Some(location);
            }
        } else {
            minimum = Some(location);
        }
    }

    if let Some(existing_minimum) = minimum {
        existing_minimum
    } else {
        -1
    }
}

fn read_input_to_seeds_pairs(input: &str) -> Vec<Vec<isize>>{
    let seed_numbers = read_input_to_seeds(input);
    let seed_pairs:Vec<_> = seed_numbers.chunks(2).map(|chunk| chunk.to_vec()).collect();

    let mut total_count = 0;
    for pair in &seed_pairs {
        println!("Plan is checking numbers from {} to {}, that's {} numbers to check",pair[0],pair[0]+pair[1],pair[1]);
        total_count += pair[1];
    }
    println!("Total number of numbers to check is {:e}",total_count);
    seed_pairs
}

fn _map_seed_ranges_to_location(all_almanacs: Vec<AlmanacMap>, seed_pairs: Vec<Vec<isize>>) -> isize {
    let mut minimum: Option<isize> = None;

    for pair in seed_pairs {
        println!("Doing numbers from {} to {}, that's {} numbers to check",pair[0],pair[0]+pair[1],pair[1]);
        for increase in 0..pair[1] {
            let seed = pair[0]+increase;
            let soil = map_with_string(all_almanacs.clone(), String::from("seed-to-soil"), seed as isize);
            let fertilizer = map_with_string(all_almanacs.clone(), String::from("soil-to-fertilizer"), soil as isize);
            let water = map_with_string(all_almanacs.clone(), String::from("fertilizer-to-water"), fertilizer as isize);
            let light = map_with_string(all_almanacs.clone(), String::from("water-to-light"), water as isize);
            let temperature = map_with_string(all_almanacs.clone(), String::from("light-to-temperature"), light as isize);
            let humidity = map_with_string(all_almanacs.clone(), String::from("temperature-to-humidity"), temperature as isize);
            let location = map_with_string(all_almanacs.clone(), String::from("humidity-to-location"), humidity as isize);
            
            if let Some(existing_minimum) = minimum {
                if location<existing_minimum {
                    minimum = Some(location);
                }
            } else {
                minimum = Some(location);
            }  
        }
        
    }

    if let Some(existing_minimum) = minimum {
        existing_minimum
    } else {
        -1
    }
}

pub fn solve_part1(input: &str) {
    let all_almanacs = read_input_to_almanac(&input);
    let seed_values = read_input_to_seeds(&input);

    let location_minimum = map_seeds_to_location(all_almanacs, seed_values);

    //println!("Minimum is {:?}",list_of_locations.iter().min().unwrap());
    println!("Minimum is {}",location_minimum);

    ()
}

fn convert_pairs_to_ranges(seed_pairs: &Vec<Vec<isize>>) -> Vec<SeedRange> {
    let mut constructed_pairs: Vec<SeedRange> = vec![];

    for pair in seed_pairs {
        let new_seed_range = SeedRange {start: pair[0], end: pair[0]+pair[1]};
        constructed_pairs.push(new_seed_range);
    }

    constructed_pairs

}
    
fn case1(_entry: &AlmanacEntry, range: &SeedRange, remaining_ranges: &mut Vec<SeedRange>, _output_ranges: &mut Vec<SeedRange>) -> () {
    // [------] <----->
    remaining_ranges.push(range.clone());
}


fn case2(entry: &AlmanacEntry, range: &SeedRange, remaining_ranges: &mut Vec<SeedRange>, output_ranges: &mut Vec<SeedRange>) -> () {
    // [----<-]--->
    let cut_off_range = SeedRange { start: range.start, end: (entry.source_start-1) as isize };

    let end_of_moved_range =  std::cmp::min(entry.destination_start+entry.range_length, entry.destination_start+range.end as usize-entry.source_start);

    let moved_range = SeedRange { start: entry.destination_start as isize, end: end_of_moved_range as isize };
    remaining_ranges.push(cut_off_range);
    output_ranges.push(moved_range);
}


fn case3(entry: &AlmanacEntry, range: &SeedRange, remaining_ranges: &mut Vec<SeedRange>, output_ranges: &mut Vec<SeedRange>) -> () {
     // <---[->----] 
    let cut_off_range = SeedRange { start: (entry.source_start+entry.range_length) as isize, end: range.end };

    let entry_not_touched_numbers =  range.start as usize- entry.source_start;
    
    let moved_range = SeedRange { start: (entry.destination_start+ entry_not_touched_numbers) as isize, end: (entry.destination_start+entry.range_length-1) as isize};

    remaining_ranges.push(cut_off_range);
    output_ranges.push(moved_range);
}


fn case4(entry: &AlmanacEntry, range: &SeedRange, _remaining_ranges: &mut Vec<SeedRange>, output_ranges: &mut Vec<SeedRange>) -> () {
    // <--[----]--->
    
    let range_span = range.end - range.start;
    let entry_not_touched_numbers =  range.start as usize- entry.source_start;

    let moved_range = SeedRange { start: (entry.destination_start+ entry_not_touched_numbers) as isize, end: (entry.destination_start as isize + range_span)+1};

    output_ranges.push(moved_range);
    //(SeedRange {start: 1,end: 2}, SeedRange {start: 1,end: 2})
}


fn case5(entry: &AlmanacEntry, range: &SeedRange, remaining_ranges: &mut Vec<SeedRange>, output_ranges: &mut Vec<SeedRange>) -> () {
    // [--<--->---]

    let leftbreak = entry.source_start-1;
    let rightbreak = entry.source_start+entry.range_length+1;

    let left_part_of_remain = SeedRange{ start: range.start, end: leftbreak as isize};
    let right_part_of_remain = SeedRange{ start: rightbreak as isize, end: range.end };
    
    let moved_range = SeedRange {start: entry.destination_start as isize, end: (entry.destination_start+entry.range_length) as isize};

    remaining_ranges.push(left_part_of_remain);
    remaining_ranges.push(right_part_of_remain);
    output_ranges.push(moved_range);
}


fn case6(_entry: &AlmanacEntry, range: &SeedRange, remaining_ranges: &mut Vec<SeedRange>, _output_ranges: &mut Vec<SeedRange>) -> () {
    // <-----> [------]
    remaining_ranges.push(range.clone());
}


fn map_range_with_string(all_almanacs: &Vec<AlmanacMap>, input_range: SeedRange, name: String) -> Vec<SeedRange> {

    let mut remaining_ranges: Vec<SeedRange> = vec![input_range];
    let mut output_ranges: Vec<SeedRange> = vec![];
    let mut ranges_to_revisit: Vec<SeedRange>;

    if let Some(almanac) = all_almanacs.iter().find(|almanac| almanac.name==name) {
        //vsak entry mora premapirati kar mu pripada
        //kar ne premapira, ostane za naslednji entry
            //prvi entry: v seznamu remaining_ranges razkosa prvega na 2 kosa
            //tista dva kosa sta v remaining ranges, naslednji jih mora oba pregledati
        for entry in almanac.entries.iter() {
            ranges_to_revisit = vec![];
            for range_slice in &remaining_ranges {
                //  a<----->b
                let a = entry.source_start as isize;
                let b = (entry.source_start+entry.range_length - 1) as isize;
                
                // c[------]d
                let c = range_slice.start;
                let d = range_slice.end;

                match (a<c,a<d,b<d,b<c) {
                    (true, true, true, true) => { case1(entry, range_slice, &mut ranges_to_revisit, &mut output_ranges); }     // [------] <----->
                    (true, true, true, false) => { case2(entry, range_slice, &mut ranges_to_revisit, &mut output_ranges); }     // [----<-]--->
                    (false, true, false, false) => { case3(entry, range_slice, &mut ranges_to_revisit, &mut output_ranges); }   // <---[->----] 
                    (false, true, true, false) => { case4(entry, range_slice, &mut ranges_to_revisit, &mut output_ranges); }    // <--[----]--->
                    (true, true, false, false) => { case5(entry, range_slice, &mut ranges_to_revisit, &mut output_ranges); }    // [--<--->---]
                    (false, false, false, false) => { case6(entry, range_slice, &mut ranges_to_revisit, &mut output_ranges); }  // <-----> [------]
                    _ => {panic!("Unforseen variant of ordering");}
                }
            }
            remaining_ranges.append(&mut ranges_to_revisit);
        }
        output_ranges.append(&mut remaining_ranges);
        //preostanek se po preverbi vseh entryijev pripopa na izhod output_ranges
    } else {
        panic!();
    }

    output_ranges
}

fn map_seed_ranges_to_location(all_almanacs: Vec<AlmanacMap>, seed_ranges: Vec<SeedRange>) -> isize {
    for range in seed_ranges {
        //mogoce prepisem tole da sprejme namesto range kar direkt seed_ranges, vse zgleda da bi pasalo
        let soil_ranges = map_range_with_string(&all_almanacs, range, String::from("seed-to-soil"));
        //potem lahko samo range naprej podajam po vseh almanahih
        for soil in soil_ranges {}


        
    }

    //na koncu preberem vse range od location, treba samo starte brat pa med njimi najmanjsega vzet
    //to je to


    

    3
}

pub fn solve_part2(input: &str) {
    let all_almanacs = read_input_to_almanac(&input);
    let seed_pairs = read_input_to_seeds_pairs(&input);
    let seed_ranges = convert_pairs_to_ranges(&seed_pairs);
    
    println!("{:?}",seed_ranges);

    let location_minimum = map_seed_ranges_to_location(all_almanacs, seed_ranges);

    //let location_minimum = map_seed_ranges_to_location(all_almanacs, seed_pairs);

    //println!("Minimum is {:?}",location_minimum);

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
    use crate::{read_input_to_almanac, map_using_almanac, case1, SeedRange, AlmanacEntry, case2, case3, case4, case5, case6};

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

    #[test]
    fn case1_test() {
        let range: SeedRange = SeedRange { start: 2, end: 4 };
        let entry: AlmanacEntry = AlmanacEntry { destination_start: 10, source_start: 6, range_length: 2 };

        let mut remaining_ranges: Vec<SeedRange> = vec![];
        let mut output_ranges: Vec<SeedRange> = vec![];

        case1(&entry, &range, &mut remaining_ranges,&mut output_ranges);

        assert_eq!(remaining_ranges,vec![range]);
        assert_eq!(output_ranges,vec![]);

    }

    #[test]
    fn case6_test() {
        let range: SeedRange = SeedRange { start: 20, end: 40 };
        let entry: AlmanacEntry = AlmanacEntry { destination_start: 10, source_start: 6, range_length: 2 };

        let mut remaining_ranges: Vec<SeedRange> = vec![];
        let mut output_ranges: Vec<SeedRange> = vec![];

        case6(&entry, &range, &mut remaining_ranges,&mut output_ranges);

        assert_eq!(remaining_ranges,vec![range]);
        assert_eq!(output_ranges,vec![]);

    }

    #[test]
    fn case2_test() {
        let range: SeedRange = SeedRange { start: 1, end: 6 };
        let entry: AlmanacEntry = AlmanacEntry { destination_start: 10, source_start: 4, range_length: 4 };

        let mut remaining_ranges: Vec<SeedRange> = vec![];
        let mut output_ranges: Vec<SeedRange> = vec![];

        let expected_remaining = vec![SeedRange { start: 1, end: 3 }];
        let expected_ouput = vec![SeedRange { start: 10, end: 12 }];

        case2(&entry, &range, &mut remaining_ranges,&mut output_ranges);

        assert_eq!(remaining_ranges,expected_remaining);
        assert_eq!(output_ranges,expected_ouput);

    }

    
    #[test]
    fn case3_test() {
        let range: SeedRange = SeedRange { start: 9, end: 16 };
        let entry: AlmanacEntry = AlmanacEntry { destination_start: 1, source_start: 8, range_length: 4 };

        let mut remaining_ranges: Vec<SeedRange> = vec![];
        let mut output_ranges: Vec<SeedRange> = vec![];

        let expected_remaining = vec![SeedRange { start: 12, end: 16 }];
        let expected_ouput = vec![SeedRange { start: 2, end: 4 }];

        case3(&entry, &range, &mut remaining_ranges,&mut output_ranges);

        assert_eq!(remaining_ranges,expected_remaining);
        assert_eq!(output_ranges,expected_ouput);

    }
    
    #[test]
    fn case4_test() {
        let range: SeedRange = SeedRange { start: 9, end: 11 };
        let entry: AlmanacEntry = AlmanacEntry { destination_start: 1, source_start: 8, range_length: 4 };

        let mut remaining_ranges: Vec<SeedRange> = vec![];
        let mut output_ranges: Vec<SeedRange> = vec![];

        let expected_remaining = vec![];
        let expected_ouput = vec![SeedRange { start: 2, end: 4 }];

        case4(&entry, &range, &mut remaining_ranges,&mut output_ranges);

        assert_eq!(remaining_ranges,expected_remaining);
        assert_eq!(output_ranges,expected_ouput);

    }

    
    #[test]
    fn case4_test2() {
        let range: SeedRange = SeedRange { start: 9, end: 11 };
        let entry: AlmanacEntry = AlmanacEntry { destination_start: 20, source_start: 7, range_length: 5 };

        let mut remaining_ranges: Vec<SeedRange> = vec![];
        let mut output_ranges: Vec<SeedRange> = vec![];

        let expected_remaining = vec![];
        let expected_ouput = vec![SeedRange { start: 22, end: 23 }];

        case4(&entry, &range, &mut remaining_ranges,&mut output_ranges);

        assert_eq!(remaining_ranges,expected_remaining);
        assert_eq!(output_ranges,expected_ouput);

    }

    
    #[test]
    fn case5_test() {
        let range: SeedRange = SeedRange { start: 2, end: 20 };
        let entry: AlmanacEntry = AlmanacEntry { destination_start: 30, source_start: 10, range_length: 2 }; //10, 11, 12 letijo ven

        let mut remaining_ranges: Vec<SeedRange> = vec![];
        let mut output_ranges: Vec<SeedRange> = vec![];

        let expected_remaining = vec![SeedRange { start: 2, end: 9 },SeedRange { start: 13, end: 20 }];
        let expected_ouput = vec![SeedRange { start: 30, end: 32 }];

        case5(&entry, &range, &mut remaining_ranges,&mut output_ranges);

        assert_eq!(remaining_ranges,expected_remaining);
        assert_eq!(output_ranges,expected_ouput);

    }
}

    