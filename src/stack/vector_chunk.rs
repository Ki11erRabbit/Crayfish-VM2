
use crate::stack::StackChunk;
use crate::value::vector::Vector;
use crate::value::Value;
use crate::value::Reference;


macro_rules! vector_chunk {
    ($variant:ident, $type:ty) => {


        impl StackChunk for $type {
            fn get_value(self) -> Value {
                Value::Vector(Vector::$variant(self.0, self.1))
            }

            fn get_boxed_value(self: Box<Self>) -> Value {
                Value::Vector(Vector::$variant(self.0, self.1))
            }
        }
    };
}

vector_chunk!(U8, (*mut u8, usize));
vector_chunk!(U16, (*mut u16, usize));
vector_chunk!(U32, (*mut u32, usize));
vector_chunk!(U64, (*mut u64, usize));
vector_chunk!(I8, (*mut i8, usize));
vector_chunk!(I16, (*mut i16, usize));
vector_chunk!(I32, (*mut i32, usize));
vector_chunk!(I64, (*mut i64, usize));
vector_chunk!(F32, (*mut f32, usize));
vector_chunk!(F64, (*mut f64, usize));
vector_chunk!(Natural, (*mut malachite::Natural, usize));
vector_chunk!(Integer, (*mut malachite::Integer, usize));
vector_chunk!(Rational, (*mut malachite::Rational, usize));
vector_chunk!(Reference, (*mut Reference, usize));

