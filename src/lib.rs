use core::panic;
use std::collections::HashMap;
#[derive(Debug)]
enum HandType {
    HighCard,
    OnePair,
    TwoPair,
    ThreeOfAKind,
    FullHouse,
    FourOfAKind,
    FiveOfAKind,
}

#[derive(Debug)]
struct Hand {
    cards: String,
    bid: usize,
    value: usize,
    handtype: HandType,
}

impl PartialEq for Hand {
    fn eq(&self, other: &Self) -> bool {
        compare_hands(self,other) == std::cmp::Ordering::Equal
    }
}

impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(compare_hands(self, other))
    }
}

fn compare_hands(a: &Hand, b: &Hand) -> std::cmp::Ordering {
    let hand1_value: usize = get_hand_value(&a.cards);
    let hand2_value: usize = get_hand_value(&b.cards);

    hand1_value.cmp(&hand2_value)
}

fn get_card_value(card: &char) -> usize {
    match card {
        'A' => 14,
        'K' => 13,
        'Q' => 12,
        'J' => 11,
        'T' => 10,
        _ => card.to_digit(10).unwrap_or(0) as usize, 
    }
}

fn get_hand_type(cards: &str) -> HandType {
    let mut card_count = HashMap::new();

    for card in cards.chars() {
        let counter = card_count.entry(card).or_insert(0);
        *counter += 1;
    }

    let mut hand_rank_vector: Vec<_> = card_count.values().cloned().collect();
    hand_rank_vector.sort_by(|a, b| b.cmp(a));
    println!("hand rank: {:?}",hand_rank_vector);
    
    if let Some(high_match) = hand_rank_vector.get(0) {
        match high_match{
            5 => HandType::FiveOfAKind,
            4 => HandType::FourOfAKind,
            3 => { if let Some(second_match) = hand_rank_vector.get(1) {
                            match (high_match,second_match) {
                                (3,2) => HandType::FullHouse,
                                (3,1) => HandType::ThreeOfAKind,
                                (2,2) => HandType::TwoPair,
                                (2,1) => HandType::OnePair,
                                _ => HandType::HighCard,
                            }
                        } else {
                            HandType::HighCard
                        }
                    }
            _ => HandType::HighCard,
            }
        }
    
        else {
            panic!("what is this hand {:?}", &cards);
        }

}

fn get_hand_type_value(handtype: HandType) -> usize {
    match handtype {
        HandType::HighCard => 1,
        HandType::OnePair => 2,
        HandType::TwoPair => 3,
        HandType::ThreeOfAKind => 4,
        HandType::FullHouse => 5,
        HandType::FourOfAKind => 6,
        HandType::FiveOfAKind => 7,
    }
}

fn get_hand_value(cards: &str) -> usize {
    let mut hand_value = 0;
    //ideja:
    //5. mesto je vredno get_card_value(5. simbola)
    hand_value+=get_card_value(&cards.chars().nth(4).unwrap_or('0'));
    //4. mesto je vredno 8*get_card_value(4. simbola)
    hand_value+=8*get_card_value(&cards.chars().nth(3).unwrap_or('0'));
    //3. mesto je 8*8
    hand_value+=8*8*get_card_value(&cards.chars().nth(2).unwrap_or('0'));
    //2. mesto je 8^3
    hand_value+=8*8*8*get_card_value(&cards.chars().nth(1).unwrap_or('0'));
    //1. mesto je 8^4*get
    hand_value+=8*8*8*8*get_card_value(&cards.chars().nth(0).unwrap_or('0'));

    //pristeje se potem kateri match je, 5-of a kind itd. one-pair mora vrniti vec kot 8^4*14, visji so samo veckratniki tega
    hand_value+=get_hand_type_value(get_hand_type(cards))*8*8*8*8*8*2;

    hand_value
}

fn load_input_into_hands(input: &str) -> Vec<Hand> {
    let mut output: Vec<Hand> = vec![];
    for line in input.lines().into_iter() {
        println!("{}",line);
        let split_line = line.split_ascii_whitespace().collect::<Vec<&str>>();
        let card_string: &str = split_line[0];
        let card_value: usize = split_line[1].parse().expect("couldnt read bid number!");

        println!("string: {:?}, value: {}",card_string,card_value);
        output.push(Hand { cards: card_string.to_string(), bid: card_value , value : 0, handtype: get_hand_type(&card_string)})
    }
    output
}

pub fn solve_part1(input: &str) -> usize {

    let mut hands = load_input_into_hands(&input);
    println!("{:?}",hands);
    for hand in &mut hands {
        hand.value = get_hand_value(&hand.cards);
        println!("hand {:?}, \t\t hand value: {}, handtype: {:?}",hand,hand.value,hand.handtype);
    }

    hands.sort_by(|a, b| a.value.cmp(&b.value));

    let mut sum = 0;
    for (rank, hand) in hands.iter().enumerate() {
        println!("rank {}, {}, value {}",rank+1, hand.cards,hand.value);
        sum += (rank+1)*hand.bid;
    }
    
    
    sum
}