fn main() {
    println!("{}", part1(include_str!("./input.txt")));
}

fn part1(input:&str) -> u32  {
    let mut current_id = 0;
    let mut sum = 0;

    for line in input.split('\n') {
        current_id += 1;
        let mut possible = true;

        for set in line.replace(format!("Game {current_id}: ").as_str(), "").split(';') {
            let mut rgb = (0, 0, 0);
            for color in set.split(',') {
                let data = color.trim().split(' ').collect::<Vec<&str>>();

                match data[1] {
                    "red" => { rgb.0 += data[0].parse::<u32>().expect("a valid integer"); },
                    "green" => { rgb.1 += data[0].parse::<u32>().expect("a valid integer"); },
                    "blue" => { rgb.2 += data[0].parse::<u32>().expect("a valid integer"); },
                    _ => ()
                }
            }

            if rgb.0 > 12 || rgb.1 > 13 || rgb.2 > 14 {
                possible = false;
            }
        }

        if possible {
            sum += current_id;
        }
    }

    return sum;
}

#[cfg(test)]
mod tests {
    use crate::part1;

    #[test]
    fn example() {
        assert_eq!(part1(
r#"Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green"#), 8)
    }
}