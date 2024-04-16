use std::io::{self, BufRead, Write};
use std::env;
use std::fs;

use parsing::utils;
mod parsing;

mod data_manipulation;
mod opcodes;
use context::Context;

mod context;
mod error;


fn prompt() {
    print!("μAssembly # ");
    io::stdout().flush().unwrap();
}

fn interp_input(input: String, context: &mut Context) -> Result<(), ()>{
    let res = utils::parse_line(input.as_str());
    if let Err(m) = res { println!("{}", m); return Err(()); }
    if context.is_in_file() { context.change_exec_stack( res.clone().unwrap() ) }
    match context.is_in_file() {
        false => {
            for opc in res.unwrap() {
                let res2 = opc.eval(context);
                if let Err(m) = res2 { println!("{}", m); return Err(()); }
            }
        }
        true => {
            while context.get_stack_index() < context.get_exec_stack_end() {
                let r = context.get_current_command().eval(context);
                if let Err(m) = r { println!("{}", m); return Err(()); }
                context.increment_stack_index();
            }
        }
    }
    Ok(())
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
            let _ = interp_input(line, &mut c);
        }
    }
    else {
        let filename = args.get(1).unwrap();
        c.change_file_context();
        let contents = fs::read_to_string(filename).expect("Should have been able to read the file");
        let _ = interp_input(contents, &mut c);
    }
}

#[cfg(test)]
mod tests {
    use std::fs;
    use crate::opcodes::OpCode;
    use crate::parsing::utils::parse_line;
    use crate::data_manipulation::*;
    use crate::context::Context;
    use crate::interp_input;

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
        let res = parse_line("MOVE $1 R1; ADD R1 R1");
        assert_eq!(Ok(vec![ OpCode::MOVE(Value::Num(1), Registers::R1), OpCode::ADD(Value::Reg(Registers::R1), Registers::R1) ]), res)
    }

    #[test]
    fn test_parsing_one_line_error_digit() {
        let res = parse_line("MOVE 1 R1; ADD R1 R1");
        assert!(matches!(res, Err(_)))
    }

    #[test]
    fn test_parsing_one_line_register_out_of_bounds() {
        let res = parse_line("MOVE $1 R9");
        assert!(matches!(res, Err(_)))
    }

    #[test]
    fn test_value_of_r1_end_of_test_file_1() {
        let mut c = Context::new();
        c.change_file_context();
        let contents = fs::read_to_string("test/test1.a").expect("Should have been able to read the file");
        let _ = interp_input(contents, &mut c);
        assert_eq!(6, Registers::R1.get_val(&c).unwrap())
    }

    #[test]
    fn test_value_of_r1_end_of_test_file_2() {
        let mut c = Context::new();
        c.change_file_context();
        let contents = fs::read_to_string("test/test2.a").expect("Should have been able to read the file");
        let _ = interp_input(contents, &mut c);
        assert_eq!(6, Registers::R1.get_val(&c).unwrap())
    }

    #[test]
    fn test_of_test_file_3() {
        let mut c = Context::new();
        c.change_file_context();
        let contents = fs::read_to_string("test/test3.a").expect("Should have been able to read the file");
        let res = interp_input(contents, &mut c);
        assert!(matches!(res, Err(_)))
    }
}
