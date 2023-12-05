fn main() {
    println!("{}", part1(include_str!("./input.txt")));
}

type AlmanacMap = Vec<(i64, i64, i64)>;

fn part1(input:&str) -> i64 {
    let lines = Vec::from_iter(input.split('\n'));
    let seeds = Vec::from_iter(lines[0].split_once(":").unwrap().1.split_whitespace().map(|val| {
        val.parse::<i64>().unwrap()
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
            let nums = Vec::from_iter(line.split_whitespace().map(|n| {n.parse::<i64>().unwrap()}));
            maps[current_map as usize].push((nums[0], nums[1], nums[2])); 
        }
    }

    let mut locations:Vec<i64> = Vec::new();
    for seed in seeds {
        let mut current = seed;
        for map in &maps {
            current = convert_through_map(current, map);
        }
        locations.push(current);
    }

    let mut lowest:Option<i64> = None;
    for n in locations {
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

fn convert_through_map(number:i64, map:&AlmanacMap) -> i64 {
    let mut line_index = 0;

    while !(number >= map[line_index].1 && number < map[line_index].1 + map[line_index].2) {
        if line_index < map.len()-1 {
            line_index += 1;
        } else {
            return number;
        }
    }

    return number + (map[line_index].0 - map[line_index].1);
}

#[cfg(test)]
mod tests {
    use crate::part1;

    #[test]
    fn example() {
        assert_eq!(part1(
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
56 93 4"#), 35)
    }
}