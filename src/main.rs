mod context;
mod error;
mod stack;
mod data_manipulation;
mod opcodes;
mod eval;

use std::process::exit;
use crate::context::{ContextManip, FileContext, LineContext};
use crate::context::Context::File;
use crate::data_manipulation::{Registers, Value};
use crate::opcodes::OpCode;

use eval::eval_file;
use crate::eval::fetch_label;

fn main() {
    let mut fc = FileContext::default();
    
    let mut context = FileContext::default();
    
    let program = vec!(
        OpCode::LABEL(String::from("zizi")),
        OpCode::MOVE(Value::Num(4), Registers::R1),
        OpCode::MOVE(Value::Num(5), Registers::R2),
        OpCode::ADD(Value::Reg(Registers::R1), Registers::R2),
        OpCode::LABEL(String::from("zizi")),
        OpCode::GOTO(String::from("zizi"))
    );

    let res0 = fetch_label(&program, &mut context);
    if let Err(m) = res0 { println!("{}", m); exit(1); }
    
    for i in 0..program.len() {
        let res = eval_file(&program[i], &mut context);
        if let Err(m) = res { println!("{}", m); exit(1); }
    }

    println!("R1 := {:?}", context.get_register_value(&Registers::R1));
    println!("R2 := {:?}", context.get_register_value(&Registers::R2));
}
