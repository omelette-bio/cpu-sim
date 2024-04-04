use std::collections::HashMap;
use crate::data_manipulation::Registers;

#[derive(Debug)]
pub struct Context {
    pub register_table: HashMap<Registers, i32>,
    in_file: bool,
}

impl Context {
    pub fn new() -> Self { Context { register_table: HashMap::new(), in_file: false } }

    pub fn change_file_context(&mut self) { self.in_file = self.in_file == false; }

    pub fn is_in_file(&self) -> bool { self.in_file }
}