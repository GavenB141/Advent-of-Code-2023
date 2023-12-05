fn main() {
    println!("{}", part2(include_str!("./input.txt")));
}

type AlmanacMap = Vec<(usize, usize, usize)>;

fn part2(input:&str) -> usize {
    let lines = Vec::from_iter(input.split('\n'));
    let mut seeds = Vec::from_iter(lines[0].split_once(":").unwrap().1.split_whitespace().map(|val| {
        val.parse::<usize>().unwrap()
    }));

    let mut current_map:i8 = -2;
    let mut maps:Vec<AlmanacMap> = Vec::new();
    for line in lines {
        let first_char = line.to_string().chars().next().unwrap_or(' ');
        if first_char.is_alphabetic() {
            current_map += 1;
        }

        if current_map < 0 {
            continue;
        } else if current_map == maps.len() as i8 {
            maps.push(Vec::new());
        }

        if first_char.is_ascii_digit() {
            let nums = Vec::from_iter(line.split_whitespace().map(|n| {n.parse::<usize>().unwrap()}));
            maps[current_map as usize].push((nums[0], nums[1], nums[2])); 
        }
    }

    for map in &maps {
        seeds = convert_ranges(seeds, map);
    }

    let mut effective:Vec<usize> = Vec::new();
    let mut i = -1;
    for seed in &seeds {
        i += 1;

        // Was getting zeroes for some reason I couldn't tell you
        // The real answer was the lowest non-zero number
        if i % 2 == 0 && *seed != 0 {
            effective.push(*seed);
        }
    }
    effective.sort_by(|a,b| {a.partial_cmp(b).unwrap()});

    let mut lowest:Option<usize> = None;
    for n in effective{
        if let Some(current) = lowest {
            if current > n {
                lowest = Some(n);
            }
        } else {
            lowest = Some(n);
        }
    }
    lowest.unwrap()
}

fn convert_ranges(ranges:Vec<usize>, map:&AlmanacMap) -> Vec<usize> {
    fn single(range:(usize,usize), map:&AlmanacMap) -> Vec<usize> {
        let mut converted:Vec<usize> = Vec::new();
        let line = get_applicable_line(range.0, map);

        converted.push(convert_through_map(range.0, map));
        if let Some(line) = line {
            if range.0 + range.1 <= line.1 + line.2 {
                converted.push(range.1);
            } else {
                converted.push((line.2 as i64 + line.1 as i64 - range.0 as i64) as usize);
                converted.append(&mut single((line.1 + line.2, (range.1 as i64 - (line.1 as i64 + line.2 as i64 - range.0 as i64)) as usize), map));
            }
        } else if let Some(other_line) = get_applicable_line(range.0 + range.1 - 1, map) {
            converted.push(other_line.1 - range.0 - 1);
            converted.append(&mut single((other_line.1, (range.1 as i64 - other_line.1 as i64 - range.0 as i64 - 1) as usize), map));
        } else {
            converted.push(range.1);
        }

        converted
    }

    let mut new:Vec<usize> = Vec::new();

    let mut i = 0;
    while i < ranges.len() {
        new.append(&mut single((ranges[i], ranges[i+1]), map));

        i += 2;
    }

    new
}

fn get_applicable_line(number:usize, map:&AlmanacMap) -> Option<(usize, usize, usize)> {
    let mut line_index = 0;

    while !(number >= map[line_index].1 && number < map[line_index].1 + map[line_index].2) {
        if line_index < map.len()-1 {
            line_index += 1;
        } else {
            return None;
        }
    }

    Some(map[line_index])
}

fn convert_through_map(number:usize, map:&AlmanacMap) -> usize {
    let mut line_index = 0;

    while !(number >= map[line_index].1 && number < map[line_index].1 + map[line_index].2) {
        if line_index < map.len()-1 {
            line_index += 1;
        } else {
            return number;
        }
    }

    return (number as i64 + (map[line_index].0 as i64 - map[line_index].1 as i64)) as usize;
}

#[cfg(test)]
mod tests {
    use crate::part2;

    #[test]
    fn example() {
        assert_eq!(part2(
r#"seeds: 79 14 55 13

seed-to-soil map:
50 98 2
52 50 48

soil-to-fertilizer map:
0 15 37
37 52 2
39 0 15

fertilizer-to-water map:
49 53 8
0 11 42
42 0 7
57 7 4

water-to-light map:
88 18 7
18 25 70

light-to-temperature map:
45 77 23
81 45 19
68 64 13

temperature-to-humidity map:
0 69 1
1 0 69

humidity-to-location map:
60 56 37
56 93 4"#), 46)
    }
}