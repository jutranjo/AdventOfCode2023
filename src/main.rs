use std::{fs, collections::{HashMap}};

fn calculate_line_sum(input: &String) -> u32 {
    input
        .lines()
        .filter_map(|line| {
            let first_digit = line.chars().find(|c| c.is_ascii_digit())?;
            let last_digit = line.chars().rev().find(|c| c.is_ascii_digit())?;

            Some(last_digit.to_digit(10)? + first_digit.to_digit(10)? * 10)
        })
        .sum()
}

fn _calculate_line_sum_part2(input: &String) -> u32 {
    let number_map = HashMap::from([
        ("one",1),
        ("two",2),
        ("three",3),
        ("four",4),
        ("five",5),
        ("six",6),
        ("seven",7),
        ("eight",8),
        ("nine",9),
    ]);


    input
        .lines()
        .filter_map(|line| {
            let _first_digit_pos = line.chars().position(|c| c.is_ascii_digit())?;
            //println!("First digit position = {}",first_digit_pos);

            let mut lowest_index = line.chars().count();
            //println!("len  = {}",lowest_index);
            for (key, _value) in &number_map {
                let key_index = line.find(key).unwrap_or(line.chars().count());
                //println!("found {} at {:?}",key, key_index);    
                if key_index<lowest_index {
                    lowest_index = key_index;
                }
            }
            //println!("lowet index = {}",lowest_index);
            
            let first_digit = line.chars().find(|c| c.is_ascii_digit())?;
            let last_digit = line.chars().rev().find(|c| c.is_ascii_digit())?;

            Some(last_digit.to_digit(10)? + first_digit.to_digit(10)? * 10)
        })
        .sum()
}

fn find_first_occurrence<'a, F, K>(
    input: &'a str, 
    replacements: &HashMap<K, &'a str>, 
    find_fn: F
) -> (Option<usize>,Option<&'a str>)
    where
        F: Fn(&'a str, &K) -> Option<usize> {
    let mut earliest_index: Option<usize> = None;
    let mut earliest_key: Option<&str> = None;

    for (key,value) in replacements.iter() {
        if let Some(index) = find_fn(input,key) {
            match earliest_index {
                Some(earliest) if index < earliest => {
                    earliest_index = Some(index);
                    earliest_key = Some(value);
                }
                None => {
                    earliest_index = Some(index);
                    earliest_key = Some(value);
                }
                _ => {}
            }
        }
    }

    (earliest_index,earliest_key)
}

fn find_last_occurrence<'a, F, K>(
    input: &'a str, 
    replacements: &HashMap<K, &'a str>, 
    find_fn: F
) -> (Option<usize>,Option<&'a str>)
    where
        F: Fn(&'a str, &K) -> Option<usize> {
    let mut earliest_index: Option<usize> = None;
    let mut earliest_key: Option<&str> = None;

    for (key,value) in replacements.iter() {
        if let Some(index) = find_fn(input,key) {
            match earliest_index {
                Some(earliest) if index > earliest => {
                    earliest_index = Some(index);
                    earliest_key = Some(value);
                }
                None => {
                    earliest_index = Some(index);
                    earliest_key = Some(value);
                }
                _ => {}
            }
        }
    }

    (earliest_index,earliest_key)
}

fn calculate_line_sum_with_words(input: &String) -> u32 {
    let number_map = HashMap::from([
        ("one","1"),
        ("two","2"),
        ("three","3"),
        ("four","4"),
        ("five","5"),
        ("six","6"),
        ("seven","7"),
        ("eight","8"),
        ("nine","9"),
    ]);


    input
        .lines()
        .filter_map(|line| {
            println!("{}",line);
            let first_digit = line.chars().find(|c| c.is_ascii_digit());

            let mut first_digit_position = None;
            if let Some(digit) = first_digit {
                first_digit_position = line.chars().position(|c| c==digit);
                println!("First digit position is {:?}",first_digit_position);
            }
            

            let last_digit = line.chars().rev().find(|c| c.is_ascii_digit());

            let mut last_digit_position = None;
            if let Some(digit) = last_digit {
                last_digit_position = line.chars().position(|c| c==digit);
                println!("Last digit position is {:?}",last_digit_position);
            }
            

            let (first_word_position, first_word) = find_first_occurrence(line, &number_map, |s, k| s.find(k));
            if let Some(first_word_position) = first_word_position {
                println!("The first word position is {}",first_word_position);
            }
            
            let (last_word_position, last_word) = find_last_occurrence(line, &number_map, |s, k| s.rfind(k));
            if let Some(last_word_position) = last_word_position {
                println!("The last word position is {:?}",last_word_position);
            }

            let mut left_number: Option<char> = None;
            //let mut right_number: Option<&str> = None;

            match (first_digit_position,first_word_position) {
                (Some(first_digit_position),Some(first_word_position)) => {
                    if first_digit_position < first_word_position {
                        println!("First is a digit: {:?}", first_digit);
                        left_number = first_digit;
                    } else {                      
                        println!("First is a word: {:?}",first_word);
                        if let Some(first_word2number) = first_word{
                            left_number = number_map.get(first_word2number).map(|s| s.chars().next());
                        }
                    }
                }
                (Some(first_digit_position),None) => {
                    println!("Only digit exists");
                    println!("First is a digit: {:?}", first_digit);
                    left_number = first_digit;
                    //left_number = first_digit.map(|c| c.to_string().as_str());
                }
                (None,Some(first_word_position)) => {
                    println!("Only words exists");
                    if let Some(first_word2number) = first_word{
                        left_number = number_map.get(first_word2number).map(|s| s.chars().next()).unwrap();
                    }
                    //left_number = first_word;
                }
                (None, None) => {
                    println!("Nothing found");
                }
                
            }

            match (last_digit_position,last_word_position) {
                (Some(last_digit_position),Some(last_word_position)) => {
                    if last_digit_position > last_word_position {
                        println!("Last is a digit: {:?}", last_digit);
                    } else {
                        println!("Last is a word: {:?}",last_word);
                    }
                }
                (Some(last_digit_position),None) => {
                    println!("Only digit exists");
                }
                (None,Some(last_word_position)) => {
                    println!("Only words exists");
                }
                (None, None) => {
                    println!("Nothing found");
                }

            }
            println!("");

            println!("Left number = {:?}",left_number?.to_digit(10));
            left_number?.to_digit(10)

            //Some(last_digit.to_digit(10)? + first_digit.to_digit(10)? * 10)
        })
        .sum()

}

// fn replace_line_and_calculate_sum(input: &String) -> u32 {
//     let number_map = HashMap::from([
//         ("one","1"),
//         ("two","2"),
//         ("three","3"),
//         ("four","4"),
//         ("five","5"),
//         ("six","6"),
//         ("seven","7"),
//         ("eight","8"),
//         ("nine","9"),
//     ]);
//     input
//         .lines()
//         .filter_map(|line| {
//             let mut modified_line = line.to_string();

//             println!("Before finding first {}",line);
//             if let Some(first_key) = find_first_occurrence(input, &number_map){
//                 println!("First key is {}, value is {:?}",first_key,number_map.get(&first_key));
//                 if let Some(replacement_value) = number_map.get(first_key) {
//                     println!("modifying {} with {}->{:?}",modified_line,first_key,replacement_value);
//                     modified_line = modified_line.replace(first_key, replacement_value);
//                     println!("inside loop replacing {}",modified_line);
//                 }
//             }
//             println!("After finding first {}",modified_line);

//             let first_digit = modified_line.chars().find(|c| c.is_ascii_digit())?;
//             let last_digit = modified_line.chars().rev().find(|c| c.is_ascii_digit())?;

//             println!("{} {}", first_digit,last_digit);

//             Some(last_digit.to_digit(10)? + first_digit.to_digit(10)? * 10)
//         })
//         .sum()
// }

fn main() {
    let input = fs::read_to_string("input.txt").expect("input.txt cannot be opened");

    println!("Part one sum: {}",calculate_line_sum(&input));


    //let input = String::from("one2xxx3four5two");
    //println!("Part two sum: {}",calculate_line_sum_part2(&input));
    let input = fs::read_to_string("two1nine.txt").expect("input.txt cannot be opened");
    //println!("Part two sum with replacing: {}",replace_line_and_calculate_sum(&input));
    println!("Part two sum without replacing = {}",calculate_line_sum_with_words(&input));
    
}
