use std::alloc::Layout;
use std::fmt::{Debug, Display, Formatter};
use malachite::Natural;
use crate::machine::Fault;
use crate::stack::StackChunk;
use crate::stack::vector_chunk::{F32Vector, F64Vector, I16Vector, I32Vector, I64Vector, I8Vector, IntegerVector, NaturalVector, RationalVector, ReferenceVector, TupleVector, U16Vector, U32Vector, U64Vector, U8Vector, VectorVector};
use crate::value::{Reference, Value};
use crate::value::decimal::Decimal;
use crate::value::integer::Integer;
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
    Vector(Box<VectorType>, usize),
    Tuple(usize)
}

impl VectorType {

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
            VectorType::Reference(size) => Layout::array::<Reference>(*size).unwrap(),
            VectorType::Tuple(size) => Layout::array::<Tuple>(*size).unwrap(),
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
        }
    }
}


#[derive(Clone)]
pub enum Vector {
    U8(*mut u8, usize),
    U16(*mut u16, usize),
    U32(*mut u32, usize),
    U64(*mut u64, usize),
    I8(*mut i8, usize),
    I16(*mut i16, usize),
    I32(*mut i32, usize),
    I64(*mut i64, usize),
    F32(*mut f32, usize),
    F64(*mut f64, usize),
    Natural(*mut Natural, usize),
    Integer(*mut malachite::Integer, usize),
    Rational(*mut malachite::Rational, usize),
    Reference(*mut Reference, usize),
    Vector(*mut Vector, usize),
    Tuple(*mut Tuple, usize)
}

impl Vector {

    pub fn into_chunk(self) -> Box<dyn StackChunk> {
        match self {
            Vector::U8(pointer, size) => Box::new(U8Vector(pointer, size)),
            Vector::U16(pointer, size) => Box::new(U16Vector(pointer, size)),
            Vector::U32(pointer, size) => Box::new(U32Vector(pointer, size)),
            Vector::U64(pointer, size) => Box::new(U64Vector(pointer, size)),
            Vector::I8(pointer, size) => Box::new(I8Vector(pointer, size)),
            Vector::I16(pointer, size) => Box::new(I16Vector(pointer, size)),
            Vector::I32(pointer, size) => Box::new(I32Vector(pointer, size)),
            Vector::I64(pointer, size) => Box::new(I64Vector(pointer, size)),
            Vector::F32(pointer, size) => Box::new(F32Vector(pointer, size)),
            Vector::F64(pointer, size) => Box::new(F64Vector(pointer, size)),
            Vector::Natural(pointer, size) => Box::new(NaturalVector(pointer, size)),
            Vector::Integer(pointer, size) => Box::new(IntegerVector(pointer, size)),
            Vector::Rational(pointer, size) => Box::new(RationalVector(pointer, size)),
            Vector::Reference(pointer, size) => Box::new(ReferenceVector(pointer, size)),
            Vector::Vector(pointer, size) => Box::new(VectorVector(pointer, size)),
            Vector::Tuple(pointer, size) => Box::new(TupleVector(pointer, size)),
        }
    }

