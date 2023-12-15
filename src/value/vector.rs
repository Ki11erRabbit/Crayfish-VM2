use std::alloc::Layout;
use std::fmt::{Debug, Display, Formatter};
use malachite::Natural;
use malachite::num::basic::traits::Zero;
use crate::machine::Fault;
use crate::stack::StackChunk;
use crate::stack::vector_chunk::{F32Vector, F64Vector, FunctionVector, I16Vector, I32Vector, I64Vector, I8Vector, IntegerVector, NaturalVector, RationalVector, ReferenceVector, TupleVector, U16Vector, U32Vector, U64Vector, U8Vector, VectorVector};
use crate::value::{Reference, Value, ValueType};
use crate::value::decimal::{Decimal, DecimalType};
use crate::value::function::Function;
use crate::value::integer::{Integer, IntegerType};
use crate::value::tuple::Tuple;

#[derive(Debug, Clone)]
pub enum VectorType {
    U8(usize),
    U16(usize),
    U32(usize),
    U64(usize),
    I8(usize),
    I16(usize),
    I32(usize),
    I64(usize),
    F32(usize),
    F64(usize),
    Natural(usize),
    Integer(usize),
    Rational(usize),
    Reference(usize),
    Tuple(usize),
    Function(usize),
    Vector(Box<VectorType>, usize),
}

impl VectorType {

    pub fn get_size(&self) -> usize {
        match self {
            VectorType::U8(size) => *size,
            VectorType::U16(size) => *size,
            VectorType::U32(size) => *size,
            VectorType::U64(size) => *size,
            VectorType::I8(size) => *size,
            VectorType::I16(size) => *size,
            VectorType::I32(size) => *size,
            VectorType::I64(size) => *size,
            VectorType::F32(size) => *size,
            VectorType::F64(size) => *size,
            VectorType::Natural(size) => *size,
            VectorType::Integer(size) => *size,
            VectorType::Rational(size) => *size,
            VectorType::Reference(size) => *size,
            VectorType::Vector(_, size) => *size,
            VectorType::Tuple(size) => *size,
            VectorType::Function(size) => *size,
        }
    }

    pub fn get_layout(&self) -> Layout {
        match self {
            VectorType::U8(size) => Layout::array::<u8>(*size).unwrap(),
            VectorType::U16(size) => Layout::array::<u16>(*size).unwrap(),
            VectorType::U32(size) => Layout::array::<u32>(*size).unwrap(),
            VectorType::U64(size) => Layout::array::<u64>(*size).unwrap(),
            VectorType::I8(size) => Layout::array::<i8>(*size).unwrap(),
            VectorType::I16(size) => Layout::array::<i16>(*size).unwrap(),
            VectorType::I32(size) => Layout::array::<i32>(*size).unwrap(),
            VectorType::I64(size) => Layout::array::<i64>(*size).unwrap(),
            VectorType::F32(size) => Layout::array::<f32>(*size).unwrap(),
            VectorType::F64(size) => Layout::array::<f64>(*size).unwrap(),
            VectorType::Natural(size) => Layout::array::<Natural>(*size).unwrap(),
            VectorType::Integer(size) => Layout::array::<malachite::Integer>(*size).unwrap(),
            VectorType::Rational(size) => Layout::array::<malachite::Rational>(*size).unwrap(),
            VectorType::Tuple(size) => Layout::array::<Tuple>(*size).unwrap(),
            VectorType::Function(size) => Layout::array::<Function>(*size).unwrap(),
            VectorType::Reference(size) => Layout::array::<Reference>(*size).unwrap(),
            _ => panic!("Cannot get layout of VectorType::Vector"),
        }
    }
    pub fn get_array_layout(&self, size: usize) -> Layout {
        match self {
            VectorType::U8(_) => Layout::array::<u8>(size).unwrap(),
            VectorType::U16(_) => Layout::array::<u16>(size).unwrap(),
            VectorType::U32(_) => Layout::array::<u32>(size).unwrap(),
            VectorType::U64(_) => Layout::array::<u64>(size).unwrap(),
            VectorType::I8(_) => Layout::array::<i8>(size).unwrap(),
            VectorType::I16(_) => Layout::array::<i16>(size).unwrap(),
            VectorType::I32(_) => Layout::array::<i32>(size).unwrap(),
            VectorType::I64(_) => Layout::array::<i64>(size).unwrap(),
            VectorType::F32(_) => Layout::array::<f32>(size).unwrap(),
            VectorType::F64(_) => Layout::array::<f64>(size).unwrap(),
            VectorType::Natural(_) => Layout::array::<Natural>(size).unwrap(),
            VectorType::Integer(_) => Layout::array::<malachite::Integer>(size).unwrap(),
            VectorType::Rational(_) => Layout::array::<malachite::Rational>(size).unwrap(),
            VectorType::Reference(_) => Layout::array::<Reference>(size).unwrap(),
            VectorType::Vector(_,_) => Layout::array::<Vector>(size).unwrap(),
            VectorType::Tuple(_) => Layout::array::<Tuple>(size).unwrap(),
            VectorType::Function(_) => Layout::array::<Function>(size).unwrap(),
        }
    }
}

