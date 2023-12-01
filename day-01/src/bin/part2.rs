fn main() {
    println!("{}", part2(include_str!("./input.txt")));
}

fn part2(input: &str) -> u32 {
    let mut sum = 0;

    let number_words:Vec<(&str, char)> = vec![
        ("one", '1'),
        ("two", '2'),
        ("three", '3'),
        ("four", '4'),
        ("five", '5'),
        ("six", '6'),
        ("seven", '7'),
        ("eight", '8'),
        ("nine", '9')
    ];

    for line in input.split('\n').into_iter() {
        let mut fixed = String::from(line);

        for num in &number_words {
            for index in line.match_indices(num.0) {
                fixed.remove(index.0);
                fixed.insert(index.0, num.1);
            }
        }

        let mut digits:Vec<char> = Vec::new();
        for digit in fixed.chars() {
            if digit.is_ascii_digit() {
                digits.push(digit);
            }
        }

        let mut extracted = String::new();
        extracted += format!("{}{}", digits[0].to_string(), &digits.last().unwrap().to_string()).as_str();

        sum += extracted.parse::<u32>().expect("Valid integer");
    }

    return sum;
}

#[cfg(test)]
mod tests {
    use crate::part2;

    #[test]
    fn example() {
        assert_eq!(part2(include_str!("./example2.txt")), 281)
    }
}