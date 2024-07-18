mod context;
mod error;
mod stack;
mod data_manipulation;
mod opcodes;
mod eval;

use crate::context::{ContextManip, FileContext, LineContext};
use crate::context::Context::File;
use crate::data_manipulation::{Registers, Value};
use crate::opcodes::OpCode;

use eval::eval_file;

fn main() {
    let mut fc = FileContext::default();
    
    let mut context = FileContext::default();
    
    let program = vec!(
        OpCode::MOVE(Value::Num(4), Registers::R1), 
        OpCode::MOVE(Value::Num(5), Registers::R2), 
        OpCode::ADD(Value::Reg(Registers::R1), Registers::R2),
        OpCode::GOTO(String::from("zizi"))
    );

    for i in 0..program.len() {
        let res = eval_file(&program[i], &mut context);
        if let Err(m) = res {println!("{}", m);}
    }

    println!("R1 := {:?}", context.get_register_value(&Registers::R1));
    println!("R2 := {:?}", context.get_register_value(&Registers::R2));
}
