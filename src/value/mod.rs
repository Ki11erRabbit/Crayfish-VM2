use std::fmt::Display;
use crate::value::decimal::Decimal;
use crate::value::function::Function;
use crate::value::integer::Integer;
use crate::value::product::ProductType;
use crate::value::sum::SumType;
use crate::value::tuple::Tuple;
use crate::value::vector::Vector;

pub mod integer;
pub mod decimal;
pub mod vector;
pub mod product;
pub mod sum;
pub mod tuple;
pub mod function;


pub type Reference = u64;

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