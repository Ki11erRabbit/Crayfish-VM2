use std::collections::HashMap;
use std::sync::Arc;
use crate::machine::core::Core;
use crate::program::FunctionPath;
use crate::program::module::Module;
use crate::value::function::Function;
use crate::value::Value;

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

#[derive(Debug)]
pub enum InstructionResult {
    Stop,
    Continue,
    Return,
    Unwind(Box<str>),
    Call(Function),
}

pub fn call_main(core: &mut Core, module: Arc<Module>) -> Result<(), Fault> {
    let function = module.get_function(&<&str as Into<FunctionPath>>::into("main"));
    match function {
        None => {
            Err(Fault::FunctionNotFound(<&str as Into<FunctionPath>>::into("main")))
        }
        Some(function) => {
            let environment = HashMap::new();
            match call_function(core, module.clone(), function, environment)? {
                InstructionResult::Stop => {
                    Ok(())
                }
                InstructionResult::Continue => {
                    Ok(())
                }
                InstructionResult::Return => {
                    Ok(())
                }
                InstructionResult::Unwind(exn) => {
                    todo!("Add exn handling code")
                }
                _ => panic!("Invalid instruction result"),
            }
        }
    }
}



fn call_function(core: &mut Core, module: Arc<Module>, function: &Function, mut environment: HashMap<Box<str>, Value>) -> Result<InstructionResult,Fault> {
    let mut program_counter = 0;
    let mut instruction = function.get_instruction(program_counter);

    loop {

        match core.execute_instruction(instruction, &mut program_counter, &mut environment)? {
            InstructionResult::Stop => {
                return Ok(InstructionResult::Stop)
            }
            InstructionResult::Continue => {
                instruction = function.get_instruction(program_counter);
            }
            InstructionResult::Return => {
                return Ok(InstructionResult::Continue)
            }
            InstructionResult::Unwind(exn) => {
                todo!("Add exn handling code")
            }
            InstructionResult::Call(function) => {
                todo!("Add code for calling a function")
            }
        }
    }
}