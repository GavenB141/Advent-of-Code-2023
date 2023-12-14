fn main() {
    println!("{}", part2(include_str!("./input.txt")));
}

struct Platform {
    layout: Vec<Vec<char>>,
    cycle_history: Vec<String>
}

impl Platform {
    fn new(input:&str) -> Self {
        let mut layout = Vec::new();

        for line in input.lines() {
            layout.push(Vec::from_iter(line.chars()));
        }

        Self { 
            layout,
            cycle_history: vec![input.to_string()]
        }
    }

    fn to_string(&self) -> String {
        let mut out = String::new();
        for line in &self.layout {
            for tile in line {
                out += tile.to_string().as_str();
            }
            out += "\n";
        }
        out
    }

    fn cycle(&mut self) -> Option<(usize, usize)> {
        self.tilt_north();
        self.tilt_west();
        self.tilt_south();
        self.tilt_east();

        let new = self.to_string();
        if let Some(i) = self.cycle_history.iter().position(|r| *r == new) {
            return Some((i, self.cycle_history.len()));
        }

        self.cycle_history.push(new);
        None
    }

    fn tilt_north(&mut self) {
        for y in 0..self.layout.len() {
            for x in 0..self.layout[y].len() {
                if self.layout[y][x] != 'O' {
                    continue;
                }

                let mut new_y = y;

                while new_y as isize - 1_isize >= 0 && self.layout[new_y-1][x] == '.' {
                    new_y -= 1;
                }

                if new_y != y {
                    self.layout[new_y][x] = 'O';
                    self.layout[y][x] = '.';
                }
            }
        }
    }

    fn tilt_south(&mut self) {
        for y in (0..self.layout.len()).rev() {
            for x in 0..self.layout[y].len() {
                if self.layout[y][x] != 'O' {
                    continue;
                }

                let mut new_y = y;

                while new_y + 1 < self.layout.len() && self.layout[new_y+1][x] == '.' {
                    new_y += 1;
                }

                if new_y != y {
                    self.layout[new_y][x] = 'O';
                    self.layout[y][x] = '.';
                }
            }
        }
    }

    fn tilt_west(&mut self) {
        for y in 0..self.layout.len() {
            for x in 0..self.layout[y].len() {
                if self.layout[y][x] != 'O' {
                    continue;
                }

                let mut new_x = x;

                while new_x as isize - 1_isize >= 0 && self.layout[y][new_x-1] == '.' {
                    new_x -= 1;
                }

                if new_x != x {
                    self.layout[y][new_x] = 'O';
                    self.layout[y][x] = '.';
                }
            }
        }
    }

    fn tilt_east(&mut self) {
        for y in 0..self.layout.len() {
            for x in (0..self.layout[y].len()).rev() {
                if self.layout[y][x] != 'O' {
                    continue;
                }

                let mut new_x = x;

                while new_x + 1 < self.layout[y].len() && self.layout[y][new_x+1] == '.' {
                    new_x += 1;
                }

                if new_x != x {
                    self.layout[y][new_x] = 'O';
                    self.layout[y][x] = '.';
                }
            }
        }
    }

    fn load(&self) -> usize {
        let mut sum = 0;

        for y in 0..self.layout.len() {
            for tile in &self.layout[y] {
                if *tile == 'O' {
                    sum += self.layout.len() - y;
                }
            }
        }

        sum
    }
}

const TARGET:usize = 1000000000;

fn part2(input:&str) -> usize {
    let mut platform = Platform::new(input);

    let range:(usize,usize);
    loop {
        if let Some(indices) = platform.cycle() {
            range = indices;
            break;
        }
    }

    let mut i = range.0;
    while i + range.1-range.0 < TARGET {
        i += range.1-range.0
    }

    let mut future_platform = Platform::new(platform.cycle_history[range.0].as_str());
    for _ in i..TARGET {
        future_platform.cycle();
    }

    future_platform.load()
}

#[cfg(test)]
mod tests {
    use crate::part2;

    #[test]
    fn example() {
        assert_eq!(part2(
r#"O....#....
O.OO#....#
.....##...
OO.#O....O
.O.....O#.
O.#..O.#.#
..O..#O..O
.......O..
#....###..
#OO..#...."#), 64);
    }
}