fn main() {
    println!("{}", part1(include_str!("./input.txt")));
}

fn part1(input:&str) -> usize {
    let mut total = 0;

    for line in input.split('\n') {
        let data = Vec::from_iter(line.split_once(':').unwrap().1.split('|'));

        let mut winners:Vec<&str> = Vec::new();

        for winner in data[0].split_whitespace() {
            winners.push(winner);
        }

        let mut points = 0;

        for number in data[1].split_whitespace() {
            if winners.contains(&number) {
                if points == 0 {
                    points = 1;
                } else {
                    points *= 2;
                }
            }
        }

        total += points;
    }

    total
}

#[cfg(test)]
mod tests {
    use crate::part1;

    #[test]
    fn example() {
        assert_eq!(part1(
r#"Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11"#), 13)
    }
}