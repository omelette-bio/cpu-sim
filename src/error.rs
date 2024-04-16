use std::fmt::{Display, Formatter};
use colored::Colorize;
use crate::data_manipulation::{Registers, Value};

#[derive(Debug ,PartialEq)]
pub enum Error {
    EmptyStack,
    RegisterNotInitialized(Registers),
    BranchNotInFileContext,
    DivisionByZero(Value),
    PCSegFault(usize, usize)
}

impl Display for Error {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let err_begin = "Error : ".red();
        match self {
            Error::EmptyStack => write!(f,"{}The stack is empty", err_begin),
            Error::RegisterNotInitialized(r) => write!(f,"{}The register {} is not initialized", err_begin, r.to_string().yellow()),
            Error::BranchNotInFileContext => write!(f,"{}Cannot branch in this context !", err_begin),
            Error::DivisionByZero(v) => write!(f,"{}Cannot divide, {} is evaluated to zero", err_begin, v.to_string().yellow()),
            Error::PCSegFault(n, m) => write!(f,"{}Program counter {} is too far in the program, {} is the line of end", err_begin, n.to_string().yellow(), m.to_string().yellow())
        }
    }
}