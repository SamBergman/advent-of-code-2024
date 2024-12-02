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
        let numbers = spl.iter().map(|val| val.parse::<i32>().unwrap());
        
        let mut last: i32;
        let mut mode;
        let mut fail;
        let number_count = numbers.count() as i32;
        
        for index in -1..number_count {
            let mut for_numbers = spl.iter().map(|val| val.parse::<i32>().unwrap());
            let mut while_count: i32 = 0;
            last = -1;
            mode = Mode::Unset;
            fail = false;
            while let Some(val) = for_numbers.next() {
                if while_count == index {
                    while_count += 1;
                    continue;
                }

                if last == -1 {
                    last = val;
                    while_count += 1;
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
                        } else {
                            fail = true;
                            break;
                        }
                    },
                    Mode::Less => {
                        if difference < 0 && range.contains(&abs_diff) {
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
                while_count += 1;
            };
            if fail == false {
                total += 1;
                break;
            }
        }
    }

    std::println!("{}", total);

}
