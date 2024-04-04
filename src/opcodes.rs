use crate::data_manipulation::{Registers, Value};
use crate::context::Context;


#[derive(Debug, PartialOrd, PartialEq)]
pub enum OpCode {
	ADD(Value, Registers), SUB(Value, Registers), MUL(Value, Registers), DIV(Value, Registers), MOVE(Value, Registers),
	PRINTF(Registers)
}


impl OpCode {
	pub fn eval (&self, c: &mut Context) -> Result<(), String> {
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
			}
			OpCode::PRINTF(reg) => {
				println!("{} := {}", reg, reg.get_val(c)?);
				Ok(())
			}
		}
	}
}
