use std::{fs, collections::HashMap};

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

fn check_draw_for_cheating(draw: &str, limits: &HashMap<&str, u32>) -> bool {
    let mut cheating = false;

    let draw_string_parts = draw.split_ascii_whitespace().collect::<Vec<&str>>();

    let draw_number = draw_string_parts[0].parse::<u32>().unwrap_or(0); //maybe need to handle this better
    let draw_colour = draw_string_parts[1];

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

fn sum_non_cheating_ids(input: &str, limits_map: &HashMap<&str, u32>) -> i32 {
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

    let limits_map = HashMap::from([
        ("red",12),
        ("green",13),
        ("blue",14),
    ]);

    println!("id sum = {}",sum_non_cheating_ids(&input,&limits_map));

}
