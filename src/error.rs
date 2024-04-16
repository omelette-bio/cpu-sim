use std::fmt::{Display, Formatter};
use colored::Colorize;
use crate::data_manipulation::{Registers, Value};

#[derive(Debug ,PartialEq)]
pub enum Error {
    EmptyStack,
    RegisterNotInitialized(Registers),
    BranchNotInFileContext,
    DivisionByZero(Value)
}

impl Display for Error {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::EmptyStack => write!(f,"{}The stack is empty", "Error : ".red()),
            Error::RegisterNotInitialized(r) => write!(f,"{}The register {} is not initialized", "Error : ".red(), r.to_string().yellow()),
            Error::BranchNotInFileContext => write!(f,"{}Cannot branch in this context !", "Error : ".red()),
            Error::DivisionByZero(v) => write!(f,"{}Cannot divide, {} is evaluated to zero", "Error : ".red(), v.to_string().yellow())
        }
    }
}