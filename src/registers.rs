use std::collections::HashMap;



#[derive(Debug)]
pub struct Context {
    register_table: HashMap<Registers, i32>,
}

impl Context {
    pub fn new() -> Self {
        Context { register_table: HashMap::new() }
    }
}

#[derive(Debug, Eq, Hash, PartialEq, Copy, Clone)]
pub enum Registers {
    R1, R2, R3, R4, R5, R6, R7, R8
}

impl Registers {
    pub fn get_val(&self, c: &Context) -> Result<i32, ()> {
        match c.register_table.get(self) {
            Some(value) => Ok(*value),
            None => Err(())
        }
    }

    pub fn set_val(&self, value: i32, c: &mut Context) {
        c.register_table.insert(*self, value);
    }
}

pub enum Value {
    Reg(Registers), Num(i32)
}

impl Value {
    pub fn get_val(&self, c: &Context) -> Result<i32, ()> {
        match self {
            Value::Num(nb) => Ok(*nb),
            Value::Reg(r) => r.get_val(c),
        }
    }
}