impl Display for VectorType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            VectorType::U8(size) => write!(f, "u8 x {}", size),
            VectorType::U16(size) => write!(f, "u16 x {}", size),
            VectorType::U32(size) => write!(f, "u32 x {}", size),
            VectorType::U64(size) => write!(f, "u64 x {}", size),
            VectorType::I8(size) => write!(f, "i8 x {}", size),
            VectorType::I16(size) => write!(f, "i16 x {}", size),
            VectorType::I32(size) => write!(f, "i32 x {}", size),
            VectorType::I64(size) => write!(f, "i64 x {}", size),
            VectorType::F32(size) => write!(f, "f32 x {}", size),
            VectorType::F64(size) => write!(f, "f64 x {}", size),
            VectorType::Natural(size) => write!(f, "Natural x {}", size),
            VectorType::Integer(size) => write!(f, "Integer x {}", size),
            VectorType::Rational(size) => write!(f, "Rational x {}", size),
            VectorType::Reference(size) => write!(f, "Reference x {}", size),
            VectorType::Vector(typ, size) => write!(f, "Vector of {} x {}", typ, size),
            VectorType::Tuple(size) => write!(f, "Tuple x {}", size),
            VectorType::Function(size) => write!(f, "Function x {}", size),
        }
    }
}


#[derive(Clone)]
pub enum Vector {
    U8(Vec<u8>),
    U16(Vec<u16>),
    U32(Vec<u32>),
    U64(Vec<u64>),
    I8(Vec<i8>),
    I16(Vec<i16>),
    I32(Vec<i32>),
    I64(Vec<i64>),
    F32(Vec<f32>),
    F64(Vec<f64>),
    Natural(Vec<Natural>),
    Integer(Vec<malachite::Integer>),
    Rational(Vec<malachite::Rational>),
    Reference(Vec<Reference>),
    Vector(Vec<Vector>),
    Tuple(Vec<Tuple>),
    Function(Vec<Function>),
}

impl Vector {

    pub fn new(size: usize, typ: &VectorType) -> Self {
        match typ {
            VectorType::U8(_) => Vector::U8(vec![0; size]),
            VectorType::U16(_) => Vector::U16(vec![0; size]),
            VectorType::U32(_) => Vector::U32(vec![0; size]),
            VectorType::U64(_) => Vector::U64(vec![0; size]),
            VectorType::I8(_) => Vector::I8(vec![0; size]),
            VectorType::I16(_) => Vector::I16(vec![0; size]),
            VectorType::I32(_) => Vector::I32(vec![0; size]),
            VectorType::I64(_) => Vector::I64(vec![0; size]),
            VectorType::F32(_) => Vector::F32(vec![0.0; size]),
            VectorType::F64(_) => Vector::F64(vec![0.0; size]),
            VectorType::Natural(_) => Vector::Natural(vec![Natural::ZERO; size]),
            VectorType::Integer(_) => Vector::Integer(vec![malachite::Integer::ZERO; size]),
            VectorType::Rational(_) => Vector::Rational(vec![malachite::Rational::ZERO; size]),
            VectorType::Reference(_) => Vector::Reference(vec![Reference(0); size]),
            VectorType::Vector(typ, sub_size) => Vector::Vector(vec![Vector::new(*sub_size, typ); size]),
            VectorType::Tuple(_) => Vector::Tuple(vec![Tuple::empty(); size]),
            VectorType::Function(_) => Vector::Function(vec![Function::empty(); size]),
        }
    }

