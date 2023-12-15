use crate::value::Value;
use crate::stack::StackChunk;
use crate::value::decimal::Decimal;

macro_rules! decimal_chunk {
    ($variant:ident, $type:ty) => {

        impl StackChunk for $type {
            fn get_value(self) -> Value {
                Value::Decimal(Decimal::$variant(self))
            }

            fn get_boxed_value(self: Box<Self>) -> Value {
                Value::Decimal(Decimal::$variant(*self))
            }

            fn into_chunk(self) -> Box<dyn StackChunk> {
                Box::new(self)
            }
        }
    };
}

decimal_chunk!(F32, f32);
decimal_chunk!(F64, f64);
decimal_chunk!(Rational, malachite::Rational);