use std::collections::HashMap;
use crate::value::Value;

pub struct ProductType {
    pub name: Box<str>,
    pub fields: HashMap<Box<str>, Value>
}