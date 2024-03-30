use crate::registers::{Registers, Value, Context};


pub enum OpCode {
	ADD(Value, Registers), SUB(Value, Registers), MUL(Value, Registers), DIV(Value, Registers), MOVE(Value, Registers)
}


impl OpCode {
	pub fn eval (&self, c: &mut Context) -> Result<(),()> {
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
			OpCode::ADD(val, reg) => {
				reg.set_val(val.get_val(c)? / reg.get_val(c)?, c);
				Ok(())
			},
			OpCode::MOVE(val, reg) => {
				reg.set_val(val.get_val(c)?, c);
				Ok(())
			},
			_ => Ok(())
		}
	}
}

// impl OpCode{
// 	pub fn eval(&self, f: &mut FileData) -> Result<(),()> {
// 		match self {
// 			OpCode::ADD(val, rd) => {
// 				let valgt = val.get_val(f);
// 				let valdt = f.get_register_value(rd);
// 				if let Err(()) = valgt { return Err(()) }
// 				let valg = valgt?;
// 				f.attribute_to_register(valg)
// 			}
// 			_ => Err(()),
// 		}
// 		Ok(())
// 	}

// 	fn get_registers_values(rg: Registers, rd: Registers, f: &FileData) -> Result<(i32,i32),()> {
// 		match (f.get_register_value(rg), f.get_register_value(rd)) {
// 			(Err(()), _) | (_, Err(())) => Err(()),
// 			(Ok(valg), Ok(vald)) => Ok((valg, vald))
// 		}
// 	}
// }