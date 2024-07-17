mod context;
mod error;
mod stack;
mod data_manipulation;
mod opcodes;

use crate::context::{ContextManip, FileContext, LineContext};
use crate::context::Context::File;
use crate::data_manipulation::{Registers, Value};
use crate::opcodes::OpCode;

fn main() {
    let mut fc = FileContext::default();
    fc.push_stack(3);
    fc.push_stack(6);
    println!("{:?}, {:?}", fc.pop_stack(), fc.pop_stack());

    let mut context = FileContext::default();
    let inst1 = OpCode::MOVE(Value::Num(4), Registers::R1);
    let inst2 = OpCode::MOVE(Value::Num(5), Registers::R2);
    let inst3 = OpCode::ADD(Value::Reg(Registers::R1), Registers::R2);

    inst1.eval(&mut context).expect("TODO: panic message");
    inst2.eval(&mut context).expect("");
    inst3.eval(&mut context).expect("");

    println!("R1 := {:?}", context.get_register_value(&Registers::R1));
    println!("R2 := {:?}", context.get_register_value(&Registers::R2));
}