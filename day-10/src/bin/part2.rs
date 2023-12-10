fn main() {
    println!("{}", part2(include_str!("./input.txt")));
}

type Point = (usize, usize);

struct Pipes {
    layout:Vec<Vec<char>>
}

impl Pipes {
    fn start(&self) -> Point {
        for y in 0..self.layout.len() {
            for x in 0..self.layout[y].len() {
                if self.layout[y][x] == 'S' {
                    return (x, y);
                }
            }
        }

        panic!("Invalid layout detected");
    }

    fn enclosed_area(&self) -> usize {
        // Space out grid
        let mut spaced = self.denoised();
        for i in 0..spaced.len() {
            for j in (0..spaced[i].len()).rev() {
                if j* 0 == 0 {
                    spaced[i].insert(j, '*')
                }
            }
        }
        for i in (0..spaced.len()).rev() {
            let mut new_row:Vec<char> = Vec::new();
            for _ in &spaced[i] {
                new_row.push('*');
            }
            spaced.insert(i, new_row);
        }

        let mut spaced = Self { layout: spaced };

        // Refill connections 
        for y in 0..spaced.layout.len() {
            for x in 0..spaced.layout[y].len() {
                let conns = spaced.connections_of((x, y));
                if y != 0 && conns.contains(&(x, y-1)) && conns.contains(&(x, y+1)) {
                    spaced.layout[y][x] = '|';
                }

                if x != 0 && conns.contains(&(x-1, y)) && conns.contains(&(x+1, y)) {
                    spaced.layout[y][x] = '-';
                }
            }
        }

        fn measure_enclosing_region(p:Point, layout:&Vec<Vec<char>>, explored:&mut Vec<Point>) -> usize {
            let mut to_explore = vec![p];
            let mut closed = true;
            let mut area = 0;

            let mut i = 0;
            while i < to_explore.len() {
                let p = to_explore[i];
                if !explored.contains(&p) {
                    if layout[p.1][p.0] == '*' || layout[p.1][p.0] == '.' {
                        if p.0 == 0 || p.1 == 0 || 
                            p.0 == layout[p.1].len()-1 || 
                            p.1 == layout.len()-1 
                        {
                            closed = false
                        }

                        if layout[p.1][p.0] == '.' {
                            area += 1;
                        }

                        if p.0 != 0 { to_explore.push((p.0-1, p.1)); }
                        if p.0 != layout[p.1].len()-1 { to_explore.push((p.0+1, p.1)); }
                        if p.1 != 0 { to_explore.push((p.0, p.1-1)); }
                        if p.1 != layout.len()-1 { to_explore.push((p.0, p.1+1)); }
                    }

                    explored.push(p);
                }
                i += 1;
            }

            if closed {
                area
            } else {
                0
            }
        }

        // Start counting
        let mut sum = 0;
        let mut explored:Vec<Point> = Vec::new();
        for y in 0..spaced.layout.len() {
            for x in 0..spaced.layout[y].len() {
                if !explored.contains(&(x, y)) && spaced.layout[y][x] == '.' {
                    sum += measure_enclosing_region((x, y), &spaced.layout, &mut explored);
                }
            }
        }

        sum
    }

    fn denoised(&self) -> Vec<Vec<char>> {
        let mut new_layout:Vec<Vec<char>> = Vec::from_iter(self.layout.iter()
            .map(|line| Vec::from_iter(line.iter().map(|_| '.'))));

        let start = self.start();

        let mut started = false;
        let mut current = start;
        let mut prev = self.connections_of(start)[0];

        while current != start || !started {
            started = true;
            let conns = self.connections_of(current);
            new_layout[current.1][current.0] = self.layout[current.1][current.0];

            for conn in conns {
                if conn != prev {
                    prev = current;
                    current = conn;
                    break;
                }
            }
        }

        new_layout
    }

    fn connections_of(&self, p:Point) -> Vec<Point> {
        let mut out:Vec<Point> = Vec::new();

        if p.0 > 0 && (
            self.layout[p.1][p.0] == 'S' ||
            self.layout[p.1][p.0] == '*' ||
            self.layout[p.1][p.0] == '7' ||
            self.layout[p.1][p.0] == 'J' ||
            self.layout[p.1][p.0] == '-'
        ) && (
            self.layout[p.1][p.0-1] == 'S' || 
            self.layout[p.1][p.0-1] == '*' || 
            self.layout[p.1][p.0-1] == 'F' || 
            self.layout[p.1][p.0-1] == 'L' || 
            self.layout[p.1][p.0-1] == '-' 
        ) {
            out.push((p.0-1, p.1));
        }

        if p.0 < self.layout[p.1].len()-1 && (
            self.layout[p.1][p.0] == 'S' ||
            self.layout[p.1][p.0] == '*' ||
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
            self.layout[p.1][p.0] == '*' ||
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
            self.layout[p.1][p.0] == '*' ||
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

        out
    }
}

fn part2(input:&str) -> usize {
    let pipes = Pipes {
        layout: Vec::from_iter(input.split('\n').map(|l| Vec::from_iter(l.chars())))
    };

    pipes.enclosed_area()
}

#[cfg(test)]
mod tests {
    use crate::part2;

    #[test]
    fn example() {
        assert_eq!(part2(
r#"...........
.S-------7.
.|F-----7|.
.||.....||.
.||.....||.
.|L-7.F-J|.
.|..|.|..|.
.L--J.L--J.
..........."#), 4);

        assert_eq!(part2(
r#".F----7F7F7F7F-7....
.|F--7||||||||FJ....
.||.FJ||||||||L7....
FJL7L7LJLJ||LJ.L-7..
L--J.L7...LJS7F-7L7.
....F-J..F7FJ|L7L7L7
....L7.F7||L7|.L7L7|
.....|FJLJ|FJ|F7|.LJ
....FJL-7.||.||||...
....L---J.LJ.LJLJ..."#), 8);

    assert_eq!(part2(
r#"FF7FSF7F7F7F7F7F---7
L|LJ||||||||||||F--J
FL-7LJLJ||||||LJL-77
F--JF--7||LJLJ7F7FJ-
L---JF-JLJ.||-FJLJJ7
|F|F-JF---7F7-L7L|7|
|FFJF7L7F-JF7|JL---7
7-L-JL7||F7|L7F-7F7|
L.L7LFJ|||||FJL7||LJ
L7JLJL-JLJLJL--JLJ.L"#), 10);
    }
}