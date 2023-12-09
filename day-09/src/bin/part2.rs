fn main() {
    println!("{}", part2(include_str!("./input.txt")));
}

fn part2(input:&str) -> isize {
    let mut sum = 0;

    for line in input.split('\n') {
        sum += extrapolate_reverse(&Vec::from_iter(line
            .split_whitespace()
            .map(|n|n.parse::<isize>()
            .unwrap())));
    }

    sum
}

fn extrapolate_reverse(line:&Vec<isize>) -> isize {
    let mut diff_map:Vec<Vec<isize>> = vec![line.clone()];

    fn bottom(line:&Vec<isize>) -> bool {
        for i in line {
            if *i != 0 {
                return false;
            }
        }

        true
    }

    while !bottom(diff_map.last().unwrap()) {
        diff_map.push(diffs(diff_map.last().unwrap()));
    }

    let last_index = diff_map.len()-1;
    diff_map[last_index].insert(0, 0);

    for i in (0..=diff_map.len()-2).rev() {
        let val = diff_map[i][0] - diff_map[i+1][0];
        diff_map[i].insert(0, val);
    }

    diff_map[0][0]
}

fn diffs(line:&Vec<isize>) -> Vec<isize> {
    let mut current = line[0];
    let mut new:Vec<isize> = Vec::new();

    for i in 1..line.len() {
        new.push(line[i] - current);
        current = line[i];
    }

    new
}

#[cfg(test)]
mod tests {
    use crate::part2;

    #[test]
    fn example() {
        assert_eq!(part2(
r#"0 3 6 9 12 15
1 3 6 10 15 21
10 13 16 21 30 45"#), 2);
    }
}