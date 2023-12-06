fn main() {
    println!("{}", part2(include_str!("./input.txt")));
}

fn part2(input:&str) -> usize {
    let fixed = input.replace(" ", "");
    let (times, distances) = fixed.split_once('\n').unwrap();
    let times = Vec::from_iter(times.split_once(":").unwrap().1.split_whitespace().map(|a| {a.parse::<usize>().unwrap()}));
    let distances = Vec::from_iter(distances.split_once(":").unwrap().1.split_whitespace().map(|a| {a.parse::<usize>().unwrap()}));

    let mut possible:Vec<usize> = Vec::new();

    for i in 0..times.len() {
        possible.push(0);
        for hold_time in 0..=times[i] {
            let drive_time = times[i] - hold_time;
            if hold_time * drive_time > distances[i] {
                possible[i] += 1
            }
        }
    }

    let mut total = 1;
    for p in possible {
        total *= p;
    }

    total
}

#[cfg(test)]
mod tests {
    use crate::part2;

    #[test]
    fn example() {
        assert_eq!(part2(
r#"Time:      7  15   30
Distance:  9  40  200"#), 71503)
    }
}