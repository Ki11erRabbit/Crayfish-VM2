mod integer_chunks;
mod decimal_chunks;

use std::cell::RefCell;
use crate::value::Value;

pub trait StackChunk {
    fn get_value(self) -> Value;

}




pub struct Stack<'a> {
    pub data: Vec<&'a RefCell<dyn StackChunk>>
}