    pub fn get(&self, index: usize) -> Result<Value, Fault> {
        match self {
            Vector::U8(pointer, max) => {
                if index < *max {
                    unsafe {
                        Ok(Value::Integer(Integer::U8(*pointer.offset(index as isize))))
                    }
                } else {
                    Err(Fault::OutOfBounds)
                }
            },
            Vector::U16(pointer, max) => {
                if index < *max {
                    unsafe {
                        Ok(Value::Integer(Integer::U16(*pointer.offset(index as isize))))
                    }
                } else {
                    Err(Fault::OutOfBounds)
                }
            },
            Vector::U32(pointer, max) => {
                if index < *max {
                    unsafe {
                        Ok(Value::Integer(Integer::U32(*pointer.offset(index as isize))))
                    }
                } else {
                    Err(Fault::OutOfBounds)
                }
            },
            Vector::U64(pointer, max) => {
                if index < *max {
                    unsafe {
                        Ok(Value::Integer(Integer::U64(*pointer.offset(index as isize))))
                    }
                } else {
                    Err(Fault::OutOfBounds)
                }
            },
            Vector::I8(pointer, max) => {
                if index < *max {
                    unsafe {
                        Ok(Value::Integer(Integer::I8(*pointer.offset(index as isize))))
                    }
                } else {
                    Err(Fault::OutOfBounds)
                }
            },
            Vector::I16(pointer, max) => {
                if index < *max {
                    unsafe {
                        Ok(Value::Integer(Integer::I16(*pointer.offset(index as isize))))
                    }
                } else {
                    Err(Fault::OutOfBounds)
                }
            },
            Vector::I32(pointer, max) => {
                if index < *max {
                    unsafe {
                        Ok(Value::Integer(Integer::I32(*pointer.offset(index as isize))))
                    }
                } else {
                    Err(Fault::OutOfBounds)
                }
            },
            Vector::I64(pointer, max) => {
                if index < *max {
                    unsafe {
                        Ok(Value::Integer(Integer::I64(*pointer.offset(index as isize))))
                    }
                } else {
                    Err(Fault::OutOfBounds)
                }
            },
            Vector::Natural(pointer, max) => {
                if index < *max {
                    unsafe {
                        Ok(Value::Integer(Integer::Natural(std::ptr::read(pointer.offset(index as isize)).clone())))
                    }
                } else {
                    Err(Fault::OutOfBounds)
                }
            },
            Vector::Integer(pointer, max) => {
                if index < *max {
                    unsafe {
                        Ok(Value::Integer(Integer::Integer(std::ptr::read(pointer.offset(index as isize)).clone())))
                    }
                } else {
                    Err(Fault::OutOfBounds)
                }
            },
            Vector::F32(pointer, max) => {
                if index < *max {
                    unsafe {
                        Ok(Value::Decimal(Decimal::F32(*pointer.offset(index as isize))))
                    }
                } else {
                    Err(Fault::OutOfBounds)
                }
            },
            Vector::F64(pointer, max) => {
                if index < *max {
                    unsafe {
                        Ok(Value::Decimal(Decimal::F64(*pointer.offset(index as isize))))
                    }
                } else {
                    Err(Fault::OutOfBounds)
                }
            },
            Vector::Rational(pointer, max) => {
                if index < *max {
                    unsafe {
                        Ok(Value::Decimal(Decimal::Rational(std::ptr::read(pointer.offset(index as isize)).clone())))
                    }
                } else {
                    Err(Fault::OutOfBounds)
                }
            },
            Vector::Reference(pointer, max) => {
                if index < *max {
                    unsafe {
                        Ok(Value::Reference(*pointer.offset(index as isize)))
                    }
                } else {
                    Err(Fault::OutOfBounds)
                }
            },
            Vector::Vector(pointer, max) => {
                if index < *max {
                    unsafe {
                        Ok(Value::Vector(std::ptr::read(pointer.offset(index as isize)).clone()))
                    }
                } else {
                    Err(Fault::OutOfBounds)
                }
            },
            Vector::Tuple(pointer, max) => {
                if index < *max {
                    unsafe {
                        Ok(Value::Tuple(std::ptr::read(pointer.offset(index as isize)).clone()))
                    }
                } else {
                    Err(Fault::OutOfBounds)
                }
            }
        }
    }

