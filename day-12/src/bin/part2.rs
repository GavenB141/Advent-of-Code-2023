fn main() {
    println!("{}", part2(include_str!("./input.txt")));
}

fn part2(input:&str) -> usize {
    let mut sum = 0;

    for (i, line) in input.lines().enumerate() {
        println!("{i}");
        sum += possible_arrangements(line).len();
    }

    sum
}

fn possible_arrangements(line:&str) -> Vec<String> {
    let (row, pattern) = line.split_once(' ').unwrap();
    let row = format!("{}?{}?{}?{}?{}",row,row,row,row,row);
    let pattern = format!("{},{},{},{},{}",pattern,pattern,pattern,pattern,pattern);

    let pattern = Vec::from_iter(pattern.split(',').map(|s|s.parse::<usize>().unwrap()));

    let total:usize = pattern.iter().fold(0, |acc, i| acc + i);
    let known:usize = row.chars().fold(0, |acc, c| if c == '#' {acc+1} else {acc});

    if total - known == 0 {
        return vec![line.replace('?', ".")];
    }
    
    let mut unknown_indices:Vec<usize> = Vec::new();
    for (i, spring) in row.chars().enumerate() {
        if spring == '?' {
            unknown_indices.push(i);
        }
    }

    let mut output = Vec::new();

    let mut slots:Vec<usize> = Vec::from_iter((0..total-known).map(|i|i));

    fn inc_slots(slots:&mut Vec<usize>, upto:usize) -> bool {
        for i in (0..slots.len()).rev() {
            if slots[i] < if i==slots.len()-1{upto}else{slots[i+1]-1} {
                slots[i] += 1;
                break;
            } else {
                if i == 0 {
                    return true;
                }
                
                slots[i] = slots[i-1]+2;
                for j in i+1..slots.len() {
                    slots[j] = slots[j-1]+1;
                    while slots[j] > if j==slots.len()-1{upto}else{slots[j+1]-1} {
                        slots[j] -= 1;
                    }
                }

                while slots[i] > if i==slots.len()-1{upto}else{slots[i+1]-1} {
                    slots[i] -= 1;
                }
            }
        }

        false
    }

    loop {
        let mut test = row.to_string();

        for i in &slots {
            test.replace_range(unknown_indices[*i]..=unknown_indices[*i], "#");
        }
        test = test.replace('?', ".");

        println!("{test}");
        if validate_row(test.as_str(), &pattern) {
            output.push(test);
        }

        if inc_slots(&mut slots, unknown_indices.len()-1) {
            break;
        }
    }
    println!("===");

    output
}

fn validate_row(row:&str, pattern:&Vec<usize>) -> bool {
    let mut acc = 0;
    let mut pattern_index = 0;

    for spring in row.chars() {
        if spring == '?' {
            return false;
        } else if spring == '#' {
            if pattern_index >= pattern.len() {
                return false;
            }
            acc += 1
        } else if spring == '.' && acc != 0 {
            if acc != pattern[pattern_index] {
                return false;
            }

            acc = 0;
            pattern_index += 1;
        }
    }

    if pattern_index <= pattern.len() - 2 {
        return false;
    } else if pattern_index == pattern.len() - 1 {
        return pattern[pattern_index] == acc;
    }

    true
}

#[cfg(test)]
mod tests {
    use crate::part2;

    #[test]
    fn example() {
        assert_eq!(part2(
r#"???.### 1,1,3
.??..??...?##. 1,1,3
?#?#?#?#?#?#?#? 1,3,1,6
????.#...#... 4,1,1
????.######..#####. 1,6,5
?###???????? 3,2,1"#), 525152);
    }
}