use std::fmt::{Debug, Display, Formatter};
use crate::program::StringTablePath;
use crate::value::decimal::Decimal;
use crate::value::integer::Integer;
use crate::value::Value;

#[derive(Clone)]
pub struct Instruction {
    pub row: usize,
    pub column: usize,
    pub instruction: RealInstruction,
}

impl Display for Instruction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} @ {}:{}", self.instruction, self.row, self.column)
    }
}

impl Debug for Instruction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?} @ {}:{}", self.instruction, self.row, self.column)
    }
}

#[derive(Clone)]
pub enum FunctionSource {
    Name(Box<str>),
    Address(u64),
    Stack,
}

impl Display for FunctionSource {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        use FunctionSource::*;
        match self {
            Name(name) => write!(f, "name: {}", name),
            Address(address) => write!(f, "address: {:#x}", address),
            Stack => write!(f, "stack"),
        }
    }
}

impl Debug for FunctionSource {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self)
    }
}

/// Represents a jump target for a jump instruction.
/// This can be a relative jump, an absolute jump.
#[derive(Clone)]
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
}

impl Display for JumpTarget {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        use JumpTarget::*;
        match self {
            Relative(offset) => write!(f, "relative: {}", offset),
            Absolute(address) => write!(f, "absolute: {:#x}", address),
        }
    }
}

impl Debug for JumpTarget {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self)
    }
}

/// Represents a condition for a jump instruction.
/// This can be a condition or no condition.
/// If the condition is not met, the jump instruction is ignored.
/// If the condition is met, the jump instruction is executed.
#[derive(Clone, Copy)]
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
}

impl Display for Condition {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        use Condition::*;
        match self {
            Always => write!(f, "always"),
            Equal => write!(f, "equal"),
            NotEqual => write!(f, "not_equal"),
            GreaterThan => write!(f, "greater_than"),
            GreaterThanOrEqual => write!(f, "greater_than_or_equal"),
            LessThan => write!(f, "less_than"),
            LessThanOrEqual => write!(f, "less_than_or_equal"),
            Zero => write!(f, "zero"),
            NotZero => write!(f, "not_zero"),
            Carry => write!(f, "carry"),
            NotCarry => write!(f, "not_carry"),
            Negative => write!(f, "negative"),
            NotNegative => write!(f, "not_negative"),
        }
    }
}

impl Debug for Condition {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self)
    }
}

#[derive(Clone)]
pub enum ComparisonType {
    Equal,
    NotEqual,
    GreaterThan,
    GreaterThanOrEqual,
    LessThan,
    LessThanOrEqual,
}

impl Display for ComparisonType {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        use ComparisonType::*;
        match self {
            Equal => write!(f, "equal"),
            NotEqual => write!(f, "not_equal"),
            GreaterThan => write!(f, "greater_than"),
            GreaterThanOrEqual => write!(f, "greater_than_or_equal"),
            LessThan => write!(f, "less_than"),
            LessThanOrEqual => write!(f, "less_than_or_equal"),
        }
    }
}

impl Debug for ComparisonType {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self)
    }
}

#[derive(Clone)]
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
    DecimalNew(Decimal),
    DecimalAdd,
    DecimalSubtract,
    DecimalMultiply,
    DecimalDivide,
    DecimalModulo,
    DecimalPower,
    DecimalNegate,
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
    // String Table
    GetStringRef(StringTablePath, usize),
}

impl Display for RealInstruction {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        use RealInstruction::*;
        match self {
            Halt => write!(f, "halt"),
            NoOp => write!(f, "noop"),
            Push(value) => write!(f, "push {}", value),
            Pop => write!(f, "pop"),
            TupleNew(size) => write!(f, "tuple.new {}", size),
            TupleGet(index) => write!(f, "tuple.get {}", index),
            VectorNew(size) => write!(f, "vector.new {}", size),
            VectorGet(index) => write!(f, "vector.get {}", index),
            VectorSet(index) => write!(f, "vector.set {}", index),
            ProductNew(size) => write!(f, "product.new {}", size),
            ProductGet(name) => write!(f, "product.get {}", name),
            ProductSet(index) => write!(f, "product.set {}", index),
            SumNew(name) => write!(f, "sum.new {}", name),
            SumGet(name) => write!(f, "sum.get {}", name),
            SumSet(name) => write!(f, "sum.set {}", name),
            FunctionCall(source) => write!(f, "function.call {}", source),
            Return => write!(f, "return"),
            ReferenceNew => write!(f, "reference.new"),
            ReferenceGet => write!(f, "reference.get"),
            ReferenceSet => write!(f, "reference.set"),
            IntegerNew(value) => write!(f, "integer.new {}", value),
            IntegerAdd => write!(f, "integer.add"),
            IntegerSubtract => write!(f, "integer.subtract"),
            IntegerMultiply => write!(f, "integer.multiply"),
            IntegerDivide => write!(f, "integer.divide"),
            IntegerModulo => write!(f, "integer.modulo"),
            IntegerPower => write!(f, "integer.power"),
            IntegerNegate => write!(f, "integer.negate"),
            IntegerBitwiseAnd => write!(f, "integer.bitwise_and"),
            IntegerBitwiseOr => write!(f, "integer.bitwise_or"),
            IntegerBitwiseXor => write!(f, "integer.bitwise_xor"),
            IntegerBitwiseNot => write!(f, "integer.bitwise_not"),
            IntegerShiftLeft => write!(f, "integer.shift_left"),
            IntegerShiftRight => write!(f, "integer.shift_right"),
            DecimalNew(value) => write!(f, "decimal.new {}", value),
            DecimalAdd => write!(f, "decimal.add"),
            DecimalSubtract => write!(f, "decimal.subtract"),
            DecimalMultiply => write!(f, "decimal.multiply"),
            DecimalDivide => write!(f, "decimal.divide"),
            DecimalModulo => write!(f, "decimal.modulo"),
            DecimalPower => write!(f, "decimal.power"),
            DecimalNegate => write!(f, "decimal.negate"),
            StringNew(value) => write!(f, "string.new {}", value),
            StringConcat => write!(f, "string.concat"),
            StringEqual => write!(f, "string.equal"),
            StringNotEqual => write!(f, "string.not_equal"),
            StringLessThan => write!(f, "string.less_than"),
            StringLessThanOrEqual => write!(f, "string.less_than_or_equal"),
            StringGreaterThan => write!(f, "string.greater_than"),
            StringGreaterThanOrEqual => write!(f, "string.greater_than_or_equal"),
            BooleanNew(value) => write!(f, "boolean.new {}", value),
            BooleanAnd => write!(f, "boolean.and"),
            BooleanOr => write!(f, "boolean.or"),
            BooleanNot => write!(f, "boolean.not"),
            CharacterNew(value) => write!(f, "character.new {}", value),
            RequestValue(name) => write!(f, "request_value {}", name),
            SetValue(name) => write!(f, "set_value {}", name),
            Compare(comparison) => write!(f, "compare {}", comparison),
            Goto(target, condition) => write!(f, "goto {} {}", target, condition),
            Store(name) => write!(f, "store {}", name),
            Lookup(name) => write!(f, "lookup {}", name),
            GlobalStore(name) => write!(f, "global_store {}", name),
            GlobalLookup(name) => write!(f, "global_lookup {}", name),
            Write => write!(f, "write"),
            Read => write!(f, "read"),
            GetStringRef(path, index) => write!(f, "get_string_ref {} {}", path, index),
        }
    }
}

impl Debug for RealInstruction {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self)
    }
}