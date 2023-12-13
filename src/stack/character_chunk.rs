use crate::stack::StackChunk;
use crate::value::Value;

pub struct CharacterChunk {
    pub data: char,
}


impl StackChunk for CharacterChunk {
    fn get_value(self) -> Value {
        Value::Character(self.data)
    }
}