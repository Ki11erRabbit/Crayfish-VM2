use std::alloc::Layout;
use std::collections::HashMap;
use std::sync::Arc;
use malachite::num::arithmetic::traits::Pow;
use crate::instruction::{ComparisonType, Condition, FunctionSource, Instruction, JumpTarget, RealInstruction};
use crate::machine::{Fault, InstructionResult};
use crate::machine::environment::Environment;
use crate::program::module::Module;
use crate::stack::{Stack, StackChunk};
use crate::value::{Value, ValueType};
use crate::value::decimal::DecimalType;
use crate::value::integer::IntegerType;
use crate::value::tuple::Tuple;
use crate::value::vector::{Vector, VectorType};


macro_rules! basic_alu_op_int {
    ($fun_name:ident, $op:tt) => {
        fn $fun_name(&mut self) -> Result<(),Fault> {
            let right = self.stack.pop().get_boxed_value();
            let left = self.stack.pop().get_boxed_value();
            match (left, right) {
                (Value::Integer(left), Value::Integer(right)) => {
                    let result = left $op right;

                    if result.is_zero() {
                        self.flags.zero = true;
                    } else {
                        self.flags.zero = false;
                    }

                    if result.is_negative() {
                        self.flags.negative = true;
                    } else {
                        self.flags.negative = false;
                    }

                    let chunk = result.into_chunk();

                    self.stack.push(chunk);
                }
                _ => return Err(Fault::NotAnInteger),
            }


            Ok(())
        }
    };
}

macro_rules! basic_alu_op_decimal {
    ($fun_name:ident, $op:tt) => {
        fn $fun_name(&mut self) -> Result<(),Fault> {
            let right = self.stack.pop().get_boxed_value();
            let left = self.stack.pop().get_boxed_value();
            match (left, right) {
                (Value::Decimal(left), Value::Decimal(right)) => {
                    let result = left $op right;

                    if result.is_zero() {
                        self.flags.zero = true;
                    } else {
                        self.flags.zero = false;
                    }

                    if result.is_negative() {
                        self.flags.negative = true;
                    } else {
                        self.flags.negative = false;
                    }

                    let chunk = result.into_chunk();

                    self.stack.push(chunk);
                }
                _ => return Err(Fault::NotAnInteger),
            }


            Ok(())
        }
    };
}


#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Comparison {
    None,
    Equal,
    NotEqual,
    LessThan,
    LessThanOrEqual,
    GreaterThan,
    GreaterThanOrEqual,
}

#[derive(Debug, Clone, Copy, PartialEq)]
struct CoreFlags {
    comparison: Comparison,
    negative: bool,
    zero: bool,
}



pub struct Core {
    flags: CoreFlags,
    stack: Stack,
}


impl Core {
    pub fn new() -> Core {
        Core {
            flags: CoreFlags {
                comparison: Comparison::None,
                negative: false,
                zero: false,
            },
            stack: Stack::new(),
        }
    }

