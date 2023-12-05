use std::collections::HashSet;
use std::cmp;

#[derive(Debug)]
struct Card {
    id: usize,
    winning_numbers: HashSet<u32>,
    our_numbers: Vec<u32>,
}

fn get_value_from_matches(matches: u32) -> u32 {
    if matches == 0 {
        0
    } else {
        u32::pow(2,matches.checked_sub(1).unwrap_or(0))
    }
}

fn calculate_card_matches(card: Card) -> u32 {
    let mut card_matches: u32 = 0;
    
    for number in card.our_numbers {
        if card.winning_numbers.contains(&number) {
            card_matches += 1;
        }
    }

    card_matches
}

fn calculate_card_value(card: Card) -> Option<u32> {
    let card_matches = calculate_card_matches(card);
    let card_value = get_value_from_matches(card_matches);
    Some(card_value)
}

fn transform_line_into_card(line: &str) -> Option<Card> {
    let left_part = line.split(':').collect::<Vec<&str>>()[0].split_whitespace().collect::<Vec<&str>>();
    let right_part = line.split(':').collect::<Vec<&str>>()[1].split('|').collect::<Vec<&str>>();

    let winning_number_string = right_part[0].trim().split_whitespace().collect::<Vec<&str>>();
    let our_numbers_string = right_part[1].trim().split_whitespace().collect::<Vec<&str>>();

    let parsed_winning_numbers: Result<Vec<u32>, _> = winning_number_string
                                                                        .iter()
                                                                        .map(|&s| s.parse::<u32>())
                                                                        .collect();

    let parsed_our_numbers: Result<Vec<u32>, _> = our_numbers_string
                                                                .iter()
                                                                .map(|&s| s.parse::<u32>())
                                                                .collect();
    
    if let Ok(game_id) = left_part[1].parse::<usize>(){
        if let Ok(winning_numbers) = parsed_winning_numbers {
            if let Ok(our_numbers) = parsed_our_numbers {
                Some(Card {
                    id: game_id,
                    winning_numbers: winning_numbers.into_iter().collect(),
                    our_numbers,
                })
            } else {
                None
            }
        } else {
            None
        }        
    } else {
        None
    }
}

fn add_won_copies(id: usize, card_value: u32, number_of_copies: &mut Vec<u32>) -> () {

    let lower_bound = id+1;
    let upper_bound = id+ card_value as usize+1;

    let max_bound = number_of_copies.len()+1;

    let upper_bound = cmp::min(upper_bound,max_bound);

    //println!("lower b:{},upper b:{}",lower_bound,upper_bound);

    for id_to_update in lower_bound..upper_bound {
        number_of_copies[id_to_update-1] += number_of_copies[id-1];
    }
}

pub fn solve_part1(input: &str) -> u32 {

    //println!("{} \n",input);

    input
        .lines()
        .map(|line| {
            //println!("\n{}",line);
            if let Some(card) = transform_line_into_card(line) {
                //println!("{:?} \n id={}, winning numbers={:?}, numbers_we_have={:?}",card,card.id,card.winning_numbers,card.our_numbers);
                calculate_card_value(card)
            } else {
                None
            }
        })
        .map(Option::unwrap_or_default)
        .sum()

}

pub fn solve_part2(input: &str) -> u32 {
    let mut number_of_copies: Vec<u32> = vec![1;input.lines().count()];

    println!("len of copies: {:?}",number_of_copies.len());


    let _total_value: u32 = input
        .lines()
        .map(|line| {
            if let Some(card) = transform_line_into_card(line) {
                let card_id = card.id;
                let card_value = calculate_card_matches(card);
                //println!("id={}, value={}",card_id,card_value);
                add_won_copies(card_id,card_value,&mut number_of_copies);
                //println!("{:?}",number_of_copies);
                Some(card_value)
            } else {
                None
            }
        })
        .map(Option::unwrap_or_default)
        .sum();

    let total_value = number_of_copies.iter().sum();

    total_value
}

