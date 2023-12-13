use crate::instruction::Instruction;

pub struct Function {
    pub code: Box<[Instruction]>,
    pub argument_names: Box<[Box<str>]>,
}