    pub fn set(&mut self, index: usize, value: Value) -> Result<(), Fault> {
        match (self, value)  {
            (Vector::U8(pointer, max), Value::Integer(Integer::U8(value))) => {
                if index < *max {
                    unsafe {
                        *pointer.offset(index as isize) = value;
                    }
                    Ok(())
                } else {
                    Err(Fault::OutOfBounds)
                }
            },
            (Vector::U16(pointer, max), Value::Integer(Integer::U16(value))) => {
                if index < *max {
                    unsafe {
                        *pointer.offset(index as isize) = value;
                    }
                    Ok(())
                } else {
                    Err(Fault::OutOfBounds)
                }
            },
            (Vector::U32(pointer, max), Value::Integer(Integer::U32(value))) => {
                if index < *max {
                    unsafe {
                        *pointer.offset(index as isize) = value;
                    }
                    Ok(())
                } else {
                    Err(Fault::OutOfBounds)
                }
            },
            (Vector::U64(pointer, max), Value::Integer(Integer::U64(value))) => {
                if index < *max {
                    unsafe {
                        *pointer.offset(index as isize) = value;
                    }
                    Ok(())
                } else {
                    Err(Fault::OutOfBounds)
                }
            },
            (Vector::I8(pointer, max), Value::Integer(Integer::I8(value))) => {
                if index < *max {
                    unsafe {
                        *pointer.offset(index as isize) = value;
                    }
                    Ok(())
                } else {
                    Err(Fault::OutOfBounds)
                }
            },
            (Vector::I16(pointer, max), Value::Integer(Integer::I16(value))) => {
                if index < *max {
                    unsafe {
                        *pointer.offset(index as isize) = value;
                    }
                    Ok(())
                } else {
                    Err(Fault::OutOfBounds)
                }
            },
            (Vector::I32(pointer, max), Value::Integer(Integer::I32(value))) => {
                if index < *max {
                    unsafe {
                        *pointer.offset(index as isize) = value;
                    }
                    Ok(())
                } else {
                    Err(Fault::OutOfBounds)
                }
            },
            (Vector::I64(pointer, max), Value::Integer(Integer::I64(value))) => {
                if index < *max {
                    unsafe {
                        *pointer.offset(index as isize) = value;
                    }
                    Ok(())
                } else {
                    Err(Fault::OutOfBounds)
                }
            },
            (Vector::Natural(pointer, max), Value::Integer(Integer::Natural(value))) => {
                if index < *max {
                    unsafe {
                        *pointer.offset(index as isize) = value;
                    }
                    Ok(())
                } else {
                    Err(Fault::OutOfBounds)
                }
            },
            (Vector::Integer(pointer, max), Value::Integer(Integer::Integer(value))) => {
                if index < *max {
                    unsafe {
                        *pointer.offset(index as isize) = value;
                    }
                    Ok(())
                } else {
                    Err(Fault::OutOfBounds)
                }
            }
            (Vector::F32(pointer, max), Value::Decimal(Decimal::F32(value))) => {
                if index < *max {
                    unsafe {
                        *pointer.offset(index as isize) = value;
                    }
                    Ok(())
                } else {
                    Err(Fault::OutOfBounds)
                }
            },
            (Vector::F64(pointer, max), Value::Decimal(Decimal::F64(value))) => {
                if index < *max {
                    unsafe {
                        *pointer.offset(index as isize) = value;
                    }
                    Ok(())
                } else {
                    Err(Fault::OutOfBounds)
                }
            },
            (Vector::Rational(pointer, max), Value::Decimal(Decimal::Rational(value))) => {
                if index < *max {
                    unsafe {
                        *pointer.offset(index as isize) = value;
                    }
                    Ok(())
                } else {
                    Err(Fault::OutOfBounds)
                }
            },
            (Vector::Reference(pointer, max), Value::Reference(value)) => {
                if index < *max {
                    unsafe {
                        *pointer.offset(index as isize) = value;
                    }
                    Ok(())
                } else {
                    Err(Fault::OutOfBounds)
                }
            },
            (Vector::Vector(pointer, max), Value::Vector(value)) => {
                if index < *max {
                    unsafe {
                        *pointer.offset(index as isize) = value;
                    }
                    Ok(())
                } else {
                    Err(Fault::OutOfBounds)
                }
            },
            (Vector::Tuple(pointer, max), Value::Tuple(value)) => {
                if index < *max {
                    unsafe {
                        *pointer.offset(index as isize) = value;
                    }
                    Ok(())
                } else {
                    Err(Fault::OutOfBounds)
                }
            }
            _ => Err(Fault::TypeMismatch)
        }
    }

