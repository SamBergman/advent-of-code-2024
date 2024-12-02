use std::fs;

#[derive(PartialEq)]
enum Mode {
    Greater,
    Less,
    Unset
}

fn main() {
    let data = fs::read_to_string("../../inputs/day2.txt").expect("Unable to read file");
    let lines = data.lines();
    let mut total = 0;

    for line in lines {
        let spl: Vec<&str> = line.split(" ").filter(|s| !s.is_empty()).collect();
        let mut numbers = spl.iter().map(|val| val.parse::<i32>().unwrap());
        
        let mut last: i32 = -1;
        let mut mode= Mode::Unset;
        let mut fail = false;
        
        while let Some(val) = numbers.next() {
            if last == -1 {
                last = val;
                continue;
            }

            let difference = last - val;
            let range = 1..4;
            let abs_diff = i32::abs(difference);

            if mode == Mode::Unset {
                if difference < 0 {
                    mode = Mode::Less;
                } else if difference > 0 {
                    mode = Mode::Greater;
                } else {
                    fail = true;
                    break;
                }
            }

            match mode {
                Mode::Greater => {
                    if difference > 0 && range.contains(&abs_diff) {
                        fail = false;
                    } else {
                        fail = true;
                        break;
                    }
                },
                Mode::Less => {
                    if difference < 0 && range.contains(&abs_diff) {
                        fail = false;
                    } else {
                        fail = true;
                        break;
                    }
                },
                _ => {
                    panic!("Should not have gotten here");
                }
            }
            last = val;
        };

        if !fail {
            total += 1;
        }
    }

    std::println!("{}", total);

}
