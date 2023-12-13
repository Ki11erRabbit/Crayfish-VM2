use std::fmt::{Debug, Display};
use malachite::Natural;
use malachite::num::basic::traits::Zero;
use crate::value::{Value, ValueType};
use crate::value::decimal::DecimalType;

#[derive(Debug,Clone, PartialEq, PartialOrd)]
pub enum IntegerType {
    U8,
    U16,
    U32,
    U64,
    I8,
    I16,
    I32,
    I64,
    Natural,
    Integer,
}

impl Display for IntegerType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            IntegerType::U8 => write!(f, "u8"),
            IntegerType::U16 => write!(f, "u16"),
            IntegerType::U32 => write!(f, "u32"),
            IntegerType::U64 => write!(f, "u64"),
            IntegerType::I8 => write!(f, "i8"),
            IntegerType::I16 => write!(f, "i16"),
            IntegerType::I32 => write!(f, "i32"),
            IntegerType::I64 => write!(f, "i64"),
            IntegerType::Natural => write!(f, "natural"),
            IntegerType::Integer => write!(f, "integer"),
        }
    }
}


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
    Integer(malachite::Integer),
}

impl Integer {
    pub fn cast(self, into_type: ValueType) -> Value {
        match into_type {
            ValueType::Integer(IntegerType::I8) => Value::Integer(Integer::I8(self.into())),
            ValueType::Integer(IntegerType::I16) => Value::Integer(Integer::I16(self.into())),
            ValueType::Integer(IntegerType::I32) => Value::Integer(Integer::I32(self.into())),
            ValueType::Integer(IntegerType::I64) => Value::Integer(Integer::I64(self.into())),
            ValueType::Integer(IntegerType::U8) => Value::Integer(Integer::U8(self.into())),
            ValueType::Integer(IntegerType::U16) => Value::Integer(Integer::U16(self.into())),
            ValueType::Integer(IntegerType::U32) => Value::Integer(Integer::U32(self.into())),
            ValueType::Integer(IntegerType::U64) => Value::Integer(Integer::U64(self.into())),
            ValueType::Integer(IntegerType::Natural) => Value::Integer(Integer::Natural(self.into())),
            ValueType::Integer(IntegerType::Integer) => Value::Integer(Integer::Integer(self.into())),
            ValueType::Boolean => Value::Boolean(self.into()),
            ValueType::Decimal(DecimalType::F32) => Value::Decimal(self.into()),
            ValueType::Decimal(DecimalType::F64) => Value::Decimal(self.into()),
            ValueType::Decimal(DecimalType::Rational) => Value::Decimal(self.into()),
            _ => panic!("Cannot cast integer to {}", into_type),
        }
    }

    pub fn is_zero(&self) -> bool {
        match self {
            Integer::U8(value) => *value == 0,
            Integer::U16(value) => *value == 0,
            Integer::U32(value) => *value == 0,
            Integer::U64(value) => *value == 0,
            Integer::I8(value) => *value == 0,
            Integer::I16(value) => *value == 0,
            Integer::I32(value) => *value == 0,
            Integer::I64(value) => *value == 0,
            Integer::Natural(value) => value == &Natural::ZERO,
            Integer::Integer(value) => value == &malachite::Integer::ZERO,
        }
    }

    pub fn is_negative(&self) -> bool {
        match self {
            Integer::U8(_) => false,
            Integer::U16(_) => false,
            Integer::U32(_) => false,
            Integer::U64(_) => false,
            Integer::I8(value) => *value < 0,
            Integer::I16(value) => *value < 0,
            Integer::I32(value) => *value < 0,
            Integer::I64(value) => *value < 0,
            Integer::Natural(_) => false,
            Integer::Integer(value) => value < &malachite::Integer::ZERO,
        }
    }
}

