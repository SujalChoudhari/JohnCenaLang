use std::io;
use std::iter;
pub struct Environment {
    memory: Vec<i32>,
    size: usize,
    needle: i32,
}

impl Environment {
    pub fn new(size: usize, current: i32) -> Environment {
        Environment {
            memory: iter::repeat(0).take(size).collect(),
            size,
            needle: current,
        }
    }

    pub fn move_left(&mut self) {
        self.needle -= 1;
        if self.needle < 0 {
            self.needle = self.size as i32 - 1;
        }
    }

    pub fn move_right(&mut self) {
        self.needle = (self.needle + 1) % self.size as i32;
    }

    pub fn increment_by(&mut self, val: i32) {
        self.memory[self.needle as usize] += val;
    }

    pub fn decrement_by(&mut self, val: i32) {
        self.memory[self.needle as usize] -= val;
    }

    pub fn get_value(&self) {
        print!("{}", self.memory[self.needle as usize])
    }

    pub fn get_value_as_char(&self) {
        print!("{}", self.memory[self.needle as usize] as u8 as char)
    }

    pub fn set_value(&mut self) {
        let mut new_value = String::new();
        let _ = io::stdin().read_line(&mut new_value);
        let num = new_value.trim().parse::<i32>().expect("Expected a number");
        self.memory[self.needle as usize] = num;
    }
}
