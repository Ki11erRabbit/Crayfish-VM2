use std::fmt::{Debug, Display, LowerHex};
use crate::stack::StackChunk;
use crate::value::decimal::{Decimal, DecimalType};
use crate::value::function::Function;
use crate::value::integer::{Integer, IntegerType};
use crate::value::product::ProductType;
use crate::value::sum::SumType;
use crate::value::tuple::Tuple;
use crate::value::vector::{Vector, VectorType};

pub mod integer;
pub mod decimal;
pub mod vector;
pub mod product;
pub mod sum;
pub mod tuple;
pub mod function;

#[derive(Debug, Clone)]
pub enum ValueType {
    String,
    Integer(IntegerType),
    Decimal(DecimalType),
    Vector(VectorType),
    Product,
    Sum,
    Function,
    Reference,
    Tuple,
    Character,
    Boolean,
}

impl Display for ValueType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ValueType::String => write!(f, "String"),
            ValueType::Integer(integer) => write!(f, "Integer: {}", integer),
            ValueType::Decimal(float) => write!(f, "Decimal: {}", float),
            ValueType::Vector(vector) => write!(f, "Vector: {}", vector),
            ValueType::Product => write!(f, "Product"),
            ValueType::Sum => write!(f, "Sum"),
            ValueType::Function => write!(f, "Function"),
            ValueType::Reference => write!(f, "Reference"),
            ValueType::Tuple => write!(f, "Tuple"),
            ValueType::Character => write!(f, "Character"),
            ValueType::Boolean => write!(f, "Boolean"),
        }
    }
}

#[derive(Clone)]
pub struct Reference(pub u64);

impl Display for Reference {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "&{:#x}", self.0)
    }
}

impl Debug for Reference {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "&{:#x}", self.0)
    }
}

#[derive(Debug, Clone)]
pub enum Value {
    String(String),
    Integer(Integer),
    Decimal(Decimal),
    Vector(Vector),
    Product(ProductType),
    Sum(SumType),
    Function(Function),
    Reference(Reference),
    Tuple(Tuple),
    Character(char),
    Boolean(bool),
}

impl Value {

    pub fn cast(self, into_type: ValueType) -> Value {
        match self {
            Value::Integer(integer) => integer.cast(into_type),
            Value::Decimal(decimal) => decimal.cast(into_type),
            x => panic!("Cannot cast {:?} to {:?}", x, into_type),
        }
    }

    pub fn into_chunk(self) -> Box<dyn StackChunk> {
        match self {
            Value::String(string) => Box::new(string),
            Value::Integer(integer) => integer.into_chunk(),
            Value::Decimal(decimal) => decimal.into_chunk(),
            Value::Vector(vector) => vector.into_chunk(),
            Value::Product(product) => product.into_chunk(),
            Value::Sum(sum) => sum.into_chunk(),
            Value::Function(function) => function.into_chunk(),
            Value::Reference(reference) => Box::new(reference),
            Value::Tuple(tuple) => tuple.into_chunk(),
            Value::Character(character) => Box::new(character),
            Value::Boolean(boolean) => Box::new(boolean),
        }
    }
}


impl Display for Value {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Value::String(string) => write!(f, "{}", string),
            Value::Integer(integer) => write!(f, "{}", integer),
            Value::Decimal(float) => write!(f, "{}", float),
            Value::Vector(vector) => write!(f, "{}", vector),
            Value::Product(product) => write!(f, "{}", product),
            Value::Sum(sum) => write!(f, "{}", sum),
            Value::Function(function) => write!(f, "{}", function),
            Value::Reference(reference) => write!(f, "{}", reference),
            Value::Tuple(tuple) => write!(f, "{}", tuple),
            Value::Character(character) => write!(f, "{}", character),
            Value::Boolean(boolean) => write!(f, "{}", boolean),
        }
    }
}