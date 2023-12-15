use std::fmt::{Debug, Display};
use crate::instruction::Instruction;
use crate::machine::environment::Environment;
use crate::stack::StackChunk;

#[derive(Clone)]
pub struct Function {
    pub code: Box<[Instruction]>,
    pub argument_names: Box<[Box<str>]>,
    pub environment: Environment,
}


impl Function {

    pub fn new(code: Box<[Instruction]>, argument_names: Box<[Box<str>]>) -> Self {
        Function {
            code,
            argument_names,
            environment: Environment::new(),
        }
    }

    pub fn empty() -> Self {
        Function {
            code: Box::new([]),
            argument_names: Box::new([]),
            environment: Environment::new(),
        }
    }

    pub fn add_environment(&mut self, environment: Environment) {
        self.environment = environment;
    }

    pub fn get_environment(&self) -> &Environment {
        &self.environment
    }
    pub fn get_instruction(&self, index: usize) -> &Instruction {
        &self.code[index]
    }

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