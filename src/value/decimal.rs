use std::alloc::Layout;
use std::fmt::{Debug, Display};
use malachite::num::arithmetic::traits::Pow;
use malachite::num::basic::traits::Zero;
use crate::stack::StackChunk;
use crate::value::integer::{Integer, IntegerType};
use crate::value::{Value, ValueType};

#[derive(Debug,Clone, PartialEq, PartialOrd)]
pub enum DecimalType {
    F32,
    F64,
    Rational,
}

impl DecimalType {

    pub fn get_array_layout(&self, size: usize) -> Layout {
        match self {
            DecimalType::F32 => Layout::array::<f32>(size).unwrap(),
            DecimalType::F64 => Layout::array::<f64>(size).unwrap(),
            DecimalType::Rational => Layout::array::<malachite::Rational>(size).unwrap(),
        }
    }
}

impl Display for DecimalType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            DecimalType::F32 => write!(f, "f32"),
            DecimalType::F64 => write!(f, "f64"),
            DecimalType::Rational => write!(f, "rational"),
        }
    }
}

#[derive(Clone)]
pub enum Decimal {
    F32(f32),
    F64(f64),
    Rational(malachite::Rational),
}


impl Decimal {

    pub fn cast(self, into_type: ValueType) -> Value {
        match into_type {
            ValueType::Decimal(DecimalType::F32) => Value::Decimal(Decimal::F32(self.into())),
            ValueType::Decimal(DecimalType::F64) => Value::Decimal(Decimal::F64(self.into())),
            ValueType::Boolean => Value::Boolean(self.into()),
            ValueType::Integer(IntegerType::I8) => Value::Integer(Integer::I8(self.into())),
            ValueType::Integer(IntegerType::I16) => Value::Integer(Integer::I16(self.into())),
            ValueType::Integer(IntegerType::I32) => Value::Integer(Integer::I32(self.into())),
            ValueType::Integer(IntegerType::I64) => Value::Integer(Integer::I64(self.into())),
            ValueType::Integer(IntegerType::U8) => Value::Integer(Integer::U8(self.into())),
            ValueType::Integer(IntegerType::U16) => Value::Integer(Integer::U16(self.into())),
            ValueType::Integer(IntegerType::U32) => Value::Integer(Integer::U32(self.into())),
            ValueType::Integer(IntegerType::U64) => Value::Integer(Integer::U64(self.into())),
            x => panic!("Cannot cast decimal to {:?}", x),
        }
    }

    pub fn is_zero(&self) -> bool {
        match self {
            Decimal::F32(value) => *value == 0.0,
            Decimal::F64(value) => *value == 0.0,
            Decimal::Rational(value) => value == &malachite::Rational::ZERO,
        }
    }

    pub fn is_negative(&self) -> bool {
        match self {
            Decimal::F32(value) => *value < 0.0,
            Decimal::F64(value) => *value < 0.0,
            Decimal::Rational(value) => value < &malachite::Rational::ZERO,
        }
    }

    pub fn into_chunk(self) -> Box<dyn StackChunk> {
        match self {
            Decimal::F32(value) => Box::new(value),
            Decimal::F64(value) => Box::new(value),
            Decimal::Rational(value) => Box::new(value),
        }
    }

    pub fn powd(self, exponent: Self) -> Self {
        match (self, exponent) {
            (Decimal::F32(left), Decimal::F32(right)) => Decimal::F32(left.powf(right)),
            (Decimal::F64(left), Decimal::F64(right)) => Decimal::F64(left.powf(right)),
            (Decimal::F32(left), Decimal::F64(right)) => Decimal::F64((left as f64).powf(right)),
            (Decimal::F64(left), Decimal::F32(right)) => Decimal::F64(left.powf(right as f64)),
            (x, y) => panic!("Cannot raise {:?} to {:?}", x, y),
        }
    }

    /// Raises the decimal to the power of the integer.
    /// For Rationals the exponent must be a i64.
    /// For floats the exponent must be a i32.
    pub fn powi(self, exponent: Integer) -> Self {

        match self {
            Decimal::F32(value) => {
                let exponent: i32 = exponent.into();
                Decimal::F32(value.powi(exponent))
            },
            Decimal::F64(value) => {
                let exponent: i32 = exponent.into();
                Decimal::F64(value.powi(exponent))
            },
            Decimal::Rational(value) => {
                let exponent: i64 = exponent.into();
                Decimal::Rational(value.pow(exponent))
            },
        }
    }

    pub fn is_rational(&self) -> bool {
        match self {
            Decimal::Rational(_) => true,
            _ => false,
        }
    }


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

impl std::ops::Mul for Decimal {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        match (self, rhs) {
            (Decimal::F32(left), Decimal::F32(right)) => Decimal::F32(left * right),
            (Decimal::F64(left), Decimal::F64(right)) => Decimal::F64(left * right),
            (Decimal::Rational(left), Decimal::Rational(right)) => Decimal::Rational(left * right),
            (Decimal::F32(left), Decimal::F64(right)) => Decimal::F64(left as f64 * right),
            (Decimal::F64(left), Decimal::F32(right)) => Decimal::F64(left * right as f64),
            (x, y) => panic!("Cannot multiply {:?} and {:?}", x, y),
        }
    }

}

impl std::ops::Div for Decimal {
    type Output = Self;

