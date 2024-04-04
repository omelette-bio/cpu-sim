use std::io::{self, BufRead, Write};
use std::env;
use std::fs;

use parsing::utils;
mod parsing;

mod data_manipulation;
mod opcodes;
use context::Context;
mod context;




fn prompt() {
    print!("assembly # ");
    io::stdout().flush().unwrap();
}

fn interp_input(input: String, context: &mut Context) {
    let res = utils::parse_line(input.as_str());
    if let Err(m) = res { println!("wrong command: \n{}", m) }
    else {
        let res = res.unwrap();
        for opc in res {
            let res2 = opc.eval(context);
            if let Err(m) = res2 { println!("{}", m); }
        }
    }
}

fn main() {
    let mut c = Context::new();
    let args: Vec<String> = env::args().collect();

    // if no file is provided
    if args.len() == 1 {
        let stdin = io::stdin();
        loop {
            prompt();
            let line = stdin.lock().lines().next().unwrap().unwrap();
            interp_input(line, &mut c);
        }
    }
    else {
        let filename = args.get(1).unwrap();
        c.change_file_context();
        let contents = fs::read_to_string(filename).expect("Should have been able to read the file");
        interp_input(contents, &mut c);
    }
}

#[cfg(test)]
mod tests {
    use crate::opcodes::OpCode;
    use crate::parsing::utils;
    use crate::parsing::utils::parse_line;
    use crate::data_manipulation::*;
    use crate::context::Context;

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
    fn test_parsing_one_line() {
        let res = parse_line("MOVE $1 R1, ADD R1 R1");
        assert_eq!(Ok(vec![ OpCode::MOVE(Value::Num(1), Registers::R1), OpCode::ADD(Value::Reg(Registers::R1), Registers::R1) ]), res)
    }
}
