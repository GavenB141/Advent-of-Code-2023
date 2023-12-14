fn main() {
    println!("{}", part1(include_str!("./input.txt")));
}

fn part1(input:&str) -> usize {
    let mut sum = 0;

    for map in input.split("\n\n") {
        let val = mirror_value(map);
        sum += val;
    }

    sum
}

fn mirror_value(map:&str) -> usize {
    let lines = Vec::from_iter(map.lines());

    fn find_reflection(lines:&Vec<&str>) -> Option<usize> {
        for i in 1..lines.len() {
            let mut mirror = true;
            let mut smudge_found = false;

            for j in 0..lines.len()-i {
                if (i as isize - j as isize - 1) < 0 { 
                    continue; 
                }
                if lines[i+j] != lines[i-j-1] {
                    if smudge_found {
                        mirror = false;
                        break;
                    }

                    if one_different_char(lines[i+j], lines[i-j-1]) {
                        smudge_found = true;
                    } else {
                        break;
                    }
                }
            }

            if mirror && smudge_found {
                return Some(i);
            }
        }

        None
    }

    if let Some(line) = find_reflection(&lines) {
        return 100 * line;
    }

    let mut rotated = String::new();

    for i in 0..lines[0].len() {
        for j in 0..lines.len() {
            rotated += &lines[j].chars().nth(i).unwrap().to_string();
        }
        rotated += "\n";
    }

    let lines = Vec::from_iter(rotated.lines());

    if let Some(line) = find_reflection(&lines) {
        return line;
    }

    panic!("No reflection found");
}

fn one_different_char(a:&str, b:&str) -> bool {
    let mut out = false;

    for i in 0..a.len() {
        if a.chars().nth(i).unwrap() != b.chars().nth(i).unwrap() {
            if out {
                return false;
            }

            out = true;
        }
    }

    out
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
#....#..#"#), 400);
    }
}