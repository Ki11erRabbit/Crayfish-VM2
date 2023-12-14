use std::sync::Arc;
use malachite::Natural;
use crate::instruction::{ComparisonType, Condition, Instruction, JumpTarget};
use crate::machine::call_main;
use crate::machine::core::Core;
use crate::program::module::Module;
use crate::value::function::Function;

pub mod value;
pub mod instruction;
pub mod stack;
pub mod program;
pub mod machine;


fn dp_fib() -> Function {
    use instruction::RealInstruction::*;
    let instructions = Box::new([
        Instruction::new(IntegerNew(0.into())),
        Instruction::new(Store("a".to_string().into())),
        Instruction::new(IntegerNew(1.into())),
        Instruction::new(Store("b".to_string().into())),
        Instruction::new(IntegerNew(2.into())),
        Instruction::new(Store("i".to_string().into())),
        Instruction::new(Lookup("i".to_string().into())),
        Instruction::new(IntegerNew(40.into())),
        Instruction::new(Compare(ComparisonType::Equal)),
        Instruction::new(Return(Condition::Equal)),
        Instruction::new(Pop),
        Instruction::new(Pop),
        Instruction::new(Lookup("b".to_string().into())),
        Instruction::new(Lookup("a".to_string().into())),
        Instruction::new(IntegerAdd),
        Instruction::new(Lookup("b".to_string().into())),
        Instruction::new(Store("a".to_string().into())),
        Instruction::new(Store("b".to_string().into())),
        Instruction::new(Lookup("i".to_string().into())),
        Instruction::new(IntegerNew(1.into())),
        Instruction::new(IntegerAdd),
        Instruction::new(Store("i".to_string().into())),
        Instruction::new(Goto(JumpTarget::Relative(-16), Condition::Always)),
    ]);

    Function::new(instructions, Box::new([]))
}



fn main() {
    let mut module = Module::default();

    module.add_function("main", dp_fib());

    let mut core = Core::new();

    let module = Arc::new(module);

    match call_main(&mut core, module.clone()) {
        Ok(_) => {}
        Err(e) => {
            println!("Error: {:?}", e);
        }
    }
}
