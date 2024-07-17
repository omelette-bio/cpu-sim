use crate::error::EvalError;

#[derive(Debug)]
pub struct Stack {
    stack: Vec<i32>
}

impl Stack {
    pub fn get_len(&self) -> usize { self.stack.len() }
    pub fn pop_value(&mut self) -> Result<i32, EvalError> {
        if self.get_len() == 0 { return Err(EvalError::EmptyStack); }
        Ok(self.stack.pop().unwrap())
    }
    pub fn push_value(&mut self, value: i32) { self.stack.push(value); }
}

impl Default for Stack {
    fn default() -> Self {
        Stack {
            stack: Vec::new()
        }
    }
}