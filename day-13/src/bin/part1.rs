fn main() {
    println!("{}", part1(include_str!("./input.txt")));
}

fn part1(input:&str) -> usize {
    let mut sum = 0;

    for map in input.split("\n\n") {
        sum += mirror_value(map);
    }

    sum
}

fn mirror_value(map:&str) -> usize {
    println!("{map}\n");
    let lines = Vec::from_iter(map.lines());

    for i in 1..lines.len() {
        let mut mirror = true;

        for j in 0..lines.len()-i {
            if (i as isize - j as isize - 1) < 0 { 
                continue; 
            }
            if lines[i+j] != lines[i-j-1] {
                mirror = false;
                break;
            }
        }

        if mirror {
            return 100 * i;
        }
    }

    let mut rotated = String::new();

    for i in 0..lines[0].len() {
        for j in 0..lines.len() {
            rotated += &lines[j].chars().nth(i).unwrap().to_string();
        }
        rotated += "\n";
    }

    let lines = Vec::from_iter(rotated.lines());

    for i in 1..lines.len() {
        let mut mirror = true;

        for j in 0..lines.len()-i {
            if (i as isize - j as isize - 1) < 0 { 
                continue; 
            }
            if lines[i+j] != lines[i-j-1] {
                mirror = false;
                break;
            }
        }

        if mirror {
            return i;
        }
    }

    panic!("No reflection found");
}

#[cfg(test)]
mod tests {
    use crate::part1;

    #[test]
    fn example() {
        assert_eq!(part1(
r#"#.##..##.
..#.##.#.
##......#
##......#
..#.##.#.
..##..##.
#.#.##.#.

#...##..#
#....#..#
..##..###
#####.##.
#####.##.
..##..###
#....#..#"#), 405);
    }
}