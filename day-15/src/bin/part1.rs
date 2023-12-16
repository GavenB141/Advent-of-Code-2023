#![feature(ascii_char)]

fn main() {
    println!("{}", part1(include_str!("./input.txt")));
}

fn part1(input:&str) -> usize {
    let mut sum = 0;

    for step in input.split(',') {
        sum += hash(step);
    }
    
    sum
}

fn hash(step:&str) -> usize {
    let mut current:usize = 0;

    for c in step.chars() {
        current += c.as_ascii().unwrap().to_u8() as usize;
        current *= 17;
        current %= 256;
    }

    current
}

#[cfg(test)]
mod tests {
    use crate::part1;

    #[test]
    fn example() {
        assert_eq!(part1(
r#"rn=1,cm-,qp=3,cm=2,qp-,pc=4,ot=9,ab=5,pc-,pc=6,ot=7"#), 1320);
    }
}