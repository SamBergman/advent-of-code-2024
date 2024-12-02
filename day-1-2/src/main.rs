use std::fs;

fn main() {
    let data = fs::read_to_string("../../inputs/day1.txt").expect("Unable to read file");
    let mut vec1: Vec<i32> = Vec::new();
    let mut vec2: Vec<i32> = Vec::new();
    let lines = data.lines();

    for line in lines {
        let spl: Vec<&str> = line.split(" ").filter(|s| !s.is_empty()).collect();
        vec1.push(spl[0].parse::<i32>().unwrap());
        vec2.push(spl[1].parse::<i32>().unwrap());
    }

    vec1.sort();
    vec2.sort();

    let mut total = 0;

    for index in 0..1000 {
        let value = vec1[index];
        let count = vec2.iter().filter(|n| **n == value).count() as i32;

        total += value * count;
    }

    std::println!("{}", total);
}
