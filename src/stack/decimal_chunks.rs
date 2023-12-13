use crate::value::Value;
use crate::stack::StackChunk;
use crate::value::decimal::Decimal;

macro_rules! decimal_chunk {
    ($name:ident, $variant:ident, $type:ty) => {
        pub struct $name {
            pub data: $type
        }

        impl StackChunk for $name {
            fn get_value(self) -> Value {
                Value::Decimal(Decimal::$variant(self.data))
            }
        }
    };
}

decimal_chunk!(F32Chunk, F32, f32);
decimal_chunk!(F64Chunk, F64, f64);
decimal_chunk!(RationalChunk, Rational, malachite::Rational);