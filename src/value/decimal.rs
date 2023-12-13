use std::fmt::{Debug, Display};

#[derive(Clone, PartialEq, PartialOrd)]
pub enum Decimal {
    F32(f32),
    F64(f64),
    Rational(malachite::Rational),
}


impl Display for Decimal {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Decimal::F32(value) => write!(f, "{}", value),
            Decimal::F64(value) => write!(f, "{}", value),
            Decimal::Rational(value) => write!(f, "{}", value),
        }
    }
}

impl Debug for Decimal {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Decimal::F32(value) => write!(f, "{:?}", value),
            Decimal::F64(value) => write!(f, "{:?}", value),
            Decimal::Rational(value) => write!(f, "{:?}", value),
        }
    }
}