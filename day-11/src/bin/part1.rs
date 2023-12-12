fn main() {
    println!("{}", part1(include_str!("./input.txt")));
}

type Point = (usize, usize);

fn part1(input:&str) -> usize {
    let map = parse_map(input);

    let mut galaxies:Vec<Point> = Vec::new();

    for y in 0..map.len() {
        for x in 0..map[y].len() {
            if map[y][x] == '#' {
                galaxies.push((y, x));
            }
        }
    }

    let mut total = 0;

    for a in 0..galaxies.len() {
        for b in 0..galaxies.len() {
            if a == b {
                continue;
            }

            total += spaces_between(galaxies[a], galaxies[b]);
        }
    }
    total /= 2;

    total
}

fn spaces_between(a:Point, b:Point) -> usize {
    (b.0 as isize - a.0 as isize).abs() as usize +
    (b.1 as isize - a.1 as isize).abs() as usize
}

fn parse_map(input:&str) -> Vec<Vec<char>> {
    let mut output = Vec::new();

    for line in input.split('\n') {
        if !line.contains('#') {
            output.push(Vec::from_iter(line.chars()));
        }
        output.push(Vec::from_iter(line.chars()));
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
                output[j].insert(i, '.');
            }
            i += 1;
        }

        i += 1;
    }

    output
}

#[cfg(test)]
mod tests {
    use crate::part1;

    #[test]
    fn example() {
        assert_eq!(part1(
r#"...#......
.......#..
#.........
..........
......#...
.#........
.........#
..........
.......#..
#...#....."#), 374);
    }
}