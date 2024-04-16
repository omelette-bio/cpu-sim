use std::fmt::{Display, Formatter};
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
            Error::EmptyStack => write!(f,"The stack is empty"),
            Error::RegisterNotInitialized(r) => write!(f,"The register {} is not initialized", r),
            Error::BranchNotInFileContext => write!(f,"Cannot branch in this context !"),
            Error::DivisionByZero(v) => write!(f,"Cannot divide, {} is evaluated to zero", v)
        }
    }
}