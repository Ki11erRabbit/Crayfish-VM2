use std::cell::RefCell;
use std::fmt::Display;
use malachite::Natural;
use crate::stack::StackChunk;
use crate::value::function::Function;
use crate::value::vector::Vector;
use crate::value::Value;
use crate::value::Reference;
use crate::value::tuple::Tuple;


macro_rules! vector_chunk {
    ($variant:ident, $type:ty) => {


        impl StackChunk for $type {
            fn get_value(self) -> Value {
                Value::Vector(Vector::$variant(self.0))
            }

            fn get_boxed_value(self: Box<Self>) -> Value {
                Value::Vector(Vector::$variant(self.0))
            }

            fn into_chunk(self) -> Box<dyn StackChunk> {
                Box::new(self)
            }
        }

        impl Display for $type {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                let mut vector = String::new();
                vector.push_str("[");
                for (i, value) in self.0.iter().enumerate() {
                    if i > 0 {
                        vector.push_str(", ");
                    }
                    vector.push_str(&format!("{}", value));
                }
                vector.push_str("]");
                write!(f, "{}", vector)
            }
        }
    };
}

pub struct U8Vector(pub Vec<u8>);
pub struct U16Vector(pub Vec<u16>);
pub struct U32Vector(pub Vec<u32>);
pub struct U64Vector(pub Vec<u64>);
pub struct I8Vector(pub Vec<i8>);
pub struct I16Vector(pub Vec<i16>);
pub struct I32Vector(pub Vec<i32>);
pub struct I64Vector(pub Vec<i64>);
pub struct F32Vector(pub Vec<f32>);
pub struct F64Vector(pub Vec<f64>);
pub struct NaturalVector(pub Vec<Natural>);
pub struct IntegerVector(pub Vec<malachite::Integer>);
pub struct RationalVector(pub Vec<malachite::Rational>);
pub struct ReferenceVector(pub Vec<Reference>);
pub struct VectorVector(pub Vec<Vector>);
pub struct TupleVector(pub Vec<Tuple>);
pub struct FunctionVector(pub Vec<Function>);


vector_chunk!(U8, U8Vector);
vector_chunk!(U16, U16Vector);
vector_chunk!(U32, U32Vector);
vector_chunk!(U64, U64Vector);
vector_chunk!(I8, I8Vector);
vector_chunk!(I16, I16Vector);
vector_chunk!(I32, I32Vector);
vector_chunk!(I64, I64Vector);
vector_chunk!(F32, F32Vector);
vector_chunk!(F64, F64Vector);
vector_chunk!(Natural, NaturalVector);
vector_chunk!(Integer, IntegerVector);
vector_chunk!(Rational, RationalVector);
vector_chunk!(Reference, ReferenceVector);
vector_chunk!(Vector, VectorVector);
vector_chunk!(Tuple, TupleVector);
vector_chunk!(Function, FunctionVector);

