use crate::value::float::Float;
use crate::value::integer::Integer;
use crate::value::Value;

pub struct Instruction {
    pub row: usize,
    pub column: usize,
    pub instruction: RealInstruction,
}


pub enum FunctionSource {
    Name(Box<str>),
    Address(u64),
    Stack,
}

/// Represents a jump target for a jump instruction.
/// This can be a relative jump, an absolute jump or a label.
#[derive(Debug, Clone)]
pub enum JumpTarget {
    /// A relative jump target.
    /// The i32 represents the offset from the current instruction.
    /// A positive value means forward, a negative value means backward.
    /// 0 means the next instruction.
    Relative(isize),
    /// An absolute jump target.
    /// The usize represents the address of the instruction.
    /// This is the address of the instruction, not the address of the memory.
    Absolute(usize),
    /// A label jump target.
    /// The String represents the name of the label.
    /// This is the name of the label, not the address of the memory.
    /// The address of the label is resolved at runtime.
    Label(Box<str>),
}

/// Represents a condition for a jump instruction.
/// This can be a condition or no condition.
/// If the condition is not met, the jump instruction is ignored.
/// If the condition is met, the jump instruction is executed.
#[derive(Debug, Clone)]
pub enum Condition {
    /// No condition.
    Always,
    /// When the last instruction resulted in an equal result.
    Equal,
    /// When the last instruction resulted in an unequal result.
    NotEqual,
    /// When the last instruction resulted in a greater than result.
    GreaterThan,
    /// When the last instruction resulted in a greater than or equal result.
    GreaterThanOrEqual,
    /// When the last instruction resulted in a less than result.
    LessThan,
    /// When the last instruction resulted in a less than or equal result.
    LessThanOrEqual,
    /// When the last instruction resulted in a zero result.
    Zero,
    /// When the last instruction resulted in a non-zero result.
    NotZero,
    /// When the last instruction resulted in a carry result.
    Carry,
    /// When the last instruction resulted in a non-carry result.
    NotCarry,
    /// When the last instruction resulted in a negative result.
    Negative,
    /// When the last instruction resulted in a non-negative result.
    NotNegative,
    /// True if the current stack frame is a continuation.
    InContinuation,
    /// True if the current stack frame is not a continuation.
    NotInContinuation,
}

#[derive(Debug, Clone)]
pub enum ComparisonType {
    Equal,
    NotEqual,
    GreaterThan,
    GreaterThanOrEqual,
    LessThan,
    LessThanOrEqual,
}

#[derive(Debug,Clone)]
pub enum RealInstruction {
    Halt,
    NoOp,
    // Stack
    Push(Value),
    Pop,
    // Tuple
    TupleNew(usize),
    TupleGet(usize),
    // Vector
    VectorNew(usize),
    VectorGet(usize),
    VectorSet(usize),
    // Product
    ProductNew(usize),
    ProductGet(Box<str>),
    ProductSet(usize),
    // Sum
    SumNew(Box<str>),
    SumGet(Box<str>),
    SumSet(Box<str>),
    // Function
    FunctionCall(FunctionSource),
    Return,
    // Reference
    ReferenceNew,
    ReferenceGet,
    ReferenceSet,
    // Integer
    IntegerNew(Integer),
    IntegerAdd,
    IntegerSubtract,
    IntegerMultiply,
    IntegerDivide,
    IntegerModulo,
    IntegerPower,
    IntegerNegate,
    IntegerBitwiseAnd,
    IntegerBitwiseOr,
    IntegerBitwiseXor,
    IntegerBitwiseNot,
    IntegerShiftLeft,
    IntegerShiftRight,
    // Float
    FloatNew(Float),
    FloatAdd,
    FloatSubtract,
    FloatMultiply,
    FloatDivide,
    FloatModulo,
    FloatPower,
    FloatNegate,
    // String
    StringNew(Box<str>),
    StringConcat,
    StringEqual,
    StringNotEqual,
    StringLessThan,
    StringLessThanOrEqual,
    StringGreaterThan,
    StringGreaterThanOrEqual,
    // Boolean
    BooleanNew(bool),
    BooleanAnd,
    BooleanOr,
    BooleanNot,
    // Character
    CharacterNew(char),
    // Server
    RequestValue(Box<str>),
    SetValue(Box<str>),
    // Control
    Compare(ComparisonType),
    Goto(JumpTarget,Condition),
    // Store
    Store(Box<str>),
    Lookup(Box<str>),
    GlobalStore(Box<str>),
    GlobalLookup(Box<str>),
    // IO
    Write,
    Read,








}