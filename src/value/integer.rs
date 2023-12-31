use std::alloc::Layout;
use std::fmt::{Debug, Display};
use std::ops::{Neg, Not};
use malachite::Natural;
use malachite::num::arithmetic::traits::Pow;
use malachite::num::basic::traits::Zero;
use crate::stack::StackChunk;
use crate::value::{Value, ValueType};
use crate::value::decimal::DecimalType;

#[derive(Debug,Clone, Copy, PartialEq, PartialOrd)]
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

impl IntegerType {
    pub fn get_layout(&self) -> Layout {
        match self {
            IntegerType::U8 => Layout::new::<u8>(),
            IntegerType::U16 => Layout::new::<u16>(),
            IntegerType::U32 => Layout::new::<u32>(),
            IntegerType::U64 => Layout::new::<u64>(),
            IntegerType::I8 => Layout::new::<i8>(),
            IntegerType::I16 => Layout::new::<i16>(),
            IntegerType::I32 => Layout::new::<i32>(),
            IntegerType::I64 => Layout::new::<i64>(),
            IntegerType::Natural => Layout::new::<Natural>(),
            IntegerType::Integer => Layout::new::<malachite::Integer>(),
        }
    }

    pub fn get_array_layout(&self, size: usize) -> Layout {
        match self {
            IntegerType::U8 => Layout::array::<u8>(size).unwrap(),
            IntegerType::U16 => Layout::array::<u16>(size).unwrap(),
            IntegerType::U32 => Layout::array::<u32>(size).unwrap(),
            IntegerType::U64 => Layout::array::<u64>(size).unwrap(),
            IntegerType::I8 => Layout::array::<i8>(size).unwrap(),
            IntegerType::I16 => Layout::array::<i16>(size).unwrap(),
            IntegerType::I32 => Layout::array::<i32>(size).unwrap(),
            IntegerType::I64 => Layout::array::<i64>(size).unwrap(),
            IntegerType::Natural => Layout::array::<Natural>(size).unwrap(),
            IntegerType::Integer => Layout::array::<malachite::Integer>(size).unwrap(),
        }
    }
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


#[derive(Clone)]
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

    pub fn into_chunk(self) -> Box<dyn StackChunk> {
        match self {
            Integer::U8(value) => Box::new(value),
            Integer::U16(value) => Box::new(value),
            Integer::U32(value) => Box::new(value),
            Integer::U64(value) => Box::new(value),
            Integer::I8(value) => Box::new(value),
            Integer::I16(value) => Box::new(value),
            Integer::I32(value) => Box::new(value),
            Integer::I64(value) => Box::new(value),
            Integer::Natural(value) => Box::new(value),
            Integer::Integer(value) => Box::new(value),
        }
    }

