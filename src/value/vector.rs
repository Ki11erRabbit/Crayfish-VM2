use std::fmt::{Debug, Display, Formatter};
use malachite::Natural;
use crate::stack::StackChunk;
use crate::value::Reference;

#[derive(Debug, Clone)]
pub enum VectorType {
    U8,
    U16,
    U32,
    U64,
    I8,
    I16,
    I32,
    I64,
    F32,
    F64,
    Natural,
    Integer,
    Rational,
    Reference,
}

impl Display for VectorType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            VectorType::U8 => write!(f, "u8"),
            VectorType::U16 => write!(f, "u16"),
            VectorType::U32 => write!(f, "u32"),
            VectorType::U64 => write!(f, "u64"),
            VectorType::I8 => write!(f, "i8"),
            VectorType::I16 => write!(f, "i16"),
            VectorType::I32 => write!(f, "i32"),
            VectorType::I64 => write!(f, "i64"),
            VectorType::F32 => write!(f, "f32"),
            VectorType::F64 => write!(f, "f64"),
            VectorType::Natural => write!(f, "natural"),
            VectorType::Integer => write!(f, "integer"),
            VectorType::Rational => write!(f, "rational"),
            VectorType::Reference => write!(f, "reference"),
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
}

impl Vector {

    pub fn into_chunk(self) -> Box<dyn StackChunk> {
        match self {
            Vector::U8(pointer, size) => Box::new((pointer, size)),
            Vector::U16(pointer, size) => Box::new((pointer, size)),
            Vector::U32(pointer, size) => Box::new((pointer, size)),
            Vector::U64(pointer, size) => Box::new((pointer, size)),
            Vector::I8(pointer, size) => Box::new((pointer, size)),
            Vector::I16(pointer, size) => Box::new((pointer, size)),
            Vector::I32(pointer, size) => Box::new((pointer, size)),
            Vector::I64(pointer, size) => Box::new((pointer, size)),
            Vector::F32(pointer, size) => Box::new((pointer, size)),
            Vector::F64(pointer, size) => Box::new((pointer, size)),
            Vector::Natural(pointer, size) => Box::new((pointer, size)),
            Vector::Integer(pointer, size) => Box::new((pointer, size)),
            Vector::Rational(pointer, size) => Box::new((pointer, size)),
            Vector::Reference(pointer, size) => Box::new((pointer, size)),
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
        }
    }
}