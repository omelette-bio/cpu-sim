use std::collections::HashMap;
use pest_derive::Parser;
use pest::Parser;
use pest::error::Error;
use crate::opcodes::OpCode;
use crate::data_manipulation::{Registers, Value};

#[derive(Parser)]
#[grammar = "parsing/grammar.pest"]
pub struct IdentParser;


fn interp(pair: pest::iterators::Pair<Rule>) -> OpCode {
    match pair.as_rule() {
        Rule::command => {
            let mut pair = pair.into_inner();
            let opc = pair.next().unwrap();
            match opc.as_str() {
                "ADD" => OpCode::ADD( interp_val( pair.next().unwrap()), interp_reg(pair.next().unwrap() )),
                "SUB" => OpCode::SUB( interp_val( pair.next().unwrap()), interp_reg(pair.next().unwrap() )),
                "MUL" => OpCode::MUL( interp_val( pair.next().unwrap()), interp_reg(pair.next().unwrap() )),
                "DIV" => OpCode::DIV( interp_val( pair.next().unwrap()), interp_reg(pair.next().unwrap() )),
                "MOVE" => OpCode::MOVE( interp_val( pair.next().unwrap()), interp_reg(pair.next().unwrap() )),
                "printf" => OpCode::PRINTF( interp_reg( pair.next().unwrap() )),
                _ => panic!("no1")
            }
        }
        _ => panic!("no2")
    }
}

fn interp_val(pair: pest::iterators::Pair<Rule>) -> Value {
    let reg_map: HashMap<&str, Registers> = HashMap::from( [("R1", Registers::R1),("R2", Registers::R2),("R3", Registers::R3),("R4", Registers::R4),("R5", Registers::R5),("R6", Registers::R6),("R7", Registers::R7),("R8", Registers::R8)] );
    match pair.as_rule() {
        Rule::digit => { Value::Num(pair.as_str()[1..].parse::<i32>().unwrap()) },
        Rule::register => { Value::Reg( *reg_map.get(pair.as_str()).unwrap() ) },
        _ => {
            println!("{:?}", pair.as_str());
            panic!("no3");
        }
    }
}

fn interp_reg(pair: pest::iterators::Pair<Rule>) -> Registers {
    let reg_map: HashMap<&str, Registers> = HashMap::from( [("R1", Registers::R1),("R2", Registers::R2),("R3", Registers::R3),("R4", Registers::R4),("R5", Registers::R5),("R6", Registers::R6),("R7", Registers::R7),("R8", Registers::R8)] );
    match pair.as_rule() {
        Rule::register => {
            *reg_map.get(pair.as_str()).unwrap()
        }
        _ => panic!("no4")
    }
}

pub fn parse_line(input: &str) -> Result<Vec<OpCode>,Error<Rule>> {
    let mut op_v = Vec::new();

    let mut pairs = IdentParser::parse(Rule::command, input)?;


    let mut pair = pairs.next();
    while pair != Option::None {
        op_v.push(interp(pair.clone().unwrap()));
        pair = pair.unwrap().into_inner().nth(3);
    }

    Ok(op_v)
}