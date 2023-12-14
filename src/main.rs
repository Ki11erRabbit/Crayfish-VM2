use std::sync::Arc;
use malachite::Natural;
use crate::instruction::{ComparisonType, Condition, FunctionSource, Instruction, JumpTarget};
use crate::machine::call_main;
use crate::machine::core::Core;
use crate::program::FunctionPath;
use crate::program::module::Module;
use crate::value::function::Function;
use crate::value::integer::Integer;

pub mod value;
pub mod instruction;
pub mod stack;
pub mod program;
pub mod machine;


fn dp_fib() -> Function {
    let a: u64 = 0;
    let b: u64 = 1;
    let i: u64 = 2;
    let compare: u64 = 1500;
    let one: u64 = 1;
    use instruction::RealInstruction::*;
    let instructions = Box::new([
        Instruction::new(IntegerNew(malachite::Natural::from(a).into())),
        Instruction::new(Store("a".to_string().into())),
        Instruction::new(IntegerNew(malachite::Natural::from(b).into())),
        Instruction::new(Store("b".to_string().into())),
        Instruction::new(IntegerNew(malachite::Natural::from(i).into())),
        Instruction::new(Store("i".to_string().into())),
        Instruction::new(Lookup("i".to_string().into())),
        Instruction::new(IntegerNew(malachite::Natural::from(compare).into())),
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
        Instruction::new(IntegerNew(malachite::Natural::from(one).into())),
        Instruction::new(IntegerAdd),
        Instruction::new(Store("i".to_string().into())),
        Instruction::new(Goto(JumpTarget::Relative(-16), Condition::Always)),
    ]);

    Function::new(instructions, Box::new([]))
}

fn rec_fib() -> Function {
    let one: u64 = 1;
    use instruction::RealInstruction::*;
    let instructions = Box::new([
        Instruction::new(Lookup("n".to_string().into())),
        Instruction::new(IntegerNew(Integer::Natural(malachite::Natural::from(one).into()))),
        Instruction::new(Compare(ComparisonType::LessThanOrEqual)),
        Instruction::new(Goto(JumpTarget::Relative(3), Condition::GreaterThan)),
        Instruction::new(Pop),
        Instruction::new(Return(Condition::Always)),
        Instruction::new(Pop),
        Instruction::new(IntegerNew(Integer::Natural(malachite::Natural::from(one).into()))),
        Instruction::new(IntegerSubtract),
        Instruction::new(Store("n".to_string().into())),
        Instruction::new(Lookup("n".to_string().into())),
        Instruction::new(FunctionCall(FunctionSource::Name(<&str as Into<FunctionPath>>::into("fib")), Condition::Always)),
        Instruction::new(Lookup("n".to_string().into())),
        Instruction::new(IntegerNew(Integer::Natural(malachite::Natural::from(one).into()))),
        Instruction::new(IntegerSubtract),
        Instruction::new(FunctionCall(FunctionSource::Name(<&str as Into<FunctionPath>>::into("fib")), Condition::Always)),
        Instruction::new(IntegerAdd),
        Instruction::new(Return(Condition::Always)),
    ]);

    Function::new(instructions, Box::new(["n".to_string().into_boxed_str()]))
}

fn rec_fib_main() -> Function {
    let ten: u64 = 10;
    use instruction::RealInstruction::*;
    let instructions = Box::new([
        Instruction::new(IntegerNew(Integer::Natural(malachite::Natural::from(ten).into()))),
        Instruction::new(FunctionCall(FunctionSource::Name(<&str as Into<FunctionPath>>::into("fib")), Condition::Always)),
        Instruction::new(Return(Condition::Always)),
    ]);

    Function::new(instructions, Box::new([]))
}



fn main() {
    let mut module = Module::default();

    module.add_function("main", dp_fib());
    module.add_function("fib", rec_fib());
    module.add_function("main", rec_fib_main());

    let mut core = Core::new();

    let module = Arc::new(module);

    match call_main(&mut core, module.clone()) {
        Ok(_) => {}
        Err(e) => {
            println!("Error: {:?}", e);
        }
    }
}
