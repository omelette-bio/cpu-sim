use crate::registers::{Registers, Value, Context};


#[derive(Debug)]
pub enum OpCode {
	ADD(Value, Registers), SUB(Value, Registers), MUL(Value, Registers), DIV(Value, Registers), MOVE(Value, Registers)
}


impl OpCode {
	pub fn eval (&self, c: &mut Context) -> Result<(), String> {
		match self {
			OpCode::ADD(val, reg) => {
				reg.set_val(val.get_val(c)? + reg.get_val(c)?, c);
				Ok(())
			},
			OpCode::SUB(val, reg) => {
				reg.set_val(val.get_val(c)? - reg.get_val(c)?, c);
				Ok(())
			},
			OpCode::MUL(val, reg) => {
				reg.set_val(val.get_val(c)? * reg.get_val(c)?, c);
				Ok(())
			},
			OpCode::DIV(val, reg) => {
				reg.set_val(val.get_val(c)? / reg.get_val(c)?, c);
				Ok(())
			},
			OpCode::MOVE(val, reg) => {
				let l_val = val.get_val(c)?;
				reg.set_val(l_val, c);
				println!("value {} moved into {}", l_val, reg);
				Ok(())
			}
		}
	}
}
