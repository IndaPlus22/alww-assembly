use std::{collections::HashMap, io};

use crate::lang_spec::*;
pub fn execute(file_contents: String) {
    let lines: Vec<&str> = file_contents.lines().collect();
    let lines_of_code = lines.len();
    let mut current_line = 0;
    let mut labels: HashMap<char, usize> = HashMap::new();
    let mut registers: [i32; 4] = [0; 4];

    while current_line < lines_of_code {
        let instructions: Vec<char> = lines[current_line].chars().collect();
        if instructions.contains(&':') {
            labels.insert(instructions.first().unwrap().to_owned(), current_line);
        }
        current_line += 1;
    }

    current_line = 0;
    while current_line < lines_of_code {
        let instructions: Vec<char> = lines[current_line].chars().collect();
        if (instructions.contains(&'紫')) {
            let index_value_of_label = labels.get(instructions.last().unwrap()).unwrap();
            if (index_value_of_label < &current_line) {
                panic!("Error!")
            }
            if ((index_value_of_label - current_line) > 2) {
                panic!("Error! {} {}", index_value_of_label, current_line)
            }
        }
        current_line += 1;
    }

    current_line = 0;
    while current_line < lines_of_code {
        let instructions: Vec<char> = lines[current_line].chars().collect();
        let OP = instructions.first().unwrap();
        match OP.to_owned() {
            OPCODE::ADD => {
                let mut tmp_target_r: usize = 0;
                let mut tmp1 = 0;
                let mut tmp2 = 0;
                match instructions[1] {
                    REGISTRY::R1 => tmp_target_r = 0,
                    REGISTRY::R2 => tmp_target_r = 1,
                    REGISTRY::R3 => tmp_target_r = 2,
                    REGISTRY::R4 => tmp_target_r = 3,
                    _ => panic!("Error parsing argument 1 line {}", current_line),
                }
                match instructions[2] {
                    REGISTRY::R1 => tmp1 = registers[0],
                    REGISTRY::R2 => tmp1 = registers[1],
                    REGISTRY::R3 => tmp1 = registers[2],
                    REGISTRY::R4 => tmp1 = registers[3],
                    _ => panic!("Error parsing argument 2 line {}", current_line),
                }
                match instructions[3] {
                    REGISTRY::R1 => tmp2 = registers[0],
                    REGISTRY::R2 => tmp2 = registers[1],
                    REGISTRY::R3 => tmp2 = registers[2],
                    REGISTRY::R4 => tmp2 = registers[3],
                    _ => panic!("Error parsing argument 3 line {}", current_line),
                }
                registers[tmp_target_r] = tmp1 + tmp2;
            }
            OPCODE::ADDI => {
                let mut tmp_target_r = 0;
                let mut tmp1 = 0;
                let mut tmp2: i32 = 0;

                match instructions[2] {
                    REGISTRY::R1 => tmp1 = registers[0],
                    REGISTRY::R2 => tmp1 = registers[1],
                    REGISTRY::R3 => tmp1 = registers[2],
                    REGISTRY::R4 => tmp1 = registers[3],
                    _ => panic!("Error parsing argument 2 line {}", current_line),
                }
                match instructions[3] {
                    '-' => tmp2 = -1 * instructions[4].to_digit(10).unwrap() as i32,
                    _ => tmp2 = instructions[3].to_digit(10).unwrap() as i32,
                }

                match instructions[1] {
                    REGISTRY::R1 => registers[0] = tmp1 + tmp2,
                    REGISTRY::R2 => registers[1] = tmp1 + tmp2,
                    REGISTRY::R3 => registers[2] = tmp1 + tmp2,
                    REGISTRY::R4 => registers[3] = tmp1 + tmp2,
                    _ => panic!("Error parsing argument 1 line {}", current_line),
                }
            }
            OPCODE::BEQ => {
                let mut tmp1 = 0;
                let mut tmp2 = 0;
                match instructions[1] {
                    REGISTRY::R1 => tmp1 = registers[0],
                    REGISTRY::R2 => tmp1 = registers[1],
                    REGISTRY::R3 => tmp1 = registers[2],
                    REGISTRY::R4 => tmp1 = registers[3],
                    '-' => tmp1 = -1 * instructions[2].to_digit(10).unwrap() as i32,
                    _ => tmp1 = instructions[1].to_digit(10).unwrap() as i32,
                }
                match instructions[2] {
                    REGISTRY::R1 => tmp2 = registers[0],
                    REGISTRY::R2 => tmp2 = registers[1],
                    REGISTRY::R3 => tmp2 = registers[2],
                    REGISTRY::R4 => tmp2 = registers[3],
                    '-' => tmp2 = -1 * instructions[3].to_digit(10).unwrap() as i32,
                    _ => tmp2 = instructions[2].to_digit(10).unwrap() as i32,
                }
                if (tmp1 == tmp2) {
                    current_line = labels.get(instructions.last().unwrap()).unwrap().to_owned();
                    continue;
                }
            }

            OPCODE::J => {
                current_line = labels.get(instructions.last().unwrap()).unwrap().to_owned();
                continue;
            }
            OPCODE::LI => {
                let mut tmp_target_r = 0;
                let mut tmp1 = 0;

                match instructions[2] {
                    '-' => tmp1 = -1 * instructions[3].to_digit(10).unwrap() as i32,
                    _ => tmp1 = instructions[2].to_digit(10).unwrap() as i32,
                }
                match instructions[1] {
                    REGISTRY::R1 => registers[0] = tmp1,
                    REGISTRY::R2 => registers[1] = tmp1,
                    REGISTRY::R3 => registers[2] = tmp1,
                    REGISTRY::R4 => registers[3] = tmp1,
                    _ => panic!("Error parsing argument 2 line {}", current_line),
                }
            }

            SYSCALL::PRINT => {
                println!("{}", registers[0])
            }
            SYSCALL::READ => {
                let mut input = String::new();

                io::stdin()
                    .read_line(&mut input)
                    .expect("Failed to read line");
                let parsed_input: i32 = input.trim().parse().unwrap();
                registers[3] = parsed_input;
            }
            SYSCALL::TERMINATE => return,

            _ => {
                current_line += 1;
                continue;
            }
        }
        current_line += 1;
    }
}
