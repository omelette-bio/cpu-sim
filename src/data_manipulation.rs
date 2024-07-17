use std::fmt;
use std::fmt::Formatter;
use crate::context::ContextManip;
use crate::error::EvalError;
// use crate::context::ContextManip;
// use crate::error::EvalError;

#[derive(Debug, Eq, Hash, PartialEq, PartialOrd, Copy, Clone)]
pub enum Registers {
    R1, R2, R3, R4, R5, R6, R7, R8
}

impl Registers {
    pub fn get_val<C>(&self, c: &C) -> Result<i32, EvalError> where C : ContextManip {
        c.get_register_value(self)
    }

    pub fn set_val<C>(&self, value: i32, c: &mut C) where C : ContextManip {
        c.set_register_value(self, value)
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
    pub fn get_val<C>(&self, c: &C) -> Result<i32, EvalError> where C : ContextManip {
        match self {
            Value::Reg(r) => r.get_val(c),
            Value::Num(nb) => Ok(*nb),
        }
    }
}

impl fmt::Display for Value {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Value::Reg(r) => write!(f,"{}",r),
            Value::Num(n) => write!(f,"{}",n)
        }
    }
}