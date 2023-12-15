pub mod integer_chunks;
pub mod decimal_chunks;
pub mod vector_chunk;
pub mod product_chunk;
pub mod sum_chunks;
pub mod tuple_chunk;
pub mod function_chunk;
pub mod reference_chunk;
pub mod character_chunk;
pub mod boolean_chunk;
mod string_chunk;

use std::cell::RefCell;
use std::fmt::Display;
use crate::value::integer::Integer;
use crate::value::Value;

pub trait StackChunk:Display {
    fn get_value(self) -> Value;
    fn get_boxed_value(self: Box<Self>) -> Value;
    fn into_chunk(self) -> Box<dyn StackChunk>;
}

impl StackChunk for usize {
    fn get_value(self) -> Value {
        Value::Integer(Integer::U64(self as u64))
    }

    fn get_boxed_value(self: Box<Self>) -> Value {
        Value::Integer(Integer::U64(*self as u64))
    }

    fn into_chunk(self) -> Box<dyn StackChunk> {
        Box::new(self)
    }
}


pub struct Stack {
    pub data: Vec<Box<dyn StackChunk>>
}


impl Stack {
    pub fn new() -> Stack {
        Stack {
            data: Vec::new()
        }
    }

    pub fn push(& mut self, chunk: Box<dyn StackChunk>) {
        self.data.push(chunk);
    }

    pub fn pop(&mut self) -> Box<dyn StackChunk> {
        self.data.pop().expect("Stack is empty")
    }

    pub fn peek(& self) -> &Box<dyn StackChunk> {
        self.data.last().expect("Stack is empty")
    }

    pub fn len(&self) -> usize {
        self.data.len()
    }

    pub fn is_empty(&self) -> bool {
        self.data.is_empty()
    }
}

impl Display for Stack {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut stack = String::new();
        for chunk in self.data.iter() {
            stack.push_str(&format!("{}, \n", chunk));
        }
        write!(f, "{}", stack)
    }
}