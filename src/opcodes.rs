use crate::data_manipulation::{Registers, Value};
use crate::context::{ContextManip};
use crate::error::EvalError;

#[derive(Debug, PartialOrd, PartialEq , Clone)]
pub enum OpCode {
	ADD(Value, Registers), SUB(Value, Registers), MUL(Value, Registers), DIV(Value, Registers), MOVE(Value, Registers),
	AND(Value, Registers), OR(Value, Registers), NOT(Registers),
	POP(Registers), PUSH(Registers),
	TLT(Value, Registers), TEQ(Value, Registers), TLE(Value, Registers),
	JUMP(Value), BEZ(Value, Registers), BNEZ(Value, Registers), GOTO(String), LABEL(String),
	PRINTF(Registers)
}

impl OpCode {
	pub fn eval<C> (&self, c: &mut C) -> Result<(), EvalError>
	where C: ContextManip
	{
		match self {
			OpCode::ADD(val, reg) => {
				let l_val = val.get_val(c)?;
				reg.set_val(reg.get_val(c)? + l_val, c);
				Ok(())
			}
			OpCode::MOVE(val, reg) => {
				reg.set_val(val.get_val(c)?, c);
				Ok(())
			}
			_ => todo!()
		}
	}
}

//
// impl OpCode {
// 	fn continue_eval(c: &mut Context, reg: &Registers) -> Result<(), Error> {
// 		if !c.is_in_file() { println!("{} := {}", reg, reg.get_val(c)?); }
// 		else { c.increment_program_counter(); }
// 		Ok(())
// 	}
//
// 	pub fn eval (&self, c: &mut Context) -> Result<(), Error> {
// 		match self {
// 			OpCode::ADD(val, reg) => {
// 				let l_val = val.get_val(c)?;
// 				reg.set_val(reg.get_val(c)? + l_val, c);
// 				OpCode::continue_eval(c, reg)
// 			},
// 			OpCode::SUB(val, reg) => {
// 				let l_val = val.get_val(c)?;
// 				reg.set_val(reg.get_val(c)? - l_val, c);
// 				OpCode::continue_eval(c, reg)
// 			},
// 			OpCode::MUL(val, reg) => {
// 				let l_val = val.get_val(c)?;
// 				reg.set_val(reg.get_val(c)? * l_val, c);
// 				OpCode::continue_eval(c, reg)
// 			},
// 			OpCode::DIV(val, reg) => {
// 				let l_val = val.get_val(c)?;
// 				if l_val == 0 { return Err(Error::DivisionByZero(val.clone())) }
// 				reg.set_val(reg.get_val(c)? / l_val, c);
// 				OpCode::continue_eval(c, reg)
// 			},
// 			OpCode::MOVE(val, reg) => {
// 				let l_val = val.get_val(c)?;
// 				reg.set_val(l_val, c);
// 				OpCode::continue_eval(c, reg)
// 			},
// 			OpCode::AND(val, reg) => {
// 				let l_val = val.get_val(c)?;
// 				reg.set_val(reg.get_val(c)? & l_val, c);
// 				OpCode::continue_eval(c, reg)
// 			},
// 			OpCode::OR(val, reg) => {
// 				let l_val = val.get_val(c)?;
// 				reg.set_val(reg.get_val(c)? | l_val, c);
// 				OpCode::continue_eval(c, reg)
// 			},
// 			OpCode::NOT(reg) => {
// 				reg.set_val(!reg.get_val(c)?, c);
// 				OpCode::continue_eval(c, reg)
// 			},
// 			OpCode::PRINTF(reg) => {
// 				println!("{} := {}", reg, reg.get_val(c)?);
// 				if c.is_in_file() { c.increment_program_counter(); }
// 				Ok(())
// 			},
// 			OpCode::POP(reg) => {
// 				reg.set_val(c.pop_stack()?, c);
// 				OpCode::continue_eval(c, reg)
// 			},
// 			OpCode::PUSH(reg) => {
// 				c.push_stack(reg.get_val(c)?);
// 				if c.is_in_file() { c.increment_program_counter(); }
// 				Ok(())
// 			}
// 			OpCode::JUMP(val) => {
// 				if !c.is_in_file() { return Err( Error::BranchNotInFileContext ) }
// 				c.set_program_counter( (c.get_program_counter() as isize + val.get_val(c)? as isize) as usize )?;
// 				Ok(())
// 			}
// 			OpCode::TLT(val, reg) => {
// 				if val.get_val(c)? < reg.get_val(c)? { reg.set_val(1, c);}
// 				else { reg.set_val(0, c);}
// 				c.increment_program_counter();
// 				Ok(())
// 			}
// 			OpCode::TEQ(val, reg) => {
// 				if val.get_val(c)? == reg.get_val(c)? { reg.set_val(1, c);}
// 				else { reg.set_val(0, c);}
// 				c.increment_program_counter();
// 				Ok(())
// 			}
// 			OpCode::TLE(val, reg) => {
// 				if val.get_val(c)? <= reg.get_val(c)? { reg.set_val(1, c);}
// 				else { reg.set_val(0, c);}
// 				c.increment_program_counter();
// 				Ok(())
// 			}
// 			OpCode::BEZ(val, reg) => {
// 				if !c.is_in_file() { return Err( Error::BranchNotInFileContext ) }
// 				if reg.get_val(c)? == 0 { OpCode::JUMP(val.clone()).eval(c)?; }
// 				else { c.increment_program_counter(); }
// 				Ok(())
// 			}
// 			OpCode::BNEZ(val, reg) => {
// 				if !c.is_in_file() { return Err( Error::BranchNotInFileContext ) }
// 				if reg.get_val(c)? != 0 { OpCode::JUMP(val.clone()).eval(c)?; }
// 				else { c.increment_program_counter(); }
// 				Ok(())
// 			}
// 		}
// 	}
// }
