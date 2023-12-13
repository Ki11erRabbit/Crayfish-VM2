use std::collections::HashMap;
use crate::value::Value;

pub struct SumType {
    name: Box<str>,
    tag: u8,
    fields: HashMap<Box<str>, Value>
}