    pub fn into_chunk(self) -> Box<dyn StackChunk> {
        match self {
            Vector::U8(pointer) => Box::new(U8Vector(pointer)),
            Vector::U16(pointer) => Box::new(U16Vector(pointer)),
            Vector::U32(pointer) => Box::new(U32Vector(pointer)),
            Vector::U64(pointer) => Box::new(U64Vector(pointer)),
            Vector::I8(pointer) => Box::new(I8Vector(pointer)),
            Vector::I16(pointer) => Box::new(I16Vector(pointer)),
            Vector::I32(pointer) => Box::new(I32Vector(pointer)),
            Vector::I64(pointer) => Box::new(I64Vector(pointer)),
            Vector::F32(pointer) => Box::new(F32Vector(pointer)),
            Vector::F64(pointer) => Box::new(F64Vector(pointer)),
            Vector::Natural(pointer) => Box::new(NaturalVector(pointer)),
            Vector::Integer(pointer) => Box::new(IntegerVector(pointer)),
            Vector::Rational(pointer) => Box::new(RationalVector(pointer)),
            Vector::Reference(pointer) => Box::new(ReferenceVector(pointer)),
            Vector::Vector(pointer) => Box::new(VectorVector(pointer)),
            Vector::Tuple(pointer) => Box::new(TupleVector(pointer)),
            Vector::Function(pointer) => Box::new(FunctionVector(pointer)),
        }
    }

    pub fn get(&self, index: usize) -> Result<Value, Fault> {
        match self {
            Vector::U8(pointer) => {
                if index < pointer.len() {
                    Ok(Value::Integer(Integer::U8(pointer[index])))
                } else {
                    Err(Fault::OutOfBounds)
                }
            },
            Vector::U16(pointer) => {
                if index < pointer.len() {
                    Ok(Value::Integer(Integer::U16(pointer[index])))
                } else {
                    Err(Fault::OutOfBounds)
                }
            },
            Vector::U32(pointer) => {
                if index < pointer.len() {
                    Ok(Value::Integer(Integer::U32(pointer[index])))
                } else {
                    Err(Fault::OutOfBounds)
                }
            },
            Vector::U64(pointer) => {
                if index < pointer.len() {
                    Ok(Value::Integer(Integer::U64(pointer[index])))
                } else {
                    Err(Fault::OutOfBounds)
                }
            },
            Vector::I8(pointer) => {
                if index < pointer.len() {
                    Ok(Value::Integer(Integer::I8(pointer[index])))
                } else {
                    Err(Fault::OutOfBounds)
                }
            },
            Vector::I16(pointer) => {
                if index < pointer.len() {
                    Ok(Value::Integer(Integer::I16(pointer[index])))
                } else {
                    Err(Fault::OutOfBounds)
                }
            },
            Vector::I32(pointer) => {
                if index < pointer.len() {
                    Ok(Value::Integer(Integer::I32(pointer[index])))
                } else {
                    Err(Fault::OutOfBounds)
                }
            },
            Vector::I64(pointer) => {
                if index < pointer.len() {
                    Ok(Value::Integer(Integer::I64(pointer[index])))
                } else {
                    Err(Fault::OutOfBounds)
                }
            },
            Vector::F32(pointer) => {
                if index < pointer.len() {
                    Ok(Value::Decimal(Decimal::F32(pointer[index])))
                } else {
                    Err(Fault::OutOfBounds)
                }
            },
            Vector::F64(pointer) => {
                if index < pointer.len() {
                    Ok(Value::Decimal(Decimal::F64(pointer[index])))
                } else {
                    Err(Fault::OutOfBounds)
                }
            },
            Vector::Natural(pointer) => {
                if index < pointer.len() {
                    Ok(Value::Integer(Integer::Natural(pointer[index].clone())))
                } else {
                    Err(Fault::OutOfBounds)
                }
            },
            Vector::Integer(pointer) => {
                if index < pointer.len() {
                    Ok(Value::Integer(Integer::Integer(pointer[index].clone())))
                } else {
                    Err(Fault::OutOfBounds)
                }
            },
            Vector::Rational(pointer) => {
                if index < pointer.len() {
                    Ok(Value::Decimal(Decimal::Rational(pointer[index].clone())))
                } else {
                    Err(Fault::OutOfBounds)
                }
            },
            Vector::Reference(pointer) => {
                if index < pointer.len() {
                    Ok(Value::Reference(pointer[index]))
                } else {
                    Err(Fault::OutOfBounds)
                }
            },
            Vector::Vector(pointer) => {
                if index < pointer.len() {
                    Ok(Value::Vector(pointer[index].clone()))
                } else {
                    Err(Fault::OutOfBounds)
                }
            },
            Vector::Tuple(pointer) => {
                if index < pointer.len() {
                    Ok(Value::Tuple(pointer[index].clone()))
                } else {
                    Err(Fault::OutOfBounds)
                }
            },
            Vector::Function(pointer) => {
                if index < pointer.len() {
                    Ok(Value::Function(pointer[index].clone()))
                } else {
                    Err(Fault::OutOfBounds)
                }
            },
        }
    }