    pub fn execute_instruction<'a>(&mut self,
                               instruction: &Instruction,
                               program_counter: &mut usize,
                               environment: &mut Environment,
                               module: &'a Module) -> Result<InstructionResult<'a>, Fault> {
        //println!("Executing instruction: {}", instruction);
        //println!("Stack: {}", self.stack);

        use RealInstruction::*;
        match &instruction.instruction {
            Halt => return Ok(InstructionResult::Stop),
            NoOp => {},
            Push(value) => {
                let chunk = value.clone().into_chunk();
                self.stack.push(chunk);
            }
            Pop => {
                self.stack.pop();
            }
            Store(name) => {
                let value = self.stack.pop();
                environment.insert(name.clone(), value.get_boxed_value());
            },
            Lookup(name) => {
                let value = environment.get(name).unwrap().clone();
                let chunk = value.into_chunk();
                self.stack.push(chunk);
            },
            Duplicate => {
                let value = self.stack.pop();
                let value = value.get_boxed_value();
                self.stack.push(value.clone().into_chunk());
                self.stack.push(value.into_chunk());
            }
            IntegerNew(integer) => {
                let chunk = integer.clone().into_chunk();
                self.stack.push(chunk);
            }
            IntegerAdd => self.integer_add()?,
            IntegerSubtract => self.integer_subtract()?,
            IntegerMultiply => self.integer_multiply()?,
            IntegerDivide => self.integer_divide()?,
            IntegerModulo => self.integer_modulo()?,
            IntegerPower => self.integer_power()?,
            IntegerNegate => self.integer_negate()?,
            IntegerBitwiseAnd => self.integer_bitwise_and()?,
            IntegerBitwiseOr => self.integer_bitwise_or()?,
            IntegerBitwiseXor => self.integer_bitwise_xor()?,
            IntegerBitwiseNot => self.integer_bitwise_not()?,
            IntegerShiftLeft => self.integer_shift_left()?,
            IntegerShiftRight => self.integer_shift_right()?,
            Compare(comparison_type) => self.compare(comparison_type)?,
            Goto(target, condition) => return self.goto(target, condition, program_counter),
            Return(condition) => return self.return_instruction(condition, program_counter),
            FunctionCall(source, condition) => return self.function_call(source, condition, program_counter, module),
            DecimalNew(decimal) => {
                let chunk = decimal.clone().into_chunk();
                self.stack.push(chunk);
            }
            DecimalAdd => self.decimal_add()?,
            DecimalSubtract => self.decimal_subtract()?,
            DecimalMultiply => self.decimal_multiply()?,
            DecimalDivide => self.decimal_divide()?,
            DecimalModulo => self.decimal_modulo()?,
            DecimalPower => self.decimal_power()?,
            DecimalNegate => self.decimal_negate()?,
            BooleanNew(boolean) => {
                let chunk = boolean.into_chunk();
                self.stack.push(chunk);
            }
            BooleanAnd => self.boolean_and()?,
            BooleanOr => self.boolean_or()?,
            BooleanNot => self.boolean_not()?,
            VectorNew(typ) => self.vector_new(typ)?,
            VectorGet => self.vector_get()?,
            VectorSet => self.vector_set()?,
            VectorLength => self.vector_length()?,
            TupleNew => self.tuple_new()?,
            TupleGet => self.tuple_get()?,
            ClosureNew(source) => self.closure_new(source, module, environment)?,

            x => panic!("Unimplemented instruction: {:?}", x),
        }


        *program_counter += 1;
        Ok(InstructionResult::Continue)
    }

    basic_alu_op_int!(integer_add, +);
    basic_alu_op_int!(integer_subtract, -);
    basic_alu_op_int!(integer_multiply, *);

    fn integer_divide(&mut self) -> Result<(),Fault> {
        let right = self.stack.pop().get_boxed_value();
        let left = self.stack.pop().get_boxed_value();
        match (left, right) {
            (Value::Integer(left), Value::Integer(right)) => {
                if right.is_zero() {
                    return Err(Fault::DivisionByZero);
                }

                let result = left / right;

                if result.is_zero() {
                    self.flags.zero = true;
                } else {
                    self.flags.zero = false;
                }

                if result.is_negative() {
                    self.flags.negative = true;
                } else {
                    self.flags.negative = false;
                }

                let chunk = result.into_chunk();

                self.stack.push(chunk);
            }
            _ => return Err(Fault::NotAnInteger),
        }
        Ok(())
    }

