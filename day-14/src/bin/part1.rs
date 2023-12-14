fn main() {
    println!("{}", part1(include_str!("./input.txt")));
}

struct Platform {
    layout: Vec<Vec<char>>
}

impl Platform {
    fn new(input:&str) -> Self {
        let mut layout = Vec::new();

        for line in input.lines() {
            layout.push(Vec::from_iter(line.chars()));
        }

        Self { 
            layout
        }
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

fn part1(input:&str) -> usize {
    let mut platform = Platform::new(input);
    platform.tilt_north();
    platform.load()
}

#[cfg(test)]
mod tests {
    use crate::part1;

    #[test]
    fn example() {
        assert_eq!(part1(
r#"O....#....
O.OO#....#
.....##...
OO.#O....O
.O.....O#.
O.#..O.#.#
..O..#O..O
.......O..
#....###..
#OO..#...."#), 136);
    }
}