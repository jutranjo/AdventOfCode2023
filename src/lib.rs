struct Hand {
    cards: String,
    bid: usize,
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
    let hand1_value: usize = get_hand_value(&a);
    let hand2_value: usize = get_hand_value(&b);

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

fn get_hand_value(hand: &Hand) -> usize {
    //ideja:
    //5. mesto je vredno get_card_value(5. simbola)
    //4. mesto je vredno 8*get_card_value(4. simbola)
    //3. mesto je 8*8
    //2. mesto je 8^3
    //1. mesto je 8^4*get

    //pristeje se potem kateri match je, 5-of a kind itd. one-pair mora vrniti vec kot 8^4*14, visji so samo veckratniki tega
    2
}

fn load_input_into_hands(input: &str) -> Vec<Hand> {
    for line in input.lines().into_iter() {
        println!("{}",line);
        let split_line = line.split_ascii_whitespace().collect::<Vec<&str>>();
        let card_string: &str = split_line[0];
        let card_value: usize = split_line[1].parse().expect("couldnt read bid number!");

        println!("string: {:?}, value: {}",card_string,card_value);
    }
    vec![]
}

pub fn solve_part1(input: &str) -> usize {

    let hands = load_input_into_hands(&input);
    
    3
}