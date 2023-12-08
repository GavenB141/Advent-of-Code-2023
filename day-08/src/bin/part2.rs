use std::collections::HashMap;

fn main() {
    println!("{}", part2(include_str!("./input.txt")));
}



fn part2(input:&str) -> usize {
    let fixed = input.replace('(', "").replace(')', "");
    let (instructions, map) = fixed.split_once('\n').unwrap();
    let instructions = Vec::from_iter(instructions.chars());
    let map = generate_map(map);

    let mut locations:Vec<&str> = Vec::new();

    for key in map.keys() {
        if key.ends_with('A') {
            locations.push(key);
        }
    }
    
    let mut steps:Vec<usize> = Vec::new();
    for i in 0..locations.len() {
        steps.push(0);

        while !locations[i].ends_with('Z') {
            let dir = instructions[steps[i] % instructions.len()];
            match dir {
                'L' => { 
                    locations[i] = map.get(locations[i]).unwrap().0;
                },
                'R' => { 
                    locations[i] = map.get(locations[i]).unwrap().1;
                },
                _ => {}
            }
            steps[i] += 1;
        }
    }
    
    let mut ans = steps[0];

    for i in 1..steps.len() {
        ans = ans * steps[i] / gcd(ans, steps[i]);
    }

    ans
}

fn gcd(first: usize, second: usize) -> usize {
    let mut max = first;
    let mut min = second;
    if min > max {
        let val = max;
        max = min;
        min = val;
    }

    loop {
        let res = max % min;
        if res == 0 {
            return min;
        }

        max = min;
        min = res;
    }
}					

fn generate_map(input:&str) -> HashMap<&str, (&str, &str)> {
    let mut map = HashMap::new();

    for line in input.split('\n') {
        let split = line
            .split_once("=");

        if split.is_none() {
            continue;
        }

        let (location, dir) = split.unwrap();

        let (left, right) = dir
            .split_once(',')
            .unwrap();

        map.insert(location.trim(), (left.trim(), right.trim()));
    }

    map
}

#[cfg(test)]
mod tests {
    use crate::part2;

    #[test]
    fn example() {
        assert_eq!(part2(
r#"LR

11A = (11B, XXX)
11B = (XXX, 11Z)
11Z = (11B, XXX)
22A = (22B, XXX)
22B = (22C, 22C)
22C = (22Z, 22Z)
22Z = (22B, 22B)
XXX = (XXX, XXX)"#), 6)
    }
}