    pub fn free(self) {
        match self {
            Vector::U8(pointer, size) => unsafe {
                std::alloc::dealloc(pointer as *mut u8, Layout::array::<u8>(size).unwrap());
            },
            Vector::U16(pointer, size) => unsafe {
                std::alloc::dealloc(pointer as *mut u8, Layout::array::<u16>(size).unwrap());
            },
            Vector::U32(pointer, size) => unsafe {
                std::alloc::dealloc(pointer as *mut u8, Layout::array::<u32>(size).unwrap());
            },
            Vector::U64(pointer, size) => unsafe {
                std::alloc::dealloc(pointer as *mut u8, Layout::array::<u64>(size).unwrap());
            },
            Vector::I8(pointer, size) => unsafe {
                std::alloc::dealloc(pointer as *mut u8, Layout::array::<i8>(size).unwrap());
            },
            Vector::I16(pointer, size) => unsafe {
                std::alloc::dealloc(pointer as *mut u8, Layout::array::<i16>(size).unwrap());
            },
            Vector::I32(pointer, size) => unsafe {
                std::alloc::dealloc(pointer as *mut u8, Layout::array::<i32>(size).unwrap());
            },
            Vector::I64(pointer, size) => unsafe {
                std::alloc::dealloc(pointer as *mut u8, Layout::array::<i64>(size).unwrap());
            },
            Vector::F32(pointer, size) => unsafe {
                std::alloc::dealloc(pointer as *mut u8, Layout::array::<f32>(size).unwrap());
            },
            Vector::F64(pointer, size) => unsafe {
                std::alloc::dealloc(pointer as *mut u8, Layout::array::<f64>(size).unwrap());
            },
            Vector::Natural(pointer, size) => unsafe {
                std::alloc::dealloc(pointer as *mut u8, Layout::array::<Natural>(size).unwrap());
            },
            Vector::Integer(pointer, size) => unsafe {
                std::alloc::dealloc(pointer as *mut u8, Layout::array::<malachite::Integer>(size).unwrap());
            },
            Vector::Rational(pointer, size) => unsafe {
                std::alloc::dealloc(pointer as *mut u8, Layout::array::<malachite::Rational>(size).unwrap());
            },
            Vector::Reference(pointer, size) => unsafe {
                std::alloc::dealloc(pointer as *mut u8, Layout::array::<Reference>(size).unwrap());
            },
            Vector::Vector(pointer, size) => unsafe {
                for i in 0..size {
                    std::ptr::read(pointer.offset(i as isize)).free();
                }
                std::alloc::dealloc(pointer as *mut u8, Layout::array::<Vector>(size).unwrap());
            },
            Vector::Tuple(pointer, size) => unsafe {
                std::alloc::dealloc(pointer as *mut u8, Layout::array::<Tuple>(size).unwrap());
            }
        }
    }
}

impl Display for Vector {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Vector::U8(pointer, size) => {
                write!(f, "[")?;
                for i in 0..*size {
                    unsafe {
                        write!(f, "{}", *pointer.offset(i as isize))?;
                    }
                    if i != *size - 1 {
                        write!(f, ", ")?;
                    }
                }
                write!(f, "]")
            },
            Vector::U16(pointer, size) => {
                write!(f, "[")?;
                for i in 0..*size {
                    unsafe {
                        write!(f, "{}", *pointer.offset(i as isize))?;
                    }
                    if i != *size - 1 {
                        write!(f, ", ")?;
                    }
                }
                write!(f, "]")
            },
            Vector::U32(pointer, size) => {
                write!(f, "[")?;
                for i in 0..*size {
                    unsafe {
                        write!(f, "{}", *pointer.offset(i as isize))?;
                    }
                    if i != *size - 1 {
                        write!(f, ", ")?;
                    }
                }
                write!(f, "]")
            },
            Vector::U64(pointer, size) => {
                write!(f, "[")?;
                for i in 0..*size {
                    unsafe {
                        write!(f, "{}", *pointer.offset(i as isize))?;
                    }
                    if i != *size - 1 {
                        write!(f, ", ")?;
                    }
                }
                write!(f, "]")
            },
            Vector::I8(pointer, size) => {
                write!(f, "[")?;
                for i in 0..*size {
                    unsafe {
                        write!(f, "{}", *pointer.offset(i as isize))?;
                    }
                    if i != *size - 1 {
                        write!(f, ", ")?;
                    }
                }
                write!(f, "]")
            },
            Vector::I16(pointer, size) => {
                write!(f, "[")?;
                for i in 0..*size {
                    unsafe {
                        write!(f, "{}", *pointer.offset(i as isize))?;
                    }
                    if i != *size - 1 {
                        write!(f, ", ")?;
                    }
                }
                write!(f, "]")
            },
            Vector::I32(pointer, size) => {
                write!(f, "[")?;
                for i in 0..*size {
                    unsafe {
                        write!(f, "{}", *pointer.offset(i as isize))?;
                    }
                    if i != *size - 1 {
                        write!(f, ", ")?;
                    }
                }
                write!(f, "]")
            },
            Vector::I64(pointer, size) => {
                write!(f, "[")?;
                for i in 0..*size {
                    unsafe {
                        write!(f, "{}", *pointer.offset(i as isize))?;
                    }
                    if i != *size - 1 {
                        write!(f, ", ")?;
                    }
                }
                write!(f, "]")
            },
            Vector::F32(pointer, size) => {
                write!(f, "[")?;
                for i in 0..*size {
                    unsafe {
                        write!(f, "{:.2}", *pointer.offset(i as isize))?;
                    }
                    if i != *size - 1 {
                        write!(f, ", ")?;
                    }
                }
                write!(f, "]")
            },
            Vector::F64(pointer, size) => {
                write!(f, "[")?;
                for i in 0..*size {
                    unsafe {
                        write!(f, "{:.2}", *pointer.offset(i as isize))?;
                    }
                    if i != *size - 1 {
                        write!(f, ", ")?;
                    }
                }
                write!(f, "]")
            },
            Vector::Natural(pointer, size) => {
                write!(f, "[")?;
                for i in 0..*size {
                    unsafe {
                        write!(f, "{}", *pointer.offset(i as isize))?;
                    }
                    if i != *size - 1 {
                        write!(f, " ")?;
                    }
                }
                write!(f, "]")
            },
            Vector::Integer(pointer, size) => {
                write!(f, "[")?;
                for i in 0..*size {
                    unsafe {
                        write!(f, "{}", *pointer.offset(i as isize))?;
                    }
                    if i != *size - 1 {
                        write!(f, " ")?;
                    }
                }
                write!(f, "]")
            },
            Vector::Rational(pointer, size) => {
                write!(f, "[")?;
                for i in 0..*size {
                    unsafe {
                        write!(f, "{:.2}", *pointer.offset(i as isize))?;
                    }
                    if i != *size - 1 {
                        write!(f, " ")?;
                    }
                }
                write!(f, "]")
            },
            Vector::Reference(pointer, size) => {
                write!(f, "[")?;
                for i in 0..*size {
                    unsafe {
                        write!(f, "{}", *pointer.offset(i as isize))?;
                    }
                    if i != *size - 1 {
                        write!(f, " ")?;
                    }
                }
                write!(f, "]")
            },
            Vector::Vector(pointer, size) => {
                write!(f, "[")?;
                for i in 0..*size {
                    unsafe {
                        write!(f, "{}", *pointer.offset(i as isize))?;
                    }
                    if i != *size - 1 {
                        write!(f, " ")?;
                    }
                }
                write!(f, "]")
            },
            Vector::Tuple(pointer, size) => {
                write!(f, "[")?;
                for i in 0..*size {
                    unsafe {
                        write!(f, "{:?}", *pointer.offset(i as isize))?;
                    }
                    if i != *size - 1 {
                        write!(f, " ")?;
                    }
                }
                write!(f, "]")
            },
        }
    }
}

