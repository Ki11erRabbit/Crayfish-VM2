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
use crate::value::Value;

pub trait StackChunk {
    fn get_value(self) -> Value;
    fn get_boxed_value(self: Box<Self>) -> Value;

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