#[cfg(test)]
mod tests {
    use std::fs;

    use crate::calculate_card_matches;
    use crate::solve_part1;

    #[test]
    fn example_input() {
        let input = fs::read_to_string("example.txt").expect("example.txt cannot be opened");
        assert_eq!(solve_part1(&input),13);
    }

    use crate::add_won_copies;
    #[test]
    fn adding_copies() {
        let card_id = 2;
        let card_value = 2;
        let mut number_of_copies: Vec<u32> = vec![1,1,1,1,1];
        let expected_result: Vec<u32> = vec![1,1,2,2,1];

        add_won_copies(card_id,card_value,&mut number_of_copies);
        assert_eq!(number_of_copies,expected_result);
    }

    #[test]
    fn example_step1() {
        let card_id = 1;
        let card_value = 4;
        let mut number_of_copies: Vec<u32> = vec![1,1,1,1,1,1];
        let expected_result: Vec<u32> = vec![1,2,2,2,2,1];

        add_won_copies(card_id,card_value,&mut number_of_copies);
        assert_eq!(number_of_copies,expected_result);
    }

    
    #[test]
    fn example_step2() {
        let card_id = 2;
        let card_value = 2;
        let mut number_of_copies: Vec<u32> = vec![1,2,2,2,2,1];
        let expected_result: Vec<u32> = vec![1,2,4,4,2,1];

        add_won_copies(card_id,card_value,&mut number_of_copies);
        assert_eq!(number_of_copies,expected_result);
    }

    
    #[test]
    fn example_step3() {
        let card_id = 3;
        let card_value = 2;
        let mut number_of_copies: Vec<u32> = vec![1,2,4,4,2,1];
        let expected_result: Vec<u32> = vec![1,2,4,8,6,1];

        add_won_copies(card_id,card_value,&mut number_of_copies);
        assert_eq!(number_of_copies,expected_result);
    }

    
    #[test]
    fn example_step4() {
        let card_id = 4;
        let card_value = 1;
        let mut number_of_copies: Vec<u32> = vec![1,2,4,8,6,1];
        let expected_result: Vec<u32> = vec![1,2,4,8,14,1];

        add_won_copies(card_id,card_value,&mut number_of_copies);
        assert_eq!(number_of_copies,expected_result);
    }

    
    #[test]
    fn example_step5() {
        let card_id = 4;
        let card_value = 1;
        let mut number_of_copies: Vec<u32> = vec![1,2,4,8,6,1];
        let expected_result: Vec<u32> = vec![1,2,4,8,14,1];

        add_won_copies(card_id,card_value,&mut number_of_copies);
        assert_eq!(number_of_copies,expected_result);
    }

    
    #[test]
    fn adding_to_last() {
        let card_id = 5;
        let card_value = 1;
        let mut number_of_copies: Vec<u32> = vec![1,2,4,8,14,1];
        let expected_result: Vec<u32> = vec![1,2,4,8,14,15];

        add_won_copies(card_id,card_value,&mut number_of_copies);
        assert_eq!(number_of_copies,expected_result);
    }

    #[test]
    fn adding_from_second_to_last() {
        let card_id = 4;
        let card_value = 2;
        let mut number_of_copies: Vec<u32> = vec![1,2,4,8,14,1];
        let expected_result: Vec<u32> = vec![1,2,4,8,22,9];

        add_won_copies(card_id,card_value,&mut number_of_copies);
        assert_eq!(number_of_copies,expected_result);
    }

    use crate::transform_line_into_card;
    #[test]
    fn does_line_transform_to_card() {
        let line = "Card   7: 79 43 87 42  8 74 51 69  3 44 | 30 27 19 42 99 28 68 43  5 36 54 24 92 97 34 44 96  2 50 82 35 69 25 45 18";
        if let Some(card) = transform_line_into_card(line){
            assert_eq!(card.id, 7);
            let matches = calculate_card_matches(card);
            assert_eq!(matches, 4);
        } else {
            panic!("card isnt correctly parsed");
        }
    }
}