    pub fn set(&mut self, index: usize, value: Value) -> Result<(), Fault> {
        match (self, value)  {
            (Vector::U8(pointer), Value::Integer(Integer::U8(value))) => {
                if index < pointer.len() {
                    pointer[index] = value;
                    Ok(())
                } else {
                    Err(Fault::OutOfBounds)
                }
            },
            (Vector::U16(pointer), Value::Integer(Integer::U16(value))) => {
                if index < pointer.len() {
                    pointer[index] = value;
                    Ok(())
                } else {
                    Err(Fault::OutOfBounds)
                }
            },
            (Vector::U32(pointer), Value::Integer(Integer::U32(value))) => {
                if index < pointer.len() {
                    pointer[index] = value;
                    Ok(())
                } else {
                    Err(Fault::OutOfBounds)
                }
            },
            (Vector::U64(pointer), Value::Integer(Integer::U64(value))) => {
                if index < pointer.len() {
                    pointer[index] = value;
                    Ok(())
                } else {
                    Err(Fault::OutOfBounds)
                }
            },
            (Vector::I8(pointer), Value::Integer(Integer::I8(value))) => {
                if index < pointer.len() {
                    pointer[index] = value;
                    Ok(())
                } else {
                    Err(Fault::OutOfBounds)
                }
            },
            (Vector::I16(pointer), Value::Integer(Integer::I16(value))) => {
                if index < pointer.len() {
                    pointer[index] = value;
                    Ok(())
                } else {
                    Err(Fault::OutOfBounds)
                }
            },
            (Vector::I32(pointer), Value::Integer(Integer::I32(value))) => {
                if index < pointer.len() {
                    pointer[index] = value;
                    Ok(())
                } else {
                    Err(Fault::OutOfBounds)
                }
            },
            (Vector::I64(pointer), Value::Integer(Integer::I64(value))) => {
                if index < pointer.len() {
                    pointer[index] = value;
                    Ok(())
                } else {
                    Err(Fault::OutOfBounds)
                }
            },
            (Vector::F32(pointer), Value::Decimal(Decimal::F32(value))) => {
                if index < pointer.len() {
                    pointer[index] = value;
                    Ok(())
                } else {
                    Err(Fault::OutOfBounds)
                }
            },
            (Vector::F64(pointer), Value::Decimal(Decimal::F64(value))) => {
                if index < pointer.len() {
                    pointer[index] = value;
                    Ok(())
                } else {
                    Err(Fault::OutOfBounds)
                }
            },
            (Vector::Natural(pointer), Value::Integer(Integer::Natural(value))) => {
                if index < pointer.len() {
                    pointer[index] = value;
                    Ok(())
                } else {
                    Err(Fault::OutOfBounds)
                }
            },
            (Vector::Integer(pointer), Value::Integer(Integer::Integer(value))) => {
                if index < pointer.len() {
                    pointer[index] = value;
                    Ok(())
                } else {
                    Err(Fault::OutOfBounds)
                }
            },
            (Vector::Rational(pointer), Value::Decimal(Decimal::Rational(value))) => {
                if index < pointer.len() {
                    pointer[index] = value;
                    Ok(())
                } else {
                    Err(Fault::OutOfBounds)
                }
            },
            (Vector::Reference(pointer), Value::Reference(value)) => {
                if index < pointer.len() {
                    pointer[index] = value;
                    Ok(())
                } else {
                    Err(Fault::OutOfBounds)
                }
            },
            (Vector::Vector(pointer), Value::Vector(value)) => {
                if index < pointer.len() {
                    pointer[index] = value;
                    Ok(())
                } else {
                    Err(Fault::OutOfBounds)
                }
            },
            (Vector::Tuple(pointer), Value::Tuple(value)) => {
                if index < pointer.len() {
                    pointer[index] = value;
                    Ok(())
                } else {
                    Err(Fault::OutOfBounds)
                }
            },
            (Vector::Function(pointer), Value::Function(value)) => {
                if index < pointer.len() {
                    pointer[index] = value;
                    Ok(())
                } else {
                    Err(Fault::OutOfBounds)
                }
            },
            _ => Err(Fault::TypeMismatch)
        }
    }



