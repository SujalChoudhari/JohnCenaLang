use std::collections::HashMap;

#[derive(Clone, Debug)]
pub enum Instruction {
    LEFT,
    RIGHT,
    INCR,
    DECR,
    READ,
    READCHAR,
    MUL10,
    MUL100,
    WRITE,
}

pub fn get_code_to_instruction_map() -> HashMap<i8, Instruction> {
    let code_to_instruction_map: HashMap<i8, Instruction> = HashMap::from([
        (1, Instruction::LEFT),
        (2, Instruction::RIGHT),
        (3, Instruction::INCR),
        (4, Instruction::DECR),
        (5, Instruction::READ),
        (6, Instruction::READCHAR),
        (7, Instruction::MUL10),
        (8, Instruction::MUL100),
        (9, Instruction::WRITE),
    ]);

    return code_to_instruction_map;
}
