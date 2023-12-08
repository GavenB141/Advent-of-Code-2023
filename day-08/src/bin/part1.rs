fn main() {
    println!("{}", part1(include_str!("./input.txt")));
}



fn part1(input:&str) -> usize {
    let (instructions, map) = input.split_once('\n').unwrap();
    let instructions = Vec::from_iter(instructions.chars());
    let mut map = Vec::from_iter(map.split('\n'));
    map.remove(0);
    let map = Vec::from_iter(map.iter().map(|line| {parse_line(line)}));

    let mut steps = 0;
    let mut location = "AAA".to_string();

    while location.as_str() != "ZZZ" {
        for line in &map {
            if line.0 == location {
                match instructions[steps % instructions.len()] {
                    'L' => { location = line.1.clone() }
                    'R' => { location = line.2.clone() }
                    _ => {}
                }
                steps += 1;
                break;
            }
        }
    }
    

    steps
}

fn parse_line(input:&str) -> (String,String,String) {
    let fixed = input
        .replace('(', "")
        .replace(')', "")
        .replace(' ', "");

    let (location, dir) = fixed
        .split_once("=")
        .unwrap();

    let (left, right) = dir
        .split_once(',')
        .unwrap();

    (location.to_string(), left.to_string(), right.to_string())
}

#[cfg(test)]
mod tests {
    use crate::part1;

    #[test]
    fn example() {
        assert_eq!(part1(
r#"RL

AAA = (BBB, CCC)
BBB = (DDD, EEE)
CCC = (ZZZ, GGG)
DDD = (DDD, DDD)
EEE = (EEE, EEE)
GGG = (GGG, GGG)
ZZZ = (ZZZ, ZZZ)"#), 2);

        assert_eq!(part1(
r#"LLR

AAA = (BBB, BBB)
BBB = (AAA, ZZZ)
ZZZ = (ZZZ, ZZZ)"#), 6)
    }
}