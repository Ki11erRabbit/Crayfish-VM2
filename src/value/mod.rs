use crate::value::float::Float;
use crate::value::function::Function;
use crate::value::integer::Integer;
use crate::value::product::ProductType;
use crate::value::sum::SumType;
use crate::value::tuple::Tuple;
use crate::value::vector::Vector;

pub mod integer;
pub mod float;
pub mod vector;
pub mod product;
pub mod sum;
pub mod tuple;
pub mod function;

pub enum Value {
    String(String),
    Integer(Integer),
    Float(Float),
    Vector(Vector),
    Product(ProductType),
    Sum(SumType),
    Function(Function),
    Reference(u64),
    Tuple(Tuple),
    Character(char),
    Boolean(bool),
}