    pub fn length(&self) -> usize {
        match self {
            Vector::U8(pointer) => pointer.len(),
            Vector::U16(pointer) => pointer.len(),
            Vector::U32(pointer) => pointer.len(),
            Vector::U64(pointer) => pointer.len(),
            Vector::I8(pointer) => pointer.len(),
            Vector::I16(pointer) => pointer.len(),
            Vector::I32(pointer) => pointer.len(),
            Vector::I64(pointer) => pointer.len(),
            Vector::F32(pointer) => pointer.len(),
            Vector::F64(pointer) => pointer.len(),
            Vector::Natural(pointer) => pointer.len(),
            Vector::Integer(pointer) => pointer.len(),
            Vector::Rational(pointer) => pointer.len(),
            Vector::Reference(pointer) => pointer.len(),
            Vector::Vector(pointer) => pointer.len(),
            Vector::Tuple(pointer) => pointer.len(),
            Vector::Function(pointer) => pointer.len(),
        }
    }
}



impl Display for Vector {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Vector::U8(pointer) => {
                write!(f, "[")?;
                for item in pointer.iter() {
                    write!(f, "{}", item)?;
                }
                write!(f, "]")
            },
            Vector::U16(pointer) => {
                write!(f, "[")?;
                for item in pointer.iter() {
                    write!(f, "{}", item)?;
                }
                write!(f, "]")
            },
            Vector::U32(pointer) => {
                write!(f, "[")?;
                for item in pointer.iter() {
                    write!(f, "{}", item)?;
                }
                write!(f, "]")
            },
            Vector::U64(pointer) => {
                write!(f, "[")?;
                for item in pointer.iter() {
                    write!(f, "{}", item)?;
                }
                write!(f, "]")
            },
            Vector::I8(pointer) => {
                write!(f, "[")?;
                for item in pointer.iter() {
                    write!(f, "{}", item)?;
                }
                write!(f, "]")
            },
            Vector::I16(pointer) => {
                write!(f, "[")?;
                for item in pointer.iter() {
                    write!(f, "{}", item)?;
                }
                write!(f, "]")
            },
            Vector::I32(pointer) => {
                write!(f, "[")?;
                for item in pointer.iter() {
                    write!(f, "{}", item)?;
                }
                write!(f, "]")
            },
            Vector::I64(pointer) => {
                write!(f, "[")?;
                for item in pointer.iter() {
                    write!(f, "{}", item)?;
                }
                write!(f, "]")
            },
            Vector::F32(pointer) => {
                write!(f, "[")?;
                for item in pointer.iter() {
                    write!(f, "{}", item)?;
                }
                write!(f, "]")
            },
            Vector::F64(pointer) => {
                write!(f, "[")?;
                for item in pointer.iter() {
                    write!(f, "{}", item)?;
                }
                write!(f, "]")
            },
            Vector::Natural(pointer) => {
                write!(f, "[")?;
                for item in pointer.iter() {
                    write!(f, "{}", item)?;
                }
                write!(f, "]")
            },
            Vector::Integer(pointer) => {
                write!(f, "[")?;
                for item in pointer.iter() {
                    write!(f, "{}", item)?;
                }
                write!(f, "]")
            },
            Vector::Rational(pointer) => {
                write!(f, "[")?;
                for item in pointer.iter() {
                    write!(f, "{}", item)?;
                }
                write!(f, "]")
            },
            Vector::Reference(pointer) => {
                write!(f, "[")?;
                for item in pointer.iter() {
                    write!(f, "{}", item)?;
                }
                write!(f, "]")
            },
            Vector::Vector(pointer) => {
                write!(f, "[")?;
                for item in pointer.iter() {
                    write!(f, "{}", item)?;
                }
                write!(f, "]")
            },
            Vector::Tuple(pointer) => {
                write!(f, "[")?;
                for item in pointer.iter() {
                    write!(f, "{}", item)?;
                }
                write!(f, "]")
            },
            Vector::Function(pointer) => {
                write!(f, "[")?;
                for item in pointer.iter() {
                    write!(f, "{}", item)?;
                }
                write!(f, "]")
            }
        }
    }
}

