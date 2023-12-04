fn main() {
    println!("{}", part1(include_str!("./input.txt")));
}

fn part1(input:&str) -> usize  {
    let lines = Vec::from_iter(input.split('\n'));
    let mut map:Vec<Vec<char>> = Vec::new();

    for line in lines {
        map.push(Vec::from_iter(line.chars().into_iter()));
    }

    /** Return the value of the number starting at the location (x, y)
        only if (x, y) is the first digit of a number and the number 
        has a symbol neighbor */
    fn value_of(x:usize, y:usize, map: &Vec<Vec<char>>) -> usize {
        if !map[y][x].is_ascii_digit() { return 0; }
        if x != 0 && map[y][x-1].is_ascii_digit() { return 0; }

        let mut neighbor = false;
        let mut value:usize = 0;

        let mut i = if x != 0 { x - 1 } else { x };
        let top = if y != 0 { y - 1 } else { y };
        let bottom = if y != map.len()-1 { y + 1 } else { y };
        let mut right = x;

        while i <= right {
            for j in top..=bottom {
                if map[j][i].is_ascii_digit() {
                    value *= 10;
                    value += map[j][i].to_digit(10).unwrap() as usize;
                    if right != map[y].len()-1 {
                        right += 1
                    }
                } else if map[j][i] != '.' {
                    neighbor = true;
                }
            }
            i += 1;
        }

        if neighbor {
            value
        } else {
            0
        }
    }

    let mut sum = 0;

    for y in 0..map.len() {
        for x in 0..map[y].len() {
            sum += value_of(x, y, &map);
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
r#"467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598.."#), 4361)
    }
}