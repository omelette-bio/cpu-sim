use std::fmt::{Display, Formatter};
use colored::Colorize;
use crate::data_manipulation::{Registers, Value};

#[derive(Debug ,PartialEq)]
pub enum EvalError {
    EmptyStack,
    RegisterNotInitialized(Registers),
    BranchNotInFileContext,
    DivisionByZero(Value),
    PCSegFault(usize, usize),
    NoFileInputed,
    LabelUndefined(String),
    LabelAlreadyDefined(String)
}

impl Display for EvalError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let err_begin = "Error : ".red();
        match self {
            EvalError::EmptyStack => write!(f,"{}The stack is empty", err_begin),
            EvalError::RegisterNotInitialized(r) => write!(f,"{}The register {} is not initialized", err_begin, r.to_string().yellow()),
            EvalError::BranchNotInFileContext => write!(f,"{}Cannot branch in this context !", err_begin),
            EvalError::DivisionByZero(v) => write!(f,"{}Cannot divide, {} is evaluated to zero", err_begin, v.to_string().yellow()),
            EvalError::PCSegFault(n, m) => write!(f,"{}Program counter {} is too far in the program, {} is the line of end", err_begin, n.to_string().yellow(), m.to_string().yellow()),
            EvalError::NoFileInputed => write!(f,"{}No file is inputed !", err_begin),
            EvalError::LabelUndefined(l) => write!(f,"{}The label {} is undefined", err_begin, l.to_string().yellow()),
            EvalError::LabelAlreadyDefined(l) => write!(f, "{}The label {} is already defined", err_begin, l.to_string().yellow())
        }
    }
}
