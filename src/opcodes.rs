use crate::data_manipulation::{Registers, Value};
use crate::context::Context;
use crate::error::Error;


#[derive(Debug, PartialOrd, PartialEq)]
pub enum OpCode {
	ADD(Value, Registers), SUB(Value, Registers), MUL(Value, Registers), DIV(Value, Registers), MOVE(Value, Registers),
	AND(Value, Registers), OR(Value, Registers), NOT(Registers),
	POP(Registers), PUSH(Registers),
	PRINTF(Registers)
}
// AND OR NOT TLT TEQ Jump BNEZ BEZ POP PUSH

impl OpCode {
	pub fn eval (&self, c: &mut Context) -> Result<(), Error> {
		match self {
			OpCode::ADD(val, reg) => {
				let l_val = val.get_val(c)?;
				reg.set_val(reg.get_val(c)? + l_val, c);
				if !c.is_in_file() { println!("{} := {}", reg, l_val); }
				Ok(())
			},
			OpCode::SUB(val, reg) => {
				let l_val = val.get_val(c)?;
				reg.set_val(reg.get_val(c)? - l_val, c);
				if !c.is_in_file() { println!("{} := {}", reg, l_val); }
				Ok(())
			},
			OpCode::MUL(val, reg) => {
				let l_val = val.get_val(c)?;
				reg.set_val(reg.get_val(c)? * l_val, c);
				if !c.is_in_file() { println!("{} := {}", reg, l_val); }
				Ok(())
			},
			OpCode::DIV(val, reg) => {
				let l_val = val.get_val(c)?;
				reg.set_val(reg.get_val(c)? / l_val, c);
				if !c.is_in_file() { println!("{} := {}", reg, l_val); }
				Ok(())
			},
			OpCode::MOVE(val, reg) => {
				let l_val = val.get_val(c)?;
				reg.set_val(l_val, c);
				if !c.is_in_file() { println!("{} := {}", reg, l_val); }
				Ok(())
			},
			OpCode::AND(val, reg) => {
				let l_val = val.get_val(c)?;
				reg.set_val(reg.get_val(c)? & l_val, c);
				if !c.is_in_file() { println!("{} := {}", reg, l_val); }
				Ok(())
			},
			OpCode::OR(val, reg) => {
				let l_val = val.get_val(c)?;
				reg.set_val(reg.get_val(c)? | l_val, c);
				if !c.is_in_file() { println!("{} := {}", reg, l_val); }
				Ok(())
			},
			OpCode::NOT(reg) => {
				reg.set_val(!reg.get_val(c)?, c);
				if !c.is_in_file() { println!("{} := {}", reg, reg.get_val(&c)?); }
				Ok(())
			},
			OpCode::PRINTF(reg) => {
				println!("{} := {}", reg, reg.get_val(c)?);
				Ok(())
			},
			OpCode::POP(reg) => {
				reg.set_val(c.pop_stack()?, c);
				if !c.is_in_file() { println!("{} := {}", reg, reg.get_val(&c)?); }
				Ok(())
			},
			OpCode::PUSH(reg) => {
				c.push_stack(reg.get_val(c)?);
				Ok(())
			}
		}
	}
}
