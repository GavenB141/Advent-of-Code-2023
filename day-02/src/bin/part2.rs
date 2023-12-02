use std::cmp::max;

fn main() {
    println!("{}", part2(include_str!("./input.txt")));
}

fn part2(input:&str) -> u32  {
    let mut current_id = 0;
    let mut sum = 0;

    for line in input.split('\n') {
        current_id += 1;
        let mut rgb = (0, 0, 0);

        for set in line.replace(format!("Game {current_id}: ").as_str(), "").split(';') {
            for color in set.split(',') {
                let data = color.trim().split(' ').collect::<Vec<&str>>();
                let num = data[0].parse::<u32>().expect("a valid integer");
                match data[1] {
                    "red" => { rgb.0 = max(num, rgb.0); },
                    "green" => { rgb.1 = max(num, rgb.1); },
                    "blue" => { rgb.2 = max(num, rgb.2); },
                    _ => ()
                }
            }
        }

        sum += rgb.0 * rgb.1 * rgb.2;
    }

    return sum;
}

#[cfg(test)]
mod tests {
    use crate::part2;

    #[test]
    fn example() {
        assert_eq!(part2(
r#"Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green"#), 2286);
    }
}