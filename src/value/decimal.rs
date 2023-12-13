use std::fmt::{Debug, Display};

#[derive(Clone, PartialEq, PartialOrd)]
pub enum Decimal {
    F32(f32),
    F64(f64),
    Rational(malachite::Rational),
}


impl std::ops::Add for Decimal {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        match (self, rhs) {
            (Decimal::F32(left), Decimal::F32(right)) => Decimal::F32(left + right),
            (Decimal::F64(left), Decimal::F64(right)) => Decimal::F64(left + right),
            (Decimal::Rational(left), Decimal::Rational(right)) => Decimal::Rational(left + right),
            (Decimal::F32(left), Decimal::F64(right)) => Decimal::F64(left as f64 + right),
            (Decimal::F64(left), Decimal::F32(right)) => Decimal::F64(left + right as f64),
            (x, y) => panic!("Cannot add {:?} and {:?}", x, y),
        }
    }

}

impl std::ops::Sub for Decimal {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        match (self, rhs) {
            (Decimal::F32(left), Decimal::F32(right)) => Decimal::F32(left - right),
            (Decimal::F64(left), Decimal::F64(right)) => Decimal::F64(left - right),
            (Decimal::Rational(left), Decimal::Rational(right)) => Decimal::Rational(left - right),
            (Decimal::F32(left), Decimal::F64(right)) => Decimal::F64(left as f64 - right),
            (Decimal::F64(left), Decimal::F32(right)) => Decimal::F64(left - right as f64),
            (x, y) => panic!("Cannot subtract {:?} and {:?}", x, y),
        }
    }

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