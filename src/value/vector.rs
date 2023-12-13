use crate::value::integer::Natural;

pub enum Vector {
    U8(*mut u8, usize),
    U16(*mut u16, usize),
    U32(*mut u32, usize),
    U64(*mut u64, usize),
    I8(*mut i8, usize),
    I16(*mut i16, usize),
    I32(*mut i32, usize),
    I64(*mut i64, usize),
    F32(*mut f32, usize),
    F64(*mut f64, usize),
    Natural(*mut Natural, usize),
    Reference(*mut u64, usize),
}