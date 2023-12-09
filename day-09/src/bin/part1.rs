fn main() {
    println!("{}", part1(include_str!("./input.txt")));
}

fn part1(input:&str) -> isize {
    let mut sum = 0;

    for line in input.split('\n') {
        sum += extrapolate(&Vec::from_iter(line.split_whitespace().map(|n|n.parse::<isize>().unwrap())));
    }

    sum
}

fn extrapolate(line:&Vec<isize>) -> isize {
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
    diff_map[last_index].push(0);

    for i in (0..=diff_map.len()-2).rev() {
        let val = diff_map[i].last().unwrap() + diff_map[i+1].last().unwrap();
        diff_map[i].push(val);
    }

    *diff_map[0].last().unwrap()
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
    use crate::part1;

    #[test]
    fn example() {
        assert_eq!(part1(
r#"0 3 6 9 12 15
1 3 6 10 15 21
10 13 16 21 30 45"#), 114);
    }
}