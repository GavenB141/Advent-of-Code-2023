fn main() {
    let modified = Vec::from_iter(include_str!("mod.txt").split('\n'));

    for i in 0..modified[0].len() {
        let mut clear = true;

        for line in &modified {
            let cur = Vec::from_iter(line.chars())[i];
            if cur == '#' || cur == 'V' {
                clear = false;
            }
        }

        if clear {
            println!("{i}");
        }
    }
}