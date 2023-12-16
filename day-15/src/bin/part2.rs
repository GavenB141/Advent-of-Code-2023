#![feature(ascii_char)]

use std::collections::HashMap;

fn main() {
    println!("{}", part1(include_str!("./input.txt")));
}

fn part1(input:&str) -> usize {
    let mut boxes:HashMap<u8, Vec<(String, u8)>> = HashMap::new();

    for step in input.split(',') {
        if step.contains('-') {
            let label = step.replace('-', "");
            let box_num = hash(label.as_str());

            if let Some(bocks) = boxes.get_mut(&box_num) {
                for i in 0..bocks.len() {
                    if bocks[i].0 == label {
                        bocks.remove(i);
                        break;
                    }
                }
            }
        } else {
            let (label, lens) = step.split_once('=').unwrap();
            let box_num = hash(label);

            if let Some(bocks) = boxes.get_mut(&box_num) {
                let mut changed = false;
                for i in 0..bocks.len() {
                    if bocks[i].0 == label {
                        bocks[i].1 = lens.parse::<u8>().unwrap();
                        changed = true;
                        break;
                    }
                }

                if !changed {
                    bocks.push((label.to_string(), lens.parse::<u8>().unwrap()));
                }
            } else {
                boxes.insert(box_num, vec![(label.to_string(), lens.parse::<u8>().unwrap())]);
            }
        }
    }

    let mut sum:usize = 0;

    for i in 0..256 {
        if let Some(bocks) = boxes.get(&(i as u8)) {
            for j in 0..bocks.len() {
                sum += (i+1) * (j+1) * bocks[j].1 as usize;
            }
        }
    }
    
    sum
}

fn hash(step:&str) -> u8 {
    let mut current:usize = 0;

    for c in step.chars() {
        current += c.as_ascii().unwrap().to_u8() as usize;
        current *= 17;
        current %= 256;
    }

    current as u8
}

#[cfg(test)]
mod tests {
    use crate::part1;

    #[test]
    fn example() {
        assert_eq!(part1(
r#"rn=1,cm-,qp=3,cm=2,qp-,pc=4,ot=9,ab=5,pc-,pc=6,ot=7"#), 145);
    }
}