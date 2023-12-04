use std::cmp::{min, max};

fn main() {
    println!("{}", part2(include_str!("./input.txt")));
}

fn part2(input:&str) -> usize  {
    let lines = Vec::from_iter(input.split('\n'));
    let mut map:Vec<Vec<char>> = Vec::new();

    for line in lines {
        map.push(Vec::from_iter(line.chars().into_iter()));
    }

    fn number_at(mut x:usize, y:usize, map: &Vec<Vec<char>>) -> Option<usize> {
        if !map[y][x].is_ascii_digit()  {
            return None;
        }

        while x != 0 && map[y][x - 1].is_ascii_digit() {
            x -= 1;
        }

        let mut value = 0;

        while x < map[y].len() && map[y][x].is_ascii_digit() {
            value *= 10;
            value += map[y][x].to_digit(10).unwrap() as usize;
            x += 1;
        }

        Some(value)
    }

    let mut sum = 0;

    for y in 0..map.len() {
        for x in 0..map[y].len() {
            if map[y][x] == '*' {
                let mut neighbors:Vec<usize> = Vec::new();

                for i in max(y-1,0)..=min(y+1, map.len()-1) {
                    let mut fresh = true;
                    for j in max(x-1,0)..=min(x+1, map[i].len()-1) {
                        if let Some(num) = number_at(j, i, &map) {
                            if fresh {
                                neighbors.push(num);
                                fresh = false;
                            }
                        } else {
                            fresh = true;
                        }
                    }
                }

                if neighbors.len() == 2 {
                    sum += neighbors[0] * neighbors[1];
                }
            }
        }
    }

    sum
}

#[cfg(test)]
mod tests {
    use crate::part2;

    #[test]
    fn example() {
        assert_eq!(part2(
r#"
467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598.."#), 467835)
    }
}