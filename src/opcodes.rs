use crate::registers::{Registers, Value, Context};


#[derive(Debug)]
pub enum OpCode {
	ADD(Value, Registers), SUB(Value, Registers), MUL(Value, Registers), DIV(Value, Registers), MOVE(Value, Registers)
}


impl OpCode {
	pub fn eval (&self, c: &mut Context) -> Result<(), String> {
		match self {
			OpCode::ADD(val, reg) => {
				let l_val = val.get_val(c)?;
				reg.set_val(reg.get_val(c)? + l_val, c);
				println!("{} := {}", reg, reg.get_val(c)?);
				Ok(())
			},
			OpCode::SUB(val, reg) => {
				let l_val = val.get_val(c)?;
				reg.set_val(reg.get_val(c)? - l_val, c);
				println!("{} := {}", reg, reg.get_val(c)?);
				Ok(())
			},
			OpCode::MUL(val, reg) => {
				let l_val = val.get_val(c)?;
				reg.set_val(reg.get_val(c)? * l_val, c);
				println!("{} := {}", reg, reg.get_val(c)?);
				Ok(())
			},
			OpCode::DIV(val, reg) => {
				let l_val = val.get_val(c)?;
				reg.set_val(reg.get_val(c)? / l_val, c);
				println!("{} := {}", reg, reg.get_val(c)?);
				Ok(())
			},
			OpCode::MOVE(val, reg) => {
				let l_val = val.get_val(c)?;
				reg.set_val(l_val, c);
				println!("{} := {}", reg, l_val);
				Ok(())
			}
		}
	}
}
