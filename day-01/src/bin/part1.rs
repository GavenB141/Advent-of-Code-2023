fn main() {
    println!("{}", part1(include_str!("./input.txt")));
}

fn part1(input: &str) -> u32 {
    let mut sum = 0;

    for line in input.split('\n').into_iter() {
        let mut digits:Vec<char> = Vec::new();

        for digit in line.chars() {
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
    use crate::part1;

    #[test]
    fn example() {
        assert_eq!(part1(include_str!("./example1.txt")), 142)
    }
}