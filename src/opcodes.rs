use crate::data_manipulation::{Registers, Value};
use crate::context::Context;
use crate::error::Error;


#[derive(Debug, PartialOrd, PartialEq , Clone)]
pub enum OpCode {
	ADD(Value, Registers), SUB(Value, Registers), MUL(Value, Registers), DIV(Value, Registers), MOVE(Value, Registers),
	AND(Value, Registers), OR(Value, Registers), NOT(Registers),
	POP(Registers), PUSH(Registers),
	JUMP(Value),
	PRINTF(Registers)
}
// AND OR NOT TLT TEQ Jump BNEZ BEZ POP PUSH

impl OpCode {
	pub fn eval (&self, c: &mut Context) -> Result<(), Error> {
		match self {
			OpCode::ADD(val, reg) => {
				let l_val = val.get_val(c)?;
				reg.set_val(reg.get_val(c)? + l_val, c);
				if !c.is_in_file() { println!("{} := {}", reg, reg.get_val(c)?); }
				else { c.increment_stack_index(); }
				Ok(())
			},
			OpCode::SUB(val, reg) => {
				let l_val = val.get_val(c)?;
				reg.set_val(reg.get_val(c)? - l_val, c);
				if !c.is_in_file() { println!("{} := {}", reg, reg.get_val(c)?); }
				else { c.increment_stack_index(); }
				Ok(())
			},
			OpCode::MUL(val, reg) => {
				let l_val = val.get_val(c)?;
				reg.set_val(reg.get_val(c)? * l_val, c);
				if !c.is_in_file() { println!("{} := {}", reg, reg.get_val(c)?); }
				else { c.increment_stack_index(); }
				Ok(())
			},
			OpCode::DIV(val, reg) => {
				let l_val = val.get_val(c)?;
				if l_val == 0 { return Err(Error::DivisionByZero(val.clone())) }
				reg.set_val(reg.get_val(c)? / l_val, c);
				if !c.is_in_file() { println!("{} := {}", reg, reg.get_val(c)?); }
				else { c.increment_stack_index(); }
				Ok(())
			},
			OpCode::MOVE(val, reg) => {
				let l_val = val.get_val(c)?;
				reg.set_val(l_val, c);
				if !c.is_in_file() { println!("{} := {}", reg, l_val); }
				else { c.increment_stack_index(); }
				Ok(())
			},
			OpCode::AND(val, reg) => {
				let l_val = val.get_val(c)?;
				reg.set_val(reg.get_val(c)? & l_val, c);
				if !c.is_in_file() { println!("{} := {}", reg, reg.get_val(c)?); }
				else { c.increment_stack_index(); }
				Ok(())
			},
			OpCode::OR(val, reg) => {
				let l_val = val.get_val(c)?;
				reg.set_val(reg.get_val(c)? | l_val, c);
				if !c.is_in_file() { println!("{} := {}", reg, reg.get_val(c)?); }
				else { c.increment_stack_index(); }
				Ok(())
			},
			OpCode::NOT(reg) => {
				reg.set_val(!reg.get_val(c)?, c);
				if !c.is_in_file() { println!("{} := {}", reg, reg.get_val(&c)?); }
				else { c.increment_stack_index(); }
				Ok(())
			},
			OpCode::PRINTF(reg) => {
				println!("{} := {}", reg, reg.get_val(c)?);
				if c.is_in_file() { c.increment_stack_index(); }
				Ok(())
			},
			OpCode::POP(reg) => {
				reg.set_val(c.pop_stack()?, c);
				if !c.is_in_file() { println!("{} := {}", reg, reg.get_val(&c)?); }
				else { c.increment_stack_index(); }
				Ok(())
			},
			OpCode::PUSH(reg) => {
				c.push_stack(reg.get_val(c)?);
				if c.is_in_file() { c.increment_stack_index(); }
				Ok(())
			}
			OpCode::JUMP(val) => {
				if !c.is_in_file() { return Err( Error::BranchNotInFileContext ) }
				c.set_stack_index( c.get_stack_index() + val.get_val(c)? as usize );
				Ok(())
			}
		}
	}
}
