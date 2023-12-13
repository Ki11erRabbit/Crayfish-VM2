use crate::stack::StackChunk;
use crate::value::integer::Integer;
use crate::value::Value;


macro_rules! integer_chunk {
    ($name:ident, $variant:ident, $type:ty) => {
        pub struct $name {
            pub data: $type
        }

        impl StackChunk for $name {
            fn get_value(self) -> Value {
                Value::Integer(Integer::$variant(self.data))
            }
        }
    };
}


integer_chunk!(U8Chunk, U8, u8);
integer_chunk!(U16Chunk, U16, u16);
integer_chunk!(U32Chunk, U32, u32);
integer_chunk!(U64Chunk, U64, u64);
integer_chunk!(I8Chunk, I8, i8);
integer_chunk!(I16Chunk, I16, i16);
integer_chunk!(I32Chunk, I32, i32);
integer_chunk!(I64Chunk, I64, i64);
integer_chunk!(NaturalChunk, Natural, malachite::Natural);
integer_chunk!(IntegerChunk, Integer, malachite::Integer);

