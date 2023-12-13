use std::fmt::{Debug, Display};
use crate::value::natural::Natural;

#[derive(Clone, PartialEq)]
pub enum Integer {
    U8(u8),
    U16(u16),
    U32(u32),
    U64(u64),
    I8(i8),
    I16(i16),
    I32(i32),
    I64(i64),
    Natural(Natural),
    Integer(bool, Natural),
}


impl Display for Integer {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Integer::U8(value) => write!(f, "{}", value),
            Integer::U16(value) => write!(f, "{}", value),
            Integer::U32(value) => write!(f, "{}", value),
            Integer::U64(value) => write!(f, "{}", value),
            Integer::I8(value) => write!(f, "{}", value),
            Integer::I16(value) => write!(f, "{}", value),
            Integer::I32(value) => write!(f, "{}", value),
            Integer::I64(value) => write!(f, "{}", value),
            Integer::Natural(value) => write!(f, "{}", value),
            Integer::Integer(sign, value) => {
                if *sign {
                    write!(f, "-{}", value)
                } else {
                    write!(f, "{}", value)
                }
            }
        }
    }
}

impl Debug for Integer {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Integer::U8(value) => write!(f, "{:?}", value),
            Integer::U16(value) => write!(f, "{:?}", value),
            Integer::U32(value) => write!(f, "{:?}", value),
            Integer::U64(value) => write!(f, "{:?}", value),
            Integer::I8(value) => write!(f, "{:?}", value),
            Integer::I16(value) => write!(f, "{:?}", value),
            Integer::I32(value) => write!(f, "{:?}", value),
            Integer::I64(value) => write!(f, "{:?}", value),
            Integer::Natural(value) => write!(f, "{:?}", value),
            Integer::Integer(sign, value) => {
                if *sign {
                    write!(f, "-{:?}", value)
                } else {
                    write!(f, "{:?}", value)
                }
            }
        }
    }
}


