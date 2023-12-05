fn main() {
    println!("{}", part2(include_str!("./input.txt")));
}

fn part2(input:&str) -> usize {
    let cards = Vec::from_iter(input.split('\n'));

    let mut total_copies:Vec<usize> = Vec::new();
    while total_copies.len() < cards.len() {
        total_copies.push(1);
    }

    let mut i = 0; 
    while i < cards.len() {
        let (card_num, numbers) = cards[i].split_once(":").unwrap();
        let card_num = card_num.split_whitespace().nth(1).unwrap().parse::<usize>().unwrap();

        let (winners, numbers) = numbers.split_once('|').unwrap();
        let winners = Vec::from_iter(winners.split_whitespace());
        let numbers = Vec::from_iter(numbers.split_whitespace());

        for _ in 0..total_copies[card_num-1] {
            let mut copies = 0;
            for num in &numbers {
                if winners.contains(&num) {
                    copies += 1;
                }
            }

            for j in card_num..std::cmp::min(card_num+copies, cards.len()-1) {
                total_copies[j] += 1;
            }
        }

        i += 1;
    }

    let mut sum  = 0;
    for n in total_copies {
        sum += n;
    }
    sum
}

#[cfg(test)]
mod tests {
    use crate::part2;

    #[test]
    fn example() {
        assert_eq!(part2(
r#"Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11"#), 30)
    }
}