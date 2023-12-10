fn main() {
    println!("{}", part1(include_str!("./input.txt")));
}

type Point = (usize, usize);

struct Pipes {
    layout:Vec<Vec<char>>
}

impl Pipes {
    fn start(&self) -> Option<Point> {
        for y in 0..self.layout.len() {
            for x in 0..self.layout[y].len() {
                if self.layout[y][x] == 'S' {
                    return Some((x, y));
                }
            }
        }

        None
    }

    fn size(&self) -> Option<usize> {
        let start = self.start()?;

        let mut size = 0;
        let mut current = start;
        let mut prev = self.connections_of(start)[0];

        while current != start || size == 0 {
            size += 1;
            let conns = self.connections_of(current);

            for conn in conns {
                if conn != prev {
                    println!("{} {} > {} {} > {} {}", prev.0, prev.1, current.0, current.1, conn.0, conn.1);
                    prev = current;
                    current = conn;
                    break;
                }
            }
        }

        Some(size)
    }

    fn connections_of(&self, p:Point) -> Vec<Point> {
        let mut out:Vec<Point> = Vec::new();

        if p.0 > 0 && (
            self.layout[p.1][p.0] == 'S' ||
            self.layout[p.1][p.0] == '7' ||
            self.layout[p.1][p.0] == 'J' ||
            self.layout[p.1][p.0] == '-'
        ) && (
            self.layout[p.1][p.0-1] == 'S' || 
            self.layout[p.1][p.0-1] == 'F' || 
            self.layout[p.1][p.0-1] == 'L' || 
            self.layout[p.1][p.0-1] == '-' 
        ) {
            out.push((p.0-1, p.1));
        }

        if p.0 < self.layout[p.1].len()-1 && (
            self.layout[p.1][p.0] == 'S' ||
            self.layout[p.1][p.0] == 'F' ||
            self.layout[p.1][p.0] == 'L' ||
            self.layout[p.1][p.0] == '-'
        ) && (
            self.layout[p.1][p.0+1] == 'S' || 
            self.layout[p.1][p.0+1] == 'J' || 
            self.layout[p.1][p.0+1] == '7' || 
            self.layout[p.1][p.0+1] == '-' 
        ) {
            out.push((p.0+1, p.1));
        }

        if p.1 > 0 && (
            self.layout[p.1][p.0] == 'S' ||
            self.layout[p.1][p.0] == 'L' ||
            self.layout[p.1][p.0] == 'J' ||
            self.layout[p.1][p.0] == '|'
        ) && (
            self.layout[p.1-1][p.0] == 'S' || 
            self.layout[p.1-1][p.0] == 'F' || 
            self.layout[p.1-1][p.0] == '7' || 
            self.layout[p.1-1][p.0] == '|' 
        ) {
            out.push((p.0, p.1-1));
        }

        if p.1 < self.layout.len()-1&& (
            self.layout[p.1][p.0] == 'S' ||
            self.layout[p.1][p.0] == 'F' ||
            self.layout[p.1][p.0] == '7' ||
            self.layout[p.1][p.0] == '|'
        ) && (
            self.layout[p.1+1][p.0] == 'S' || 
            self.layout[p.1+1][p.0] == 'J' || 
            self.layout[p.1+1][p.0] == 'L' || 
            self.layout[p.1+1][p.0] == '|' 
        ) {
            out.push((p.0, p.1+1));
        }

        assert_eq!(out.len(), 2);

        out
    }
}

fn part1(input:&str) -> usize {
    let pipes = Pipes {
        layout: Vec::from_iter(input.split('\n').map(|l| Vec::from_iter(l.chars())))
    };

    pipes.size().unwrap() / 2
}

#[cfg(test)]
mod tests {
    use crate::part1;

    #[test]
    fn example() {
        assert_eq!(part1(
r#".....
.S-7.
.|.|.
.L-J.
....."#), 4);

        assert_eq!(part1(
r#"..F7.
.FJ|.
SJ.L7
|F--J
LJ..."#), 8);
    }
}