    fn integer_modulo(&mut self) -> Result<(),Fault> {
        let right = self.stack.pop().get_boxed_value();
        let left = self.stack.pop().get_boxed_value();
        match (left, right) {
            (Value::Integer(left), Value::Integer(right)) => {
                if right.is_zero() {
                    return Err(Fault::DivisionByZero);
                }

                let result = left % right;

                if result.is_zero() {
                    self.flags.zero = true;
                } else {
                    self.flags.zero = false;
                }

                if result.is_negative() {
                    self.flags.negative = true;
                } else {
                    self.flags.negative = false;
                }

                let chunk = result.into_chunk();

                self.stack.push(chunk);
            }
            _ => return Err(Fault::NotAnInteger),
        }
        Ok(())
    }


    basic_alu_op_int!(integer_bitwise_and, &);
    basic_alu_op_int!(integer_bitwise_or, |);
    basic_alu_op_int!(integer_bitwise_xor, ^);
    basic_alu_op_int!(integer_shift_left, <<);
    basic_alu_op_int!(integer_shift_right, >>);

    fn integer_power(&mut self) -> Result<(),Fault> {
        let right = self.stack.pop().get_boxed_value();
        let left = self.stack.pop().get_boxed_value();
        match (left, right) {
            (Value::Integer(left), Value::Integer(right)) => {
                let result = left.pow(right);

                if result.is_zero() {
                    self.flags.zero = true;
                } else {
                    self.flags.zero = false;
                }

                if result.is_negative() {
                    self.flags.negative = true;
                } else {
                    self.flags.negative = false;
                }

                let chunk = result.into_chunk();

                self.stack.push(chunk);
            }
            _ => return Err(Fault::NotAnInteger),
        }
        Ok(())
    }

    fn integer_negate(&mut self) -> Result<(),Fault> {
        let value = self.stack.pop().get_boxed_value();
        match value {
            Value::Integer(value) => {
                let result = -value;

                if result.is_zero() {
                    self.flags.zero = true;
                } else {
                    self.flags.zero = false;
                }

                if result.is_negative() {
                    self.flags.negative = true;
                } else {
                    self.flags.negative = false;
                }

                let chunk = result.into_chunk();

                self.stack.push(chunk);
            }
            _ => return Err(Fault::NotAnInteger),
        }
        Ok(())
    }

    fn integer_bitwise_not(&mut self) -> Result<(),Fault> {
        let value = self.stack.pop().get_boxed_value();
        match value {
            Value::Integer(value) => {
                let result = !value;

                if result.is_zero() {
                    self.flags.zero = true;
                } else {
                    self.flags.zero = false;
                }

                if result.is_negative() {
                    self.flags.negative = true;
                } else {
                    self.flags.negative = false;
                }

                let chunk = result.into_chunk();

                self.stack.push(chunk);
            }
            _ => return Err(Fault::NotAnInteger),
        }
        Ok(())
    }

    basic_alu_op_decimal!(decimal_add, +);
    basic_alu_op_decimal!(decimal_subtract, -);
    basic_alu_op_decimal!(decimal_multiply, *);

    fn decimal_divide(&mut self) -> Result<(),Fault> {
        let right = self.stack.pop().get_boxed_value();
        let left = self.stack.pop().get_boxed_value();
        match (left, right) {
            (Value::Decimal(left), Value::Decimal(right)) => {
                if right.is_zero() {
                    return Err(Fault::DivisionByZero);
                }

                let result = left / right;

                if result.is_zero() {
                    self.flags.zero = true;
                } else {
                    self.flags.zero = false;
                }

                if result.is_negative() {
                    self.flags.negative = true;
                } else {
                    self.flags.negative = false;
                }

                let chunk = result.into_chunk();

                self.stack.push(chunk);
            }
            _ => return Err(Fault::NotAnInteger),
        }
        Ok(())
    }

    fn decimal_modulo(&mut self) -> Result<(),Fault> {
        let right = self.stack.pop().get_boxed_value();
        let left = self.stack.pop().get_boxed_value();
        match (left, right) {
            (Value::Decimal(left), Value::Decimal(right)) => {
                if right.is_zero() {
                    return Err(Fault::DivisionByZero);
                }

                let result = left % right;

                if result.is_zero() {
                    self.flags.zero = true;
                } else {
                    self.flags.zero = false;
                }

                if result.is_negative() {
                    self.flags.negative = true;
                } else {
                    self.flags.negative = false;
                }

                let chunk = result.into_chunk();

                self.stack.push(chunk);
            }
            _ => return Err(Fault::NotAnInteger),
        }
        Ok(())
    }

