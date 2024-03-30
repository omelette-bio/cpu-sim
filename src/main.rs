use std::io::{self, BufRead, Write};

use crate::registers::{Context, Registers, Value};
mod registers;

use crate::opcodes::OpCode;
mod opcodes;

use crate::parsing::utils;
mod parsing;


fn prompt() {
    print!("assembly # ");
    io::stdout().flush().unwrap();
}

fn main() {
    // loop {
    //     prompt();
    //     let stdin = io::stdin();
    //     let line = stdin.lock().lines().next().unwrap().unwrap();
    //     let res = utils::parse_line(line.as_str());
    //     if let Err(m) = res { println!("wrong command: \n{}", m)}
    // }

    // let mut c = Context::new();
    // let res = OpCode::MOVE(Value::Num(8), Registers::R1).eval(&mut c);
    // println!("{:?}", res);
    // println!("{:?}", c);
}

#[cfg(test)]
mod tests {
    use crate::registers::*;
    use crate::opcodes::OpCode;
    use crate::parsing::utils;

    #[test]
    fn move_int_reg() {
        let mut c = Context::new();
        let _ = OpCode::MOVE(Value::Num(12), Registers::R1).eval(&mut c);
        assert_eq!(Ok(12), Registers::R1.get_val(&c));
    }

    #[test]
    fn add_two_reg() {
        let mut c = Context::new();
        let _ = OpCode::MOVE(Value::Num(8), Registers::R1).eval(&mut c);
        let _ = OpCode::MOVE(Value::Num(6), Registers::R2).eval(&mut c);
        let _ = OpCode::ADD(Value::Reg(Registers::R1), Registers::R2).eval(&mut c);
        assert_eq!(Ok(14), Registers::R2.get_val(&c));
    }

    #[test]
    fn add_two_reg_not_init() {
        let mut c = Context::new();
        let _ = OpCode::MOVE(Value::Num(8), Registers::R1).eval(&mut c);
        let _ = OpCode::ADD(Value::Reg(Registers::R1), Registers::R2).eval(&mut c);
        assert_ne!(Ok(8), Registers::R2.get_val(&c));
    }

    #[test]
    fn test_parse1() {
        let res = utils::parse_line("ADD $1 R1");
        assert_eq!(Ok(()), res);
    }

    #[test]
    fn test_parse2() {
        let res = utils::parse_line("ADD R1 R1");
        assert_eq!(Ok(()), res);
    }

    #[test]
    fn test_parse3() {
        let res = utils::parse_line("ADD R1 R9");
        assert_ne!(Ok(()), res);
    }
}
