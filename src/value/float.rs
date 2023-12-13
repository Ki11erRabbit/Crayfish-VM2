use std::fmt::{Debug, Display};

#[derive(Clone, PartialEq, Copy, PartialOrd)]
pub enum Float {
    F32(f32),
    F64(f64),
}


impl Display for Float {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Float::F32(value) => write!(f, "{}", value),
            Float::F64(value) => write!(f, "{}", value),
        }
    }
}

impl Debug for Float {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Float::F32(value) => write!(f, "{:?}", value),
            Float::F64(value) => write!(f, "{:?}", value),
        }
    }
}