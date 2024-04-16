use std::fmt;
use std::fmt::Formatter;
use crate::context::Context;
use crate::error::Error;

#[derive(Debug, Eq, Hash, PartialEq, PartialOrd, Copy, Clone)]
pub enum Registers {
    R1, R2, R3, R4, R5, R6, R7, R8
}

impl Registers {
    pub fn get_val(&self, c: &Context) -> Result<i32, Error> {
        c.get(*self)
    }

    pub fn set_val(&self, value: i32, c: &mut Context) {
        c.set(*self, value)
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

#[derive(Debug, PartialEq, PartialOrd, Clone)]
pub enum Value {
    Reg(Registers), Num(i32)
}

impl Value {
    pub fn get_val(&self, c: &Context) -> Result<i32, Error> {
        match self {
            Value::Num(nb) => Ok(*nb),
            Value::Reg(r) => r.get_val(c),
        }
    }
}