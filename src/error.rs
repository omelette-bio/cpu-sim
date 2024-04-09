use std::fmt::{Display, Formatter};
use crate::data_manipulation::Registers;

#[derive(Debug ,PartialEq)]
pub enum Error {
    EmptyStack,
    RegisterNotInitialized(Registers),
}

impl Display for Error {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::EmptyStack => write!(f,"the stack is empty"),
            Error::RegisterNotInitialized(r) => write!(f,"the register {} is not initialized", r)
        }
    }
}