    pub fn to_usize(self) -> Option<usize> {
        match self {
            Integer::U8(value) => Some(value as usize),
            Integer::U16(value) => Some(value as usize),
            Integer::U32(value) => Some(value as usize),
            Integer::U64(value) => Some(value as usize),
            Integer::I8(value) => Some(value as usize),
            Integer::I16(value) => Some(value as usize),
            Integer::I32(value) => Some(value as usize),
            Integer::I64(value) => Some(value as usize),
            Integer::Natural(_) => None,
            Integer::Integer(_) => None,
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
                    (x, y) => panic!("Cannot perform op with {:?} and {:?}", x, y),
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

impl Pow<Integer> for Integer {
    type Output = Self;

    fn pow(self, rhs: Integer) -> Self::Output {
        match (self, rhs) {
            (Integer::U8(left), Integer::U32(right)) => Integer::U8(left.pow(right)),
            (Integer::U16(left), Integer::U32(right)) => Integer::U16(left.pow(right)),
            (Integer::U32(left), Integer::U32(right)) => Integer::U32(left.pow(right)),
            (Integer::U64(left), Integer::U32(right)) => Integer::U64(left.pow(right)),
            (Integer::I8(left), Integer::U32(right)) => Integer::I8(left.pow(right)),
            (Integer::I16(left), Integer::U32(right)) => Integer::I16(left.pow(right)),
            (Integer::I32(left), Integer::U32(right)) => Integer::I32(left.pow(right)),
            (Integer::I64(left), Integer::U32(right)) => Integer::I64(left.pow(right)),
            (Integer::Natural(left), Integer::U64(right)) => Integer::Natural(left.pow(right)),
            (Integer::Integer(left), Integer::U64(right)) => Integer::Integer(left.pow(right)),
            (x, y) => panic!("Cannot pow {:?} and {:?}", x, y),
        }
    }

}

impl Neg for Integer {
    type Output = Integer;

    fn neg(self) -> Self::Output {
        match self {
            Integer::U8(value) => {
                if value == 0 {
                    Integer::U8(0)
                } else {
                    if value > 128 {
                        Integer::I16(-(value as i16))
                    } else {
                        Integer::I8(-(value as i8))
                    }
                }
            }
            Integer::U16(value) => {
                if value == 0 {
                    Integer::U16(0)
                } else {
                    if value > 32768 {
                        Integer::I32(-(value as i32))
                    } else {
                        Integer::I16(-(value as i16))
                    }
                }
            }
            Integer::U32(value) => {
                if value == 0 {
                    Integer::U32(0)
                } else {
                    if value > 2147483648 {
                        Integer::I64(-(value as i64))
                    } else {
                        Integer::I32(-(value as i32))
                    }
                }
            }
            Integer::U64(value) => {
                if value == 0 {
                    Integer::U64(0)
                } else {
                    if value > 9223372036854775808 {
                        Integer::Integer(-<u64 as Into<malachite::Integer>>::into(value))
                    } else {
                        Integer::I64(-(value as i64))
                    }
                }
            }
            Integer::I8(value) => {
                if value == 0 {
                    Integer::I8(0)
                } else {
                    Integer::I8(-value)
                }
            }
            Integer::I16(value) => {
                if value == 0 {
                    Integer::I16(0)
                } else {
                    Integer::I16(-value)
                }
            }
            Integer::I32(value) => {
                if value == 0 {
                    Integer::I32(0)
                } else {
                    Integer::I32(-value)
                }
            }
            Integer::I64(value) => {
                if value == 0 {
                    Integer::I64(0)
                } else {
                    Integer::I64(-value)
                }
            }
            Integer::Natural(value) => {
                if value == Natural::ZERO {
                    Integer::Natural(Natural::ZERO)
                } else {
                    Integer::Integer(-<Natural as Into<malachite::Integer>>::into(value))
                }
            }
            Integer::Integer(value) => {
                if value == malachite::Integer::ZERO {
                    Integer::Integer(malachite::Integer::ZERO)
                } else {
                    Integer::Integer(-value)
                }
            }
        }
    }
}

impl Not for Integer {
    type Output = Integer;

    fn not(self) -> Self::Output {
        match self {
            Integer::U8(value) => Integer::U8(!value),
            Integer::U16(value) => Integer::U16(!value),
            Integer::U32(value) => Integer::U32(!value),
            Integer::U64(value) => Integer::U64(!value),
            Integer::I8(value) => Integer::I8(!value),
            Integer::I16(value) => Integer::I16(!value),
            Integer::I32(value) => Integer::I32(!value),
            Integer::I64(value) => Integer::I64(!value),
            Integer::Integer(value) => Integer::Integer(!value),
            x => panic!("Cannot not {:?}", x),
        }
    }
}


impl PartialEq<Integer> for Integer {
    fn eq(&self, other: &Integer) -> bool {
        match (self, other) {
            (Integer::U8(left), Integer::U8(right)) => left == right,
            (Integer::U16(left), Integer::U16(right)) => left == right,
            (Integer::U32(left), Integer::U32(right)) => left == right,
            (Integer::U64(left), Integer::U64(right)) => left == right,
            (Integer::I8(left), Integer::I8(right)) => left == right,
            (Integer::I16(left), Integer::I16(right)) => left == right,
            (Integer::I32(left), Integer::I32(right)) => left == right,
            (Integer::I64(left), Integer::I64(right)) => left == right,
            (Integer::Natural(left), Integer::Natural(right)) => left == right,
            (Integer::Integer(left), Integer::Integer(right)) => left == right,
            (x, y) => panic!("Cannot compare {:?} and {:?}", x, y),
        }
    }
}

impl PartialOrd<Integer> for Integer {
    fn partial_cmp(&self, other: &Integer) -> Option<std::cmp::Ordering> {
        match (self, other) {
            (Integer::U8(left), Integer::U8(right)) => left.partial_cmp(right),
            (Integer::U16(left), Integer::U16(right)) => left.partial_cmp(right),
            (Integer::U32(left), Integer::U32(right)) => left.partial_cmp(right),
            (Integer::U64(left), Integer::U64(right)) => left.partial_cmp(right),
            (Integer::I8(left), Integer::I8(right)) => left.partial_cmp(right),
            (Integer::I16(left), Integer::I16(right)) => left.partial_cmp(right),
            (Integer::I32(left), Integer::I32(right)) => left.partial_cmp(right),
            (Integer::I64(left), Integer::I64(right)) => left.partial_cmp(right),
            (Integer::Natural(left), Integer::Natural(right)) => left.partial_cmp(right),
            (Integer::Integer(left), Integer::Integer(right)) => left.partial_cmp(right),
            (x, y) => panic!("Cannot compare {:?} and {:?}", x, y),
        }
    }
}


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


