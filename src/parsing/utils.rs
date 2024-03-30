use pest_derive::Parser;
use pest::Parser;
use pest::error::Error;

#[derive(Parser)]
#[grammar = "parsing/grammar.pest"]
pub struct IdentParser;

pub fn parse_line(input: &str) -> Result<(),Error<Rule>> {
    let pairs = IdentParser::parse(Rule::command, input)?;
    for pair in pairs {
        for inner_pair in pair.into_inner() {
            match inner_pair.as_rule() {
                Rule::calc_opcode => println!("OpCode:   {}", inner_pair.as_str()),
                Rule::data => println!("Data:     {}", inner_pair.as_str()),
                Rule::register => println!("Register: {}", inner_pair.as_str()),
                _ => unreachable!()
            };
        }
    }
    Ok(())
}