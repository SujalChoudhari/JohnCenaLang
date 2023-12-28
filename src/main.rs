use std::env;
use std::fs;
use std::process::exit;
mod constants;
mod environment;
use constants::Instruction;

fn main() {
    let source_code = get_source_code();
    let tokens = tokenize(&source_code);
    evaluate(tokens);
}

fn get_source_code() -> String {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        println!("Needed a File to run");
        exit(1);
    }
    let file_path: &String = &args[1];
    if !fs::metadata(file_path).is_ok() {
        println!("{file_path} does not exist");
    }
    let source_code =
        fs::read_to_string(file_path).expect("Should have been able to read the file");
    return source_code;
}

fn tokenize(source_code: &String) -> Vec<Instruction> {
    let mut size_of_line_array: Vec<i8> = Vec::new();
    let mut instruction_array: Vec<Instruction> = Vec::new();

    for line in source_code.lines() {
        for code_line in line.split("\t") {
            let mut new_line = code_line.replace(" ", "");
            new_line = new_line.replace("\t", "");
            new_line = new_line.replace("\r", ""); // Remove carriage return if present
            new_line = new_line.replace("\n", "");

            if new_line.len() > 0 {
                continue;
            }

            size_of_line_array.push(code_line.len() as i8);
        }
    }

    for number in size_of_line_array {
        let cti = constants::get_code_to_instruction_map();
        if let Some(instruction) = cti.get(&number) {
            instruction_array.push(instruction.clone());
        }
    }

    return instruction_array;
}

fn evaluate(instructions: Vec<Instruction>) {
    let mut environment = environment::Environment::new(5000, 2500); // Replace with your actual initialization

    let mut current_multiplier = 1;

    for instruction in instructions {
        match instruction {
            Instruction::LEFT => environment.move_left(),
            Instruction::RIGHT => environment.move_right(),
            Instruction::INCR => {
                environment.increment_by(current_multiplier);
                current_multiplier = 1
            }
            Instruction::DECR => {
                environment.decrement_by(current_multiplier);
                current_multiplier = 1
            }
            Instruction::READ => environment.get_value(),
            Instruction::READCHAR => environment.get_value_as_char(),
            Instruction::WRITE => environment.set_value(),
            Instruction::MUL10 => current_multiplier = 10,
            Instruction::MUL100 => current_multiplier = 100,
        }
    }
}
