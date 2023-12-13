
pub struct Natural {
    bytes: Vec<u8>,
}

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
    Integer(bool, Natural),
}