impl Debug for Vector {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Vector::U8(_) => write!(f, "{}", self),
            Vector::U16(_) => write!(f, "{}", self),
            Vector::U32(_) => write!(f, "{}", self),
            Vector::U64(_) => write!(f, "{}", self),
            Vector::I8(_) => write!(f, "{}", self),
            Vector::I16(_) => write!(f, "{}", self),
            Vector::I32(_) => write!(f, "{}", self),
            Vector::I64(_) => write!(f, "{}", self),
            Vector::F32(_) => write!(f, "{}", self),
            Vector::F64(_) => write!(f, "{}", self),
            Vector::Natural(pointer) => {
                write!(f, "[")?;
                for item in pointer.iter() {
                    write!(f, "{:?}", item)?;
                }
                write!(f, "]")
            },
            Vector::Integer(pointer) => {
                write!(f, "[")?;
                for item in pointer.iter() {
                    write!(f, "{:?}", item)?;
                }
                write!(f, "]")
            },
            Vector::Rational(pointer) => {
                write!(f, "[")?;
                for item in pointer.iter() {
                    write!(f, "{:?}", item)?;
                }
                write!(f, "]")
            },
            Vector::Reference(_) => write!(f, "{}", self),
            Vector::Vector(pointer) => {
                write!(f, "[")?;
                for item in pointer.iter() {
                    write!(f, "{:?}", item)?;
                }
                write!(f, "]")
            },
            Vector::Tuple(pointer) => {
                write!(f, "[")?;
                for item in pointer.iter() {
                    write!(f, "{:?}", item)?;
                }
                write!(f, "]")
            }
            Vector::Function(pointer) => {
                write!(f, "[")?;
                for item in pointer.iter() {
                    write!(f, "{:?}", item)?;
                }
                write!(f, "]")
            }
        }
    }
}