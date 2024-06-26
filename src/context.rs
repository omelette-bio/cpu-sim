use std::collections::HashMap;
use crate::data_manipulation::Registers;
use crate::error::Error;
use crate::opcodes::OpCode;

#[derive(Debug)]
pub struct Context {
    register_table: HashMap<Registers, i32>,
    stack: Vec<i32>,

    // if the interpretation has been launched with a file
    in_file: bool,
    exec_stack: Vec<OpCode>,
    program_counter: usize,
}

impl Context {
    pub fn new() -> Self { Context { register_table: HashMap::new(), stack: Vec::new(), in_file: false, exec_stack: Vec::new(), program_counter: 0 } }

    pub fn change_file_context(&mut self) { self.in_file = self.in_file == false; }

    pub fn is_in_file(&self) -> bool { self.in_file }

    pub fn get(&self, registers: Registers) -> Result<i32, Error> {
        match self.register_table.get(&registers) {
            Some(&value) => Ok(value),
            None => Err(Error::RegisterNotInitialized(registers))
        }
    }

    pub fn set(&mut self, registers: Registers, value: i32) { self.register_table.insert(registers, value); }

    pub fn pop_stack(&mut self) -> Result<i32, Error> {
        if self.stack.len() == 0 { return Err(Error::EmptyStack); }
        Ok(self.stack.pop().unwrap())
    }

    pub fn push_stack(&mut self, v: i32) { self.stack.push(v) }

    pub fn change_exec_stack(&mut self, exec_stack: Vec<OpCode>) { self.exec_stack = exec_stack }

    pub fn get_program_counter(&self) -> usize { self.program_counter }

    pub fn set_program_counter(&mut self, val: usize) -> Result<(), Error> {
        if val >= self.exec_stack.len(){ return Err(Error::PCSegFault(val, self.get_exec_stack_end()-1)) }
        self.program_counter = val;
        Ok(())
    }

    pub fn increment_program_counter(&mut self) { self.program_counter += 1; }

    pub fn get_exec_stack_end(&self) -> usize { self.exec_stack.len() }

    pub fn get_current_command(&self) -> OpCode { self.exec_stack[self.program_counter].clone() }

}