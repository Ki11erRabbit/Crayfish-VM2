use crate::program::FunctionPath;

pub mod core;


#[derive(Debug)]
pub enum Fault {
    DivisionByZero,
    StackOverflow,
    Overflow,
    Underflow,
    InvalidInstruction,
    InvalidRegister,
    InvalidJump,
    FunctionNotFound(FunctionPath),
    ContinuationNotFound(u64),
    InvalidString,
    InvalidOperation(String),
    MemoryError(String),
    NotAnInteger,
}