use std::collections::HashMap;
use std::fmt;
use std::fmt::Formatter;


#[derive(Debug)]
pub struct Context {
    register_table: HashMap<Registers, i32>,
    in_file: bool,
}

impl Context {
    pub fn new() -> Self {
        Context { register_table: HashMap::new(), in_file: false }
    }

    pub fn change_file_context(&mut self) {
        self.in_file = self.in_file == false;
    }

    pub fn is_in_file(&self) -> bool {
        self.in_file
    }
}

#[derive(Debug, Eq, Hash, PartialEq, PartialOrd, Copy, Clone)]
pub enum Registers {
    R1, R2, R3, R4, R5, R6, R7, R8
}

impl Registers {
    pub fn get_val(&self, c: &Context) -> Result<i32, String> {
        match c.register_table.get(self) {
            Some(value) => Ok(*value),
            None => Err(format!("no value attributed to this register : {}", self))
        }
    }

    pub fn set_val(&self, value: i32, c: &mut Context) {
        c.register_table.insert(*self, value);
    }
}

impl fmt::Display for Registers {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Registers::R1 => write!(f,"{}", "R1"),
            Registers::R2 => write!(f,"{}", "R2"),
            Registers::R3 => write!(f,"{}", "R3"),
            Registers::R4 => write!(f,"{}", "R4"),
            Registers::R5 => write!(f,"{}", "R5"),
            Registers::R6 => write!(f,"{}", "R6"),
            Registers::R7 => write!(f,"{}", "R7"),
            Registers::R8 => write!(f,"{}", "R8"),
        }
    }
}

#[derive(Debug, PartialEq, PartialOrd)]
pub enum Value {
    Reg(Registers), Num(i32)
}

impl Value {
    pub fn get_val(&self, c: &Context) -> Result<i32, String> {
        match self {
            Value::Num(nb) => Ok(*nb),
            Value::Reg(r) => r.get_val(c),
        }
    }
}