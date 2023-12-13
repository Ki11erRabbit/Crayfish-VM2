mod integer_chunks;
mod decimal_chunks;
mod vector_chunk;
mod product_chunk;
mod sum_chunks;
mod tuple_chunk;
mod function_chunk;
mod reference_chunk;
mod character_chunk;
mod boolean_chunk;

use std::cell::RefCell;
use crate::value::Value;

pub trait StackChunk {
    fn get_value(self) -> Value;

}




pub struct Stack<'a> {
    pub data: Vec<&'a RefCell<dyn StackChunk>>
}