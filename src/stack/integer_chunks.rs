use crate::stack::StackChunk;
use crate::value::integer::Integer;
use crate::value::Value;


macro_rules! integer_chunk {
    ($variant:ident, $type:ty) => {
        impl StackChunk for $type {
            fn get_value(self) -> Value {
                Value::Integer(Integer::$variant(self))
            }
            fn get_boxed_value(self: Box<Self>) -> Value {
                Value::Integer(Integer::$variant(*self))
            }
        }
    };
}


integer_chunk!(U8, u8);
integer_chunk!(U16, u16);
integer_chunk!(U32, u32);
integer_chunk!(U64, u64);
integer_chunk!(I8, i8);
integer_chunk!(I16, i16);
integer_chunk!(I32, i32);
integer_chunk!(I64, i64);
integer_chunk!(Natural, malachite::Natural);
integer_chunk!(Integer, malachite::Integer);

