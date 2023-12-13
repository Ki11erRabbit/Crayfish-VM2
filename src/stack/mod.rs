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

use std::cell::RefCell;
use crate::value::Value;

pub trait StackChunk {
    fn get_value(self) -> Value;

}




pub struct Stack<'a> {
    pub data: Vec<&'a RefCell<dyn StackChunk>>
}


impl<'a> Stack<'a> {
    pub fn new() -> Stack<'a> {
        Stack {
            data: Vec::new()
        }
    }

    pub fn push(&'a mut self, chunk: &'a RefCell<dyn StackChunk>) {
        self.data.push(chunk);
    }

    pub fn pop(&'a mut self) -> &'a RefCell<dyn StackChunk> {
        self.data.pop().expect("Stack is empty")
    }

    pub fn peek(&'a self) -> &'a RefCell<dyn StackChunk> {
        self.data.last().expect("Stack is empty")
    }

    pub fn len(&self) -> usize {
        self.data.len()
    }

    pub fn is_empty(&self) -> bool {
        self.data.is_empty()
    }
}