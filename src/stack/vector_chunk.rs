
use crate::stack::StackChunk;
use crate::value::vector::Vector;
use crate::value::Value;
use crate::value::Reference;


macro_rules! vector_chunk {
    ($name:ident, $variant:ident, $type:ty) => {
        pub struct $name {
            pub data: $type
        }

        impl StackChunk for $name {
            fn get_value(self) -> Value {
                Value::Vector(Vector::$variant(self.data.0, self.data.1))
            }
        }
    };
}

vector_chunk!(U8VectorChunk, U8, (*mut u8, usize));
vector_chunk!(U16VectorChunk, U16, (*mut u16, usize));
vector_chunk!(U32VectorChunk, U32, (*mut u32, usize));
vector_chunk!(U64VectorChunk, U64, (*mut u64, usize));
vector_chunk!(I8VectorChunk, I8, (*mut i8, usize));
vector_chunk!(I16VectorChunk, I16, (*mut i16, usize));
vector_chunk!(I32VectorChunk, I32, (*mut i32, usize));
vector_chunk!(I64VectorChunk, I64, (*mut i64, usize));
vector_chunk!(F32VectorChunk, F32, (*mut f32, usize));
vector_chunk!(F64VectorChunk, F64, (*mut f64, usize));
vector_chunk!(NaturalVectorChunk, Natural, (*mut malachite::Natural, usize));
vector_chunk!(IntegerVectorChunk, Integer, (*mut malachite::Integer, usize));
vector_chunk!(RationalVectorChunk, Rational, (*mut malachite::Rational, usize));
vector_chunk!(ReferenceVectorChunk, Reference, (*mut Reference, usize));

