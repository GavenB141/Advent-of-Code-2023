use std::cmp::{min, max};

fn main() {
    println!("{}", part2(include_str!("./input.txt"), 1_000_000));
}

type Point = (usize, usize);

fn part2(input:&str, spacing:usize) -> usize {
    let map = parse_map(input);

    let mut galaxies:Vec<Point> = Vec::new();

    for y in 0..map.len() {
        for x in 0..map[y].len() {
            print!("{}", map[y][x]);
            if map[y][x] == '#' {
                galaxies.push((x, y));
            }
        }
        println!("");
    }

    let mut total = 0;

    for a in 0..galaxies.len() {
        for b in 0..galaxies.len() {
            if a == b {
                continue;
            }

            total += spaces_between(galaxies[a], galaxies[b], &map, spacing);
        }
    }
    total /= 2;

    total
}

fn spaces_between(a:Point, b:Point, map:&Vec<Vec<char>>, spacing:usize) -> usize {
    let mut total = 0;

    for y in min(a.1, b.1)+1..=max(a.1, b.1) {
        total += if map[y][a.0] == 'H' {
            spacing
        } else { 1 };
    }

    for x in min(a.0, b.0)+1..=max(a.0, b.0) {
        total += if map[a.1][x] == 'V' {
            spacing
        } else { 1 };
    }

    total
}

fn parse_map(input:&str) -> Vec<Vec<char>> {
    let mut output = Vec::new();

    for line in input.split('\n') {
        if !line.contains('#') {
            output.push(Vec::from_iter(line.chars().map(|_| 'H')));
        } else {
            output.push(Vec::from_iter(line.chars()));
        }
    }

    let mut i = 0;
    while i < output[0].len() {
        let mut clear = true;
        for line in &output {
            if line[i] == '#' {
                clear = false;
                break;
            }
        }

        if clear {
            for j in 0..output.len() {
                output[j][i] = if output[j][i] == 'H' { 'B' } else { 'V' };
            }
        }

        i += 1;
    }

    output
}

#[cfg(test)]
mod tests {
    use crate::part2;

    #[test]
    fn example() {
        assert_eq!(part2(
r#"...#......
.......#..
#.........
..........
......#...
.#........
.........#
..........
.......#..
#...#....."#, 10), 1030);

        assert_eq!(part2(
r#"...#......
.......#..
#.........
..........
......#...
.#........
.........#
..........
.......#..
#...#....."#, 100), 8410);
    }
}