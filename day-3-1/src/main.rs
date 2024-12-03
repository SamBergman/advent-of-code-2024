use std::{fs, num::ParseIntError};
use regex::Regex;

fn main() {
    let data = fs::read_to_string("../../inputs/day3.txt").expect("Unable to read file");
    let r_exp = Regex::new(r"mul\(([0-9,]+)\)").unwrap();
    let matches: Vec<&str> = r_exp.find_iter(&data).map(|m| m.as_str()).collect();
    let mut total: u64 = 0;
    for group in  matches {
        let values = group.replace("mul(", "").replace(")", "");
        let numbers: Vec<Result<u64, ParseIntError>> = values.split(",").map(|v| v.parse::<u64>()).collect();
        let mut value: u64 = 1;
        let mut ran_count = 0;

        for n in numbers {
            match n {
                Ok(v) => {
                    value *= v;
                    ran_count += 1;
                },
                Err(error) => {
                    println!("Error found {}", error);
                    break;
                }
            }
        }

        if ran_count == 2 {
            total += value;
        }

    }
    println!("{}", total);

}
