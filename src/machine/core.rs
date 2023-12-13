use crate::instruction::{Instruction, RealInstruction};
use crate::machine::Fault;
use crate::stack::Stack;
use crate::value::Value;


macro_rules! basic_alu_op {
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
    program_counter: usize,
}


impl Core {
    pub fn new(stack: Stack) -> Core {
        Core {
            flags: CoreFlags {
                comparison: Comparison::None,
                negative: false,
                zero: false,
            },
            stack,
            program_counter: 0,
        }
    }

    pub fn execute_instruction(& mut self,
                               instruction: Instruction) -> Result<bool, Fault> {
        use RealInstruction::*;
        match instruction.instruction {
            Halt => return Ok(false),
            NoOp => {},
            Push(value) => {
                let chunk = value.into_chunk();
                self.stack.push(chunk);
            }
            Pop => {
                self.stack.pop();
            }
            IntegerNew(integer) => {
                let chunk = integer.into_chunk();
                self.stack.push(chunk);
            }
            IntegerAdd => self.integer_add()?,
            IntegerSubtract => self.integer_subtract()?,
            IntegerMultiply => self.integer_multiply()?,
            IntegerDivide => self.integer_divide()?,
            IntegerModulo => self.integer_modulo()?,
            //IntegerPower => self.integer_power()?,
            IntegerNegate => self.integer_negate()?,
            IntegerBitwiseAnd => self.integer_bitwise_and()?,
            IntegerBitwiseOr => self.integer_bitwise_or()?,
            IntegerBitwiseXor => self.integer_bitwise_xor()?,
            IntegerBitwiseNot => self.integer_bitwise_not()?,
            IntegerShiftLeft => self.integer_shift_left()?,
            IntegerShiftRight => self.integer_shift_right()?,
            x => panic!("Unimplemented instruction: {:?}", x),
        }


        self.program_counter += 1;
        Ok(true)
    }

    basic_alu_op!(integer_add, +);
    basic_alu_op!(integer_subtract, -);
    basic_alu_op!(integer_multiply, *);

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


    basic_alu_op!(integer_bitwise_and, &);
    basic_alu_op!(integer_bitwise_or, |);
    basic_alu_op!(integer_bitwise_xor, ^);
    basic_alu_op!(integer_shift_left, <<);
    basic_alu_op!(integer_shift_right, >>);

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




}