    fn decimal_power(&mut self) -> Result<(),Fault> {
        let right = self.stack.pop().get_boxed_value();
        let left = self.stack.pop().get_boxed_value();
        match (left, right) {
            (Value::Decimal(left), Value::Integer(right)) => {
                let result = left.powi(right);

                if result.is_zero() {
                    self.flags.zero = true;
                } else {
                    self.flags.zero = false;
                }

                if result.is_negative() {
                    self.flags.negative = true;
                } else {
                    self.flags.negative = false;
                }

                let chunk = result.into_chunk();

                self.stack.push(chunk);
            }
            (Value::Decimal(left), Value::Decimal(right)) if !left.is_rational() && !right.is_rational() => {
                let result = left.powd(right);

                if result.is_zero() {
                    self.flags.zero = true;
                } else {
                    self.flags.zero = false;
                }

                if result.is_negative() {
                    self.flags.negative = true;
                } else {
                    self.flags.negative = false;
                }

                let chunk = result.into_chunk();

                self.stack.push(chunk);
            }
            _ => return Err(Fault::NotAnInteger),
        }
        Ok(())
    }

    fn decimal_negate(&mut self) -> Result<(),Fault> {
        let value = self.stack.pop().get_boxed_value();
        match value {
            Value::Decimal(value) => {
                let result = -value;

                if result.is_zero() {
                    self.flags.zero = true;
                } else {
                    self.flags.zero = false;
                }

                if result.is_negative() {
                    self.flags.negative = true;
                } else {
                    self.flags.negative = false;
                }

                let chunk = result.into_chunk();

                self.stack.push(chunk);
            }
            _ => return Err(Fault::NotAnInteger),
        }
        Ok(())
    }

    fn boolean_and(&mut self) -> Result<(),Fault> {
        let right = self.stack.pop().get_boxed_value();
        let left = self.stack.pop().get_boxed_value();

        match (left, right) {
            (Value::Boolean(left), Value::Boolean(right)) => {
                let result = left && right;
                let chunk = result.into_chunk();
                self.stack.push(chunk);
            }
            _ => return Err(Fault::NotABoolean),
        }
        Ok(())
    }

    fn boolean_or(&mut self) -> Result<(),Fault> {
        let right = self.stack.pop().get_boxed_value();
        let left = self.stack.pop().get_boxed_value();

        match (left, right) {
            (Value::Boolean(left), Value::Boolean(right)) => {
                let result = left || right;
                let chunk = result.into_chunk();
                self.stack.push(chunk);
            }
            _ => return Err(Fault::NotABoolean),
        }
        Ok(())
    }

    fn boolean_not(&mut self) -> Result<(),Fault> {
        let value = self.stack.pop().get_boxed_value();

        match value {
            Value::Boolean(value) => {
                let result = !value;
                let chunk = result.into_chunk();
                self.stack.push(chunk);
            }
            _ => return Err(Fault::NotABoolean),
        }
        Ok(())
    }

    fn vector_new(&mut self, typ: &VectorType) -> Result<(), Fault> {

        let size = self.stack.pop().get_boxed_value();
        let size = match size {
            Value::Integer(size) => size.to_usize().unwrap(),
            _ => return Err(Fault::NotAnInteger),
        };

        let mut vec = Vector::new(size, typ);

        self.stack.push(vec.into_chunk());

        Ok(())
    }


    fn vector_get(&mut self) -> Result<(), Fault> {
        let index = self.stack.pop().get_boxed_value();
        let vector = self.stack.pop().get_boxed_value();
        match (vector, index) {
            (Value::Vector(vector), Value::Integer(index)) => {
                let index = index.to_usize().unwrap();
                let value = vector.get(index)?.clone();
                self.stack.push(vector.into_chunk());
                self.stack.push(value.into_chunk());
            }
            _ => return Err(Fault::NotAVector),
        }
        Ok(())
    }

    fn vector_set(&mut self) -> Result<(), Fault> {
        let value = self.stack.pop().get_boxed_value();
        let index = self.stack.pop().get_boxed_value();
        let mut vector = self.stack.pop().get_boxed_value();
        match (vector, index) {
            (Value::Vector(mut vector), Value::Integer(index)) => {
                let index = index.to_usize().unwrap();
                vector.set(index,value)?;
                self.stack.push(vector.into_chunk());
            }
            _ => return Err(Fault::NotAVector),
        }
        Ok(())
    }

    fn vector_length(&mut self) -> Result<(), Fault> {
        let vector = self.stack.pop().get_boxed_value();
        match vector {
            Value::Vector(vector) => {
                let length = vector.length();
                let chunk = length.into_chunk();
                self.stack.push(vector.into_chunk());
                self.stack.push(chunk);
            }
            _ => return Err(Fault::NotAVector),
        }
        Ok(())
    }

    fn tuple_new(&mut self) -> Result<(), Fault> {
        let size = self.stack.pop().get_boxed_value();
        let size = match size {
            Value::Integer(size) => size.to_usize().unwrap(),
            _ => return Err(Fault::NotAnInteger),
        };

        let mut data = Vec::new();
        for _ in 0..size {
            let value = self.stack.pop().get_boxed_value();
            data.push(value);
        }

        let tuple = Value::Tuple(Tuple::new(data.into_boxed_slice()));
        self.stack.push(tuple.into_chunk());

        Ok(())
    }

    fn tuple_get(&mut self) -> Result<(), Fault> {
        let index = self.stack.pop().get_boxed_value();
        let tuple = self.stack.pop().get_boxed_value();
        match (tuple, index) {
            (Value::Tuple(tuple), Value::Integer(index)) => {
                let index = index.to_usize().unwrap();
                let value = tuple.get(index).clone();
                self.stack.push(tuple.into_chunk());
                self.stack.push(value.into_chunk());
            }
            _ => return Err(Fault::NotATuple),
        }
        Ok(())
    }

    fn closure_new(&mut self, function_source: &FunctionSource, module: &Module, env: &mut Environment) -> Result<(), Fault> {
        let mut function = match function_source {
            FunctionSource::Name(name) => {
                let function = module.get_function(name)
                    .ok_or(Fault::FunctionNotFound(name.clone()))?;
                function.clone()
            }
            _ => panic!("Closure source must be a function name (i.e. a lifted lambda)"),
        };

        function.add_environment(env.clone());

        let closure = Value::Function(function);
        self.stack.push(closure.into_chunk());

        Ok(())
    }

    fn compare(&mut self, comparison_type: &ComparisonType) -> Result<(),Fault> {
        let right = self.stack.pop().get_boxed_value();
        let left = self.stack.pop().get_boxed_value();
        match comparison_type {
            ComparisonType::Equal => {
                if left == right {
                    self.flags.comparison = Comparison::Equal;
                } else {
                    self.flags.comparison = Comparison::NotEqual;
                }
            }
            ComparisonType::NotEqual => {
                if left != right {
                    self.flags.comparison = Comparison::NotEqual;
                } else {
                    self.flags.comparison = Comparison::Equal;
                }
            }
            ComparisonType::LessThan => {
                if left < right {
                    self.flags.comparison = Comparison::LessThan;
                } else {
                    self.flags.comparison = Comparison::GreaterThanOrEqual;
                }
            }
            ComparisonType::LessThanOrEqual => {
                if left <= right {
                    self.flags.comparison = Comparison::LessThanOrEqual;
                } else {
                    self.flags.comparison = Comparison::GreaterThan;
                }
            }
            ComparisonType::GreaterThan => {
                if left > right {
                    self.flags.comparison = Comparison::GreaterThan;
                } else {
                    self.flags.comparison = Comparison::LessThanOrEqual;
                }
            }
            ComparisonType::GreaterThanOrEqual => {
                if left >= right {
                    self.flags.comparison = Comparison::GreaterThanOrEqual;
                } else {
                    self.flags.comparison = Comparison::LessThan;
                }
            }
        }
        self.stack.push(left.into_chunk());
        self.stack.push(right.into_chunk());

        Ok(())
    }

