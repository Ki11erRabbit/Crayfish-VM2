use std::fmt::Display;
use malachite::Natural;
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

        impl Display for $type {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                let mut vector = String::new();
                vector.push_str("[");
                for i in 0..self.1 {
                    vector.push_str(&format!("{}, ", unsafe { std::ptr::read(self.0.offset(i as isize)) }));
                }
                vector.push_str("]");
                write!(f, "{}", vector)
            }
        }
    };
}

pub struct U8Vector(pub *mut u8, pub usize);
pub struct U16Vector(pub *mut u16, pub usize);
pub struct U32Vector(pub *mut u32, pub usize);
pub struct U64Vector(pub *mut u64, pub usize);
pub struct I8Vector(pub *mut i8, pub usize);
pub struct I16Vector(pub *mut i16, pub usize);
pub struct I32Vector(pub *mut i32, pub usize);
pub struct I64Vector(pub *mut i64, pub usize);
pub struct F32Vector(pub *mut f32, pub usize);
pub struct F64Vector(pub *mut f64, pub usize);
pub struct NaturalVector(pub *mut Natural, pub usize);
pub struct IntegerVector(pub *mut malachite::Integer, pub usize);
pub struct RationalVector(pub *mut malachite::Rational, pub usize);
pub struct ReferenceVector(pub *mut Reference, pub usize);


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


