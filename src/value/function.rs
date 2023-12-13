use std::fmt::{Debug, Display};
use crate::instruction::Instruction;
use crate::stack::StackChunk;

#[derive(Clone)]
pub struct Function {
    pub code: Box<[Instruction]>,
    pub argument_names: Box<[Box<str>]>,
}


impl Function {

    pub fn into_chunk(self) -> Box<dyn StackChunk> {
        Box::new(self)
    }
}


impl Display for Function {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut code = String::new();
        code.push_str("fn(");
        for argument in self.argument_names.iter() {
            code.push_str(&format!("{}, ", argument));
        }

        code.push_str(") {\n");

        for instruction in self.code.iter() {
            code.push_str(&format!("{}, \n", instruction));
        }

        code.push_str("}");
        write!(f, "{}", code)
    }
}


impl Debug for Function {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut code = String::new();
        code.push_str("fn(");
        for argument in self.argument_names.iter() {
            code.push_str(&format!("{}, ", argument));
        }

        code.push_str(") {\n");

        for instruction in self.code.iter() {
            code.push_str(&format!("{:?}, \n", instruction));
        }

        code.push_str("}");
        write!(f, "{}", code)
    }
}