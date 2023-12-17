fn main() {
    println!("{}", part1(include_str!("./input.txt")));
}

fn part1(input:&str) -> usize {
    let mut map = Vec::new();

    for line in input.lines() {
        map.push(Vec::from_iter(line.chars()));
    }

    let mut max = 0;

    let ranges = [
        ('>', 0..1, 0..map.len()),
        ('<', map[0].len()-1..map[0].len(), 0..map.len()),
        ('^', 0..map[0].len(), map.len()-1..map.len()),
        ('v', (0..map[0].len()), 0..1),
    ];

    for range in ranges {
        for y in range.2 {
            for x in range.1.clone() {
                let mut energized = Vec::new();

                for line in input.lines() {            
                    let mut empty = Vec::new();
                    for _ in 0..line.len() {
                        empty.push(Vec::new());
                    }
                    energized.push(empty);
                }

                energize(&map, (x as isize ,y as isize), range.0, &mut energized);

                let mut sum = 0;

                for line in energized {
                    for tile in line {
                        if tile.len() > 0 {
                            sum += 1;
                        }
                    }
                }
                
                if sum > max {
                    max = sum;
                }
            }
        }
    }

    max
}

fn energize(map:&Vec<Vec<char>>, mut pos:(isize, isize), dir:char, acc:&mut Vec<Vec<Vec<char>>>) {
    let mut effective = false;

    while !((pos.0 < 0 || pos.1 < 0) || (pos.1 as usize >= map.len() || pos.0 as usize >= map[pos.1 as usize].len())) && map[pos.1 as usize][pos.0 as usize] == '.' {
        if !acc[pos.1 as usize][pos.0 as usize].contains(&dir) {
            acc[pos.1 as usize][pos.0 as usize].push(dir);
            effective = true;
        }

        match dir {
            '<' => { pos.0 -= 1 }
            '>' => { pos.0 += 1 }
            '^' => { pos.1 -= 1 }
            'v' => { pos.1 += 1 }
            _ => {}
        }
    }

    if (pos.0 < 0 || pos.1 < 0) || (pos.1 as usize >= map.len() || pos.0 as usize >= map[pos.1 as usize].len()) {
        return;
    }

    if !acc[pos.1 as usize][pos.0 as usize].contains(&dir) {
        acc[pos.1 as usize][pos.0 as usize].push(dir);
        effective = true;
    }

    if !effective {
        return;
    }

    match map[pos.1 as usize][pos.0 as usize] {
        '/' => {
            match dir {
                '>' => { energize(map, (pos.0, pos.1-1), '^', acc); }
                '<' => { energize(map, (pos.0, pos.1+1), 'v', acc); }
                'v' => { energize(map, (pos.0-1, pos.1), '<', acc); }
                '^' => { energize(map, (pos.0+1, pos.1), '>', acc); }
                _ => {}
            }
        }

        '\\' => {
            match dir {
                '>' => { energize(map, (pos.0, pos.1+1), 'v', acc); }
                '<' => { energize(map, (pos.0, pos.1-1), '^', acc); }
                'v' => { energize(map, (pos.0+1, pos.1), '>', acc); }
                '^' => { energize(map, (pos.0-1, pos.1), '<', acc); }
                _ => {}
            }
        }

        '|' => {
            if dir == 'v' || dir == '>' || dir == '<' {
                energize(map, (pos.0, pos.1+1), 'v', acc);
            } 
            
            if dir == '^' || dir == '>' || dir == '<' {
                energize(map, (pos.0, pos.1-1), '^', acc);
            }
        }

        '-' => {
            if dir == '^' || dir == 'v' || dir == '<' {
                energize(map, (pos.0-1, pos.1), '<', acc);
            }

            if dir == '^' || dir == 'v' || dir == '>' {
                energize(map, (pos.0+1, pos.1), '>', acc);
            }
        }

        _ => {}
    }
}

#[cfg(test)]
mod tests {
    use crate::part1;

    #[test]
    fn example() {
        assert_eq!(part1(
r#".|...\....
|.-.\.....
.....|-...
........|.
..........
.........\
..../.\\..
.-.-/..|..
.|....-|.\
..//.|...."#), 51);
    }
}