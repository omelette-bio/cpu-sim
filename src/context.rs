use std::collections::HashMap;
use crate::data_manipulation::Registers;
use crate::error::Error;

#[derive(Debug)]
pub struct Context {
    in_file: bool,
    register_table: HashMap<Registers, i32>,
    stack: Vec<i32>,
}

impl Context {
    pub fn new() -> Self { Context { register_table: HashMap::new(), stack: Vec::new(), in_file: false } }

    pub fn change_file_context(&mut self) { self.in_file = self.in_file == false; }

    pub fn is_in_file(&self) -> bool { self.in_file }

    pub fn get(&self, registers: Registers) -> Result<i32, Error> {
        match self.register_table.get(&registers) {
            Some(&value) => Ok(value),
            None => Err(Error::RegisterNotInitialized(registers))
        }
    }

    pub fn set(&mut self, registers: Registers, value: i32) {
        self.register_table.insert(registers, value);
    }

    pub fn pop_stack(&mut self) -> Result<i32, Error> {
        if self.stack.len() == 0 { return Err(Error::EmptyStack); }
        Ok(self.stack.pop().unwrap())
    }

    pub fn push_stack(&mut self, v: i32) {
        self.stack.push(v)
    }
}