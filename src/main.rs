use std::io::{self, BufRead, Write};

use crate::registers::{Context, Registers, Value};
mod registers;

use crate::opcodes::OpCode;
mod opcodes;

fn prompt() {
    print!("assembly # ");
    io::stdout().flush().unwrap();
}

fn main() {
    let mut c = Context::new();
    OpCode::MOVE(12, Registers::R1).eval(&mut c);
    OpCode::MOVE(12, Registers::R2).eval(&mut c);
    OpCode::ADD(Value::Num(12), Registers::R1).eval(&mut c);
    OpCode::ADD(Value::Reg(Registers::R1), Registers::R2).eval(&mut c);
    println!("{:?}", Value::Reg(Registers::R1).get_val(&c));
    println!("{:?}", c);
}

#[cfg(test)]
mod tests {
    use crate::registers::*;
    use crate::opcodes::OpCode;

    #[test]
    fn move_int_reg() {
        let mut c = Context::new();
        OpCode::MOVE(12, Registers::R1).eval(&mut c);
        assert_eq!(Ok(12), Registers::R1.get_val(&c));
    }

    #[test]
    fn add_two_reg() {
        let mut c = Context::new();
        OpCode::MOVE(6, Registers::R1).eval(&mut c);
        OpCode::MOVE(8, Registers::R2).eval(&mut c);
        OpCode::ADD(Value::Reg(Registers::R1), Registers::R2).eval(&mut c);
        assert_eq!(Ok(14), Registers::R2.get_val(&c));
    }
}