macro_rules! generate_op {
    ($Opname:ident, $fun_name:ident, $op:tt) => {
        impl std::ops::$Opname for Integer {
            type Output = Self;
        
            fn $fun_name(self, rhs: Self) -> Self::Output {
                match (self, rhs) {
                    (Integer::U8(left), Integer::U8(right)) => Integer::U8(left $op right),
                    (Integer::U16(left), Integer::U16(right)) => Integer::U16(left $op right),
                    (Integer::U32(left), Integer::U32(right)) => Integer::U32(left $op right),
                    (Integer::U64(left), Integer::U64(right)) => Integer::U64(left $op right),
                    (Integer::I8(left), Integer::I8(right)) => Integer::I8(left $op right),
                    (Integer::I16(left), Integer::I16(right)) => Integer::I16(left $op right),
                    (Integer::I32(left), Integer::I32(right)) => Integer::I32(left $op right),
                    (Integer::I64(left), Integer::I64(right)) => Integer::I64(left $op right),
                    (Integer::Natural(left), Integer::Natural(right)) => Integer::Natural(left $op right),
                    (Integer::Integer(left), Integer::Integer(right)) => Integer::Integer(left $op right),
                    (x, y) => panic!("Cannot add {:?} and {:?}", x, y),
                }
            }
        }
    }
}

macro_rules! generate_shift {
    ($Opname:ident, $fun_name:ident, $op:tt) => {
        impl std::ops::$Opname for Integer {
            type Output = Self;

            fn $fun_name(self, rhs: Self) -> Self::Output {
                match (self, rhs) {
                    (Integer::U8(left), Integer::U8(right)) => Integer::U8(left $op right),
                    (Integer::U16(left), Integer::U16(right)) => Integer::U16(left $op right),
                    (Integer::U32(left), Integer::U32(right)) => Integer::U32(left $op right),
                    (Integer::U64(left), Integer::U64(right)) => Integer::U64(left $op right),
                    (Integer::I8(left), Integer::I8(right)) => Integer::I8(left $op right),
                    (Integer::I16(left), Integer::I16(right)) => Integer::I16(left $op right),
                    (Integer::I32(left), Integer::I32(right)) => Integer::I32(left $op right),
                    (Integer::I64(left), Integer::I64(right)) => Integer::I64(left $op right),
                    (x, y) => panic!("Cannot add {:?} and {:?}", x, y),
                }
            }
        }
    }
}


generate_op!(Add, add, +);
generate_op!(Sub, sub, -);
generate_op!(Mul, mul, *);
generate_op!(Div, div, /);
generate_op!(Rem, rem, %);
generate_op!(BitAnd, bitand, &);
generate_op!(BitOr, bitor, |);
generate_op!(BitXor, bitxor, ^);
generate_shift!(Shl, shl, <<);
generate_shift!(Shr, shr, >>);



macro_rules! generate_from_base {
    ($id:ident, $type:ty) => {
        impl From<$type> for Integer {
            fn from(value: $type) -> Self {
                Integer::$id(value)
            }
        }
    }
}

macro_rules! generate_from_integer {
    ($id:ident, $type:ty) => {
        impl From<Integer> for $type {
            fn from(value: Integer) -> Self {
                match value {
                    Integer::$id(value) => value,
                    _ => panic!("Cannot convert {:?} to {}", value, stringify!($type)),
                }
            }
        }
    }
}

generate_from_base!(U8, u8);
generate_from_base!(U16, u16);
generate_from_base!(U32, u32);
generate_from_base!(U64, u64);
generate_from_base!(I8, i8);
generate_from_base!(I16, i16);
generate_from_base!(I32, i32);
generate_from_base!(I64, i64);
generate_from_base!(Natural, Natural);
generate_from_base!(Integer, malachite::Integer);
generate_from_integer!(U8, u8);
generate_from_integer!(U16, u16);
generate_from_integer!(U32, u32);
generate_from_integer!(U64, u64);
generate_from_integer!(I8, i8);
generate_from_integer!(I16, i16);
generate_from_integer!(I32, i32);
generate_from_integer!(I64, i64);
generate_from_integer!(Natural, Natural);
generate_from_integer!(Integer, malachite::Integer);

impl From<Integer> for bool {
    fn from(value: Integer) -> bool {
        !value.is_zero()
    }
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
            Integer::Integer(value) => write!(f, "{}", value),
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
            Integer::Integer(value) => write!(f, "{:?}", value),
        }
    }
}


