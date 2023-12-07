fn main() {
    println!("{}", part1(include_str!("./input.txt")));
}

struct Hand {
    cards:Vec<usize>,
    bid:usize
}

impl Hand {
    fn new(input_line:&str) -> Self {
        let (hand, bid) = input_line.split_once(" ").unwrap();

        let mut numerical_hand:Vec<usize> = Vec::new();

        for char in hand.chars() {
            numerical_hand.push(match char {
                'T' => { 10 },
                'J' => { 11 },
                'Q' => { 12 },
                'K' => { 13 },
                'A' => { 14 },
                _ => { char.to_string().parse::<usize>().unwrap() }
            });
        }

        Self {
            cards: numerical_hand,
            bid: bid.parse::<usize>().unwrap()
        }
    }

    fn hand_type(&self) -> usize {
        let mut match3 = false;
        let mut pairs:Vec<usize> = Vec::new();

        for card in &self.cards {
            match self.cards.iter().filter(|c| *c == card).count() {
                5 => { return 6; },
                4 => { return 5; },
                3 => { match3 = true },
                2 => { if !pairs.contains(card) {
                    pairs.push(*card) 
                }},
                _ => {}
            }
        }

        if match3 && pairs.len() == 1 {
            return 4;
        } else if match3 {
            return 3;
        }

        pairs.len()
    }
}

fn part1(input:&str) -> usize {
    let mut hands:Vec<Hand> = Vec::from_iter(input.split('\n').map(|i| Hand::new(i)));
    
    hands.sort_by(|a, b| {
        if a.hand_type() != b.hand_type() {
            return a.hand_type().partial_cmp(&b.hand_type()).unwrap();
        } else {
            for i in 0..a.cards.len() {
                if a.cards[i] != b.cards[i] {
                    return a.cards[i].partial_cmp(&b.cards[i]).unwrap();
                }
            }
        }

        std::cmp::Ordering::Equal
    });

    let mut sum = 0;

    for i in 0..hands.len() {
        sum += hands[i].bid * (i+1);
    }

    sum
}

#[cfg(test)]
mod tests {
    use crate::part1;

    #[test]
    fn example() {
        assert_eq!(part1(
r#"32T3K 765
T55J5 684
KK677 28
KTJJT 220
QQQJA 483"#), 6440)
    }
}