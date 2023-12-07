fn main() {
    println!("{}", part2(include_str!("./input.txt")));
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
                'J' => { 1 },
                'Q' => { 11 },
                'K' => { 12 },
                'A' => { 13 },
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
        let mut effective = self.cards.clone();
        let mode = self.most_frequent_card();

        for i in 0..effective.len() {
            if effective[i] == 1 {
                effective[i] = mode;
            }
        }

        for card in &effective {
            let matches = effective.iter().filter(|c| *c == card).count();

            match matches{
                5 => { return 6; },
                4 => { return 5; },
                3 => { match3 = true },
                2 => {if !pairs.contains(card) {
                    pairs.push(*card);
                }}
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

    fn most_frequent_card(&self) -> usize {
        let mut most_count = 0;
        let mut most_frequent = 1;

        for card in &self.cards {
            let matches = self.cards.iter().filter(|c| *c == card).count();

            if matches > most_count && *card != 1 {
                most_count = matches;
                most_frequent = *card;
            }
        }

        most_frequent
    }
}

fn part2(input:&str) -> usize {
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
    use crate::part2;

    #[test]
    fn example() {
        assert_eq!(part2(
r#"32T3K 765
T55J5 684
KK677 28
KTJJT 220
QQQJA 483"#), 5905)
    }
}