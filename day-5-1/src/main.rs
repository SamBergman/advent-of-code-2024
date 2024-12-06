use std::fs;

struct Instruction {
    x: i32,
    y: i32,
}

impl Instruction {
    fn contains(&self, num: &i32) -> bool {
        self.x == *num
    }

    fn check_instruction(&self, num: &i32, index: usize, update: &Vec<i32>) -> bool {
        let mut pass = true;
        if self.contains(num) {
            let count = update.iter().count();
            if *num == self.x {
                for i in 0..index {
                    pass = pass && update[i] != self.y;
                }
            } else {
                for i in index..count {
                    pass = pass && update[i] != self.x;
                }
            }
        }

        return pass
    }
}

fn get_instructions(s: &String) -> Vec<Instruction> {
    let lines = s.lines();
    let mut instructions: Vec<Instruction> = Vec::new();
    for line in lines {
        if line.contains('|') {
            let values: Vec<&str> = line.split('|').collect();
            instructions.push(Instruction {
                x: values[0].parse::<i32>().unwrap(),
                y: values[1].parse::<i32>().unwrap(),
            })
        }
    }

    instructions
}

fn get_updates(s: &String) -> Vec<Vec<i32>> {
    let lines = s.lines();
    let mut updates: Vec<Vec<i32>> = Vec::new();

    for line in lines {
        if line.contains(',') {
            let values: Vec<i32> = line.split(',').map(|v| v.parse::<i32>().unwrap()).collect();
            updates.push(values);
        }
    }

    updates
}

fn main() {
    let data = fs::read_to_string("../../inputs/day5.txt").expect("Unable to read file");
    
    let instructions = get_instructions(&data);
    let updates = get_updates(&data);
    let mut total = 0;
    let mut passing_updates: Vec<Vec<i32>> = Vec::new();

    for update in updates {
        let mut passes: Vec<bool> = Vec::new();
        // println!("update {:?}", update);
        for (i, val) in update.iter().enumerate() {
            let mut pass = true;
            for instruction in &instructions {
                pass = pass && instruction.check_instruction(&val, i, &update)
            }
            passes.push(pass);
        }
        if passes.into_iter().reduce(|a, b| a && b).unwrap() {
            total += update.get((update.iter().count() - 1) / 2).unwrap();
        }
    }

    println!("{}", total);
}