impl Debug for Vector {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Vector::U8(_, _) => write!(f, "{}", self),
            Vector::U16(_, _) => write!(f, "{}", self),
            Vector::U32(_, _) => write!(f, "{}", self),
            Vector::U64(_, _) => write!(f, "{}", self),
            Vector::I8(_, _) => write!(f, "{}", self),
            Vector::I16(_, _) => write!(f, "{}", self),
            Vector::I32(_, _) => write!(f, "{}", self),
            Vector::I64(_, _) => write!(f, "{}", self),
            Vector::F32(_, _) => write!(f, "{}", self),
            Vector::F64(_, _) => write!(f, "{}", self),
            Vector::Natural(pointer, size) => {
                write!(f, "[")?;
                for i in 0..*size {
                    unsafe {
                        write!(f, "{}", *pointer.offset(i as isize))?;
                    }
                    if i != *size - 1 {
                        write!(f, " ")?;
                    }
                }
                write!(f, "]")
            },
            Vector::Integer(pointer, size) => {
                write!(f, "[")?;
                for i in 0..*size {
                    unsafe {
                        write!(f, "{:?}", *pointer.offset(i as isize))?;
                    }
                    if i != *size - 1 {
                        write!(f, " ")?;
                    }
                }
                write!(f, "]")
            },
            Vector::Rational(pointer, size) => {
                write!(f, "[")?;
                for i in 0..*size {
                    unsafe {
                        write!(f, "{:?}", *pointer.offset(i as isize))?;
                    }
                    if i != *size - 1 {
                        write!(f, " ")?;
                    }
                }
                write!(f, "]")
            },
            Vector::Reference(_, _) => write!(f, "{}", self),
            Vector::Vector(pointer, size) => {
                write!(f, "[")?;
                for i in 0..*size {
                    unsafe {
                        write!(f, "{:?}", *pointer.offset(i as isize))?;
                    }
                    if i != *size - 1 {
                        write!(f, " ")?;
                    }
                }
                write!(f, "]")
            },
            Vector::Tuple(pointer, size) => {
                write!(f, "[")?;
                for i in 0..*size {
                    unsafe {
                        write!(f, "{:?}", *pointer.offset(i as isize))?;
                    }
                    if i != *size - 1 {
                        write!(f, " ")?;
                    }
                }
                write!(f, "]")
            }
        }
    }
}