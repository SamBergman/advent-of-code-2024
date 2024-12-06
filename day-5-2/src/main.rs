use std::fs;

#[derive(Debug)]
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
            for i in 0..index {
                pass = pass && update[i] != self.y;
            }
        }

        return pass
    }

    fn fix_update_with_instruction(&self, update: &mut [i32]) {
        let x_index = update.iter().position(|&s| s == self.x).unwrap();
        let y_index = update.iter().position(|&s| s == self.y).unwrap();

        println!("{}, {}, {:?}", x_index, y_index, update);

        if y_index < x_index + 1 {
            update[y_index..=y_index + 1].rotate_left(1);
        } else {
            update[y_index + 1..=y_index].rotate_right(1);
        }
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

    instructions.sort_by_key(|a| a.x);
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
    // let mut total = 0;

    for mut update in updates {
        // let mut passes: Vec<bool> = Vec::new();
        let mut instructions_to_run: Vec<&Instruction> = Vec::new();
        for (i, val) in update.iter().enumerate() {
            // let mut pass = true;
            for instruction in &instructions {
                if !instruction.check_instruction(&val, i, &update) {
                    instructions_to_run.push(&instruction);
                }
            }
        }
        
        for instruction in instructions_to_run {
            instruction.fix_update_with_instruction(&mut update);
        }

        let mut passes: Vec<bool> = Vec::new();
        for (i, val) in update.iter().enumerate() {
            let mut pass = true;
            for instruction in &instructions {
                pass = pass && instruction.check_instruction(&val, i, &update)
            }
            passes.push(pass);
        }

        for pass in passes {
            println!("{}", pass);
        }
    }
}
