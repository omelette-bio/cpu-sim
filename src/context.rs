use std::collections::HashMap;
use crate::data_manipulation::Registers;
use crate::error::EvalError;
use crate::opcodes::OpCode;
use crate::stack::Stack;

pub enum Context {
    LineByLine(LineContext),
    File(FileContext)
}

pub trait ContextManip {
    fn get_register_value(&self, register: &Registers) -> Result<i32, EvalError>;
    fn set_register_value(&mut self, register: &Registers, value: i32);
    fn pop_stack(&mut self) -> Result<i32, EvalError>;
    fn push_stack(&mut self, value: i32);
}

pub trait ProgrammeCounterManip {
    fn increment_program_counter(&self);
    fn add_program_counter(&self, value: usize);
    fn sub_program_counter(&self, value: usize);
}


#[derive(Debug)]
pub struct FileContext {
    register_table: HashMap<Registers, i32>,
    stack: Stack,

    exec_stack: Vec<OpCode>,
    program_counter: usize,
    label_map: HashMap<String, usize>
}

#[derive(Debug)]
pub struct LineContext {
    register_table: HashMap<Registers, i32>,
    stack: Stack,
}

impl Default for FileContext {
    fn default() -> Self {
        FileContext { register_table: HashMap::default(), stack: Stack::default(), exec_stack: Vec::new(), program_counter: 0, label_map: HashMap::default() }
    }
}

impl ContextManip for FileContext {
    fn get_register_value(&self, register: &Registers) -> Result<i32, EvalError> {
        match self.register_table.get(register) {
            Some(&value) => Ok(value),
            None => Err(EvalError::RegisterNotInitialized(*register))
        }
    }
    fn set_register_value(&mut self, register: &Registers, value: i32) { self.register_table.insert(*register, value); }
    fn pop_stack(&mut self) -> Result<i32, EvalError> { self.stack.pop_value() }
    fn push_stack(&mut self, value: i32) { self.stack.push_value(value) }
}

impl ContextManip for LineContext {
    fn get_register_value(&self, register: &Registers) -> Result<i32, EvalError> {
        match self.register_table.get(register) {
            Some(&value) => Ok(value),
            None => Err(EvalError::RegisterNotInitialized(*register))
        }
    }
    fn set_register_value(&mut self, register: &Registers, value: i32) { self.register_table.insert(*register, value); }
    fn pop_stack(&mut self) -> Result<i32, EvalError> { self.stack.pop_value() }
    fn push_stack(&mut self, value: i32) { self.stack.push_value(value) }
}

impl Default for LineContext {
    fn default() -> Self {
        LineContext { register_table: HashMap::new(), stack: Stack::default() }
    }
}

// impl Context {
//     pub fn new() -> Self { Context { register_table: HashMap::new(), stack: Vec::new(), in_file: false, exec_stack: Vec::new(), program_counter: 0 } }
//
//     pub fn change_file_context(&mut self) { self.in_file = self.in_file == false; }
//
//     pub fn is_in_file(&self) -> bool { self.in_file }
//
//     pub fn get(&self, registers: Registers) -> Result<i32, Error> {
//         match self.register_table.get(&registers) {
//             Some(&value) => Ok(value),
//             None => Err(Error::RegisterNotInitialized(registers))
//         }
//     }
//
//     pub fn set(&mut self, registers: Registers, value: i32) { self.register_table.insert(registers, value); }
//
//     pub fn pop_stack(&mut self) -> Result<i32, Error> {
//         if self.stack.len() == 0 { return Err(Error::EmptyStack); }
//         Ok(self.stack.pop().unwrap())
//     }
//
//     pub fn push_stack(&mut self, v: i32) { self.stack.push(v) }
//
//     pub fn change_exec_stack(&mut self, exec_stack: Vec<OpCode>) { self.exec_stack = exec_stack }
//
//     pub fn get_program_counter(&self) -> usize { self.program_counter }
//
//     pub fn set_program_counter(&mut self, val: usize) -> Result<(), Error> {
//         if val >= self.exec_stack.len(){ return Err(Error::PCSegFault(val, self.get_exec_stack_end()-1)) }
//         self.program_counter = val;
//         Ok(())
//     }
//
//     pub fn increment_program_counter(&mut self) { self.program_counter += 1; }
//
//     pub fn get_exec_stack_end(&self) -> usize { self.exec_stack.len() }
//
//     pub fn get_current_command(&self) -> OpCode { self.exec_stack[self.program_counter].clone() }
//
// }