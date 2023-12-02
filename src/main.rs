use std::{fs, collections::{HashMap, btree_map::Values}};

fn extract_id(input: &str) -> i32 {
    let parts: Vec<&str> = input.split_whitespace().collect();
    
    match parts[1].parse::<i32>() {
        Ok(parsed_number) => {
            parsed_number
        }
        Err(err) => {
            println!("Failed to parse game ID, error: {}",err);
            0
        }
    }
}

fn split_draw_into_two(draw: &str) -> (&str,i32) {
    let draw_string_parts = draw.split_ascii_whitespace().collect::<Vec<&str>>();

    let draw_number = draw_string_parts[0].parse::<i32>().unwrap_or(0); //maybe need to handle this better
    let draw_colour = draw_string_parts[1];
    (draw_colour,draw_number)
}

fn check_draw_for_cheating(draw: &str, limits: &HashMap<&str, i32>) -> bool {
    let mut cheating = false;

    let (draw_colour,draw_number) = split_draw_into_two(draw);

    if let Some(&limit) = limits.get(draw_colour) {
        if limit < draw_number {
            //println!("Elf is cheating limit={} < drawn={}",limit, draw_number);
            cheating = true;
        }
        else {
            //println!("We're within limits! limit={} > drawn={}", limit, draw_number);
        }
    }
    
    cheating
}

fn multiply_minimum_cube_numbers(input: &str) -> i32 {
    input
        .lines()
        .map(|line| {
            //println!("Looking at line: {}",line);
            let mut minimum_cube_counts: HashMap<&str, i32> = HashMap::from([
                ("red",0),
                ("green",0),
                ("blue",0),
            ]);

            let game_rounds = line.split(':').collect::<Vec<&str>>()[1]
                                    .split(';').map(|item| item.trim());
            for round in game_rounds {
                //println!("round = {:?}",round);
                let draws = round.split(',');
                for draw in draws {
                    let (draw_colour, draw_number) = split_draw_into_two(draw.trim());
                    if let Some(&cube_count) = minimum_cube_counts.get(draw_colour) {
                        if draw_number > cube_count {
                            //println!("updating cube count for {}! {} -> {}",draw_colour,cube_count,draw_number);
                            minimum_cube_counts.insert(draw_colour, draw_number);
                        }
                    }
                }
            }
            minimum_cube_counts.iter().map(|(_,&value)| value).fold(1, |acc, x| acc * x)
        }).sum()
}

fn sum_non_cheating_ids(input: &str, limits_map: &HashMap<&str, i32>) -> i32 {
    input
        .lines()
        .map(|line| {
            //println!("{}",line);
            let parts = line.split(':').collect::<Vec<&str>>();
            let game_id = extract_id(parts[0]);
            let mut cheating_detected = false;

            let rounds = parts[1].split(';');
            for round in rounds {
                //println!("Round is {}", round.trim());
                let draws = round.split(',');
                for draw in draws {
                    let cheating = check_draw_for_cheating(draw.trim(), &limits_map);
                    //println!("cheating = {}",cheating);
                    if cheating {
                        cheating_detected = true;
                    }
                }
            }
            
            if !cheating_detected {
                //println!("no cheating, adding {}",game_id);
                game_id
            } else {
                //println!("elf cheated, he gets 0");
                0
            }
        })
        .sum()
}

fn main() {
    let input = fs::read_to_string("input.txt").expect("input.txt cannot be opened");
    let example2 = fs::read_to_string("example2.txt").expect("input.txt cannot be opened");

    let limits_map: HashMap<&str, i32> = HashMap::from([
        ("red",12),
        ("green",13),
        ("blue",14),
    ]);

    println!("id sum for  part 1 = {}",sum_non_cheating_ids(&input,&limits_map));

    println!("cube count thing for part 2 example = {}, should be 2286",multiply_minimum_cube_numbers(&example2));

    println!("cube count for part 2 = {}", multiply_minimum_cube_numbers(&input));
}