    fn can_jump(&self, condition: &Condition) -> bool {
        match condition {
            Condition::Always => true,
            Condition::Equal => self.flags.comparison == Comparison::Equal,
            Condition::NotEqual => self.flags.comparison == Comparison::NotEqual,
            Condition::LessThan => self.flags.comparison == Comparison::LessThan,
            Condition::LessThanOrEqual => self.flags.comparison == Comparison::LessThanOrEqual,
            Condition::GreaterThan => self.flags.comparison == Comparison::GreaterThan,
            Condition::GreaterThanOrEqual => self.flags.comparison == Comparison::GreaterThanOrEqual,
            Condition::Negative => self.flags.negative,
            Condition::NotNegative => !self.flags.negative,
            Condition::Zero => self.flags.zero,
            Condition::NotZero => !self.flags.zero,
        }
    }

    fn goto<'a>(&mut self, target: &JumpTarget, condition: &Condition, program_counter: &mut usize) -> Result<InstructionResult<'a>, Fault> {
        if self.can_jump(condition) {
            match target {
                JumpTarget::Relative(offset) => {
                    *program_counter = (*program_counter as isize + offset) as usize;
                }
                JumpTarget::Absolute(address) => {
                    *program_counter = *address;
                }
            }
        } else {
            *program_counter += 1;
        }
        Ok(InstructionResult::Continue)
    }

    fn return_instruction<'a>(&mut self, condition: &Condition, program_counter: &mut usize) -> Result<InstructionResult<'a>, Fault> {
        if self.can_jump(condition) {
            return Ok(InstructionResult::Return);
        }
        *program_counter += 1;
        Ok(InstructionResult::Continue)
    }

    fn function_call<'a>(&mut self,
                     source: &FunctionSource,
                     condition: &Condition,
                     program_counter: &mut usize,
                     module: &'a Module) -> Result<InstructionResult<'a>, Fault> {

        *program_counter += 1;
        if self.can_jump(condition) {
            match source {
                FunctionSource::Name(name) => {
                    let function = module.get_function(name)
                        .ok_or(Fault::FunctionNotFound(name.clone()))?;
                    let mut environment = Environment::new();
                    for parameter in function.argument_names.iter() {
                        let value = self.stack.pop();
                        environment.insert(parameter.clone(), value.get_boxed_value());
                    }
                    return Ok(InstructionResult::CallRef(function, environment));
                }
                FunctionSource::Address => {
                    let reference = self.stack.pop().get_boxed_value();
                    match reference {
                        Value::Reference(_reference) => {

                            todo!("Implement function call by address");
                        }
                        _ => return Err(Fault::NotAReference),
                    }
                }
                FunctionSource::Stack => {
                    let function = self.stack.pop().get_boxed_value();
                    match function {
                        Value::Function(function) => {
                            let mut environment = Environment::new();
                            for parameter in function.argument_names.iter() {
                                let value = self.stack.pop();
                                environment.insert(parameter.clone(), value.get_boxed_value());
                            }
                            return Ok(InstructionResult::Call(function, environment));
                        }
                        _ => return Err(Fault::NotAFunction),
                    }
                }
            }
        }
        Ok(InstructionResult::Continue)
    }

}