    fn div(self, rhs: Self) -> Self::Output {
        match (self, rhs) {
            (Decimal::F32(left), Decimal::F32(right)) => Decimal::F32(left / right),
            (Decimal::F64(left), Decimal::F64(right)) => Decimal::F64(left / right),
            (Decimal::Rational(left), Decimal::Rational(right)) => Decimal::Rational(left / right),
            (Decimal::F32(left), Decimal::F64(right)) => Decimal::F64(left as f64 / right),
            (Decimal::F64(left), Decimal::F32(right)) => Decimal::F64(left / right as f64),
            (x, y) => panic!("Cannot divide {:?} and {:?}", x, y),
        }
    }

}

impl std::ops::Rem for Decimal {
    type Output = Self;

    fn rem(self, rhs: Self) -> Self::Output {
        match (self, rhs) {
            (Decimal::F32(left), Decimal::F32(right)) => Decimal::F32(left % right),
            (Decimal::F64(left), Decimal::F64(right)) => Decimal::F64(left % right),
            (Decimal::F32(left), Decimal::F64(right)) => Decimal::F64(left as f64 % right),
            (Decimal::F64(left), Decimal::F32(right)) => Decimal::F64(left % right as f64),
            (x, y) => panic!("Cannot divide {:?} and {:?}", x, y),
        }
    }

}

impl std::ops::Neg for Decimal {
    type Output = Self;

    fn neg(self) -> Self::Output {
        match self {
            Decimal::F32(value) => Decimal::F32(-value),
            Decimal::F64(value) => Decimal::F64(-value),
            Decimal::Rational(value) => Decimal::Rational(-value),
        }
    }

}

impl PartialEq<Decimal> for Decimal {
    fn eq(&self, other: &Decimal) -> bool {
        match (self, other) {
            (Decimal::F32(left), Decimal::F32(right)) => left == right,
            (Decimal::F64(left), Decimal::F64(right)) => left == right,
            (Decimal::Rational(left), Decimal::Rational(right)) => left == right,
            (x, y) => panic!("Cannot compare {:?} and {:?}", x, y),
        }
    }
}

impl PartialOrd<Decimal> for Decimal {
    fn partial_cmp(&self, other: &Decimal) -> Option<std::cmp::Ordering> {
        match (self, other) {
            (Decimal::F32(left), Decimal::F32(right)) => left.partial_cmp(right),
            (Decimal::F64(left), Decimal::F64(right)) => left.partial_cmp(right),
            (Decimal::Rational(left), Decimal::Rational(right)) => left.partial_cmp(right),
            (x, y) => panic!("Cannot compare {:?} and {:?}", x, y),
        }
    }
}

impl From<u8> for Decimal {
    fn from(value: u8) -> Self {
        Decimal::F32(value as f32)
    }
}

impl From<u16> for Decimal {
    fn from(value: u16) -> Self {
        Decimal::F32(value as f32)
    }
}

impl From<u32> for Decimal {
    fn from(value: u32) -> Self {
        Decimal::F32(value as f32)
    }
}

impl From<u64> for Decimal {
    fn from(value: u64) -> Self {
        Decimal::F64(value as f64)
    }
}

impl From<i8> for Decimal {
    fn from(value: i8) -> Self {
        Decimal::F32(value as f32)
    }
}

impl From<i16> for Decimal {
    fn from(value: i16) -> Self {
        Decimal::F32(value as f32)
    }
}

impl From<i32> for Decimal {
    fn from(value: i32) -> Self {
        Decimal::F32(value as f32)
    }
}

impl From<i64> for Decimal {
    fn from(value: i64) -> Self {
        Decimal::F64(value as f64)
    }
}

impl From<f32> for Decimal {
    fn from(value: f32) -> Self {
        Decimal::F32(value)
    }
}

impl From<f64> for Decimal {
    fn from(value: f64) -> Self {
        Decimal::F64(value)
    }
}

impl From<malachite::Rational> for Decimal {
    fn from(value: malachite::Rational) -> Self {
        Decimal::Rational(value)
    }
}

impl From<Integer> for Decimal {
    fn from(value: Integer) -> Self {
        match value {
            Integer::U8(value) => value.into(),
            Integer::U16(value) => value.into(),
            Integer::U32(value) => value.into(),
            Integer::U64(value) => value.into(),
            Integer::I8(value) => value.into(),
            Integer::I16(value) => value.into(),
            Integer::I32(value) => value.into(),
            Integer::I64(value) => value.into(),
            x => panic!("Cannot convert {:?} to decimal", x),
        }
    }
}

impl From<Decimal> for bool {
    fn from(value: Decimal) -> Self {
        !value.is_zero()
    }
}

macro_rules! generate_from_decimal {
    ($type:ty) => {
        impl From<Decimal> for $type {
            fn from(value: Decimal) -> Self {
                match value {
                    Decimal::F32(value) => value as $type,
                    Decimal::F64(value) => value as $type,
                    x => panic!("Cannot convert {:?} to {}", x, stringify!($type)),
                }
            }
        }
    }
}

generate_from_decimal!(u8);
generate_from_decimal!(u16);
generate_from_decimal!(u32);
generate_from_decimal!(u64);
generate_from_decimal!(i8);
generate_from_decimal!(i16);
generate_from_decimal!(i32);
generate_from_decimal!(i64);
generate_from_decimal!(f32);
generate_from_decimal!(f64);







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