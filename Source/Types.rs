pub type Byte = u8;
pub type Half = [Byte; 2];
pub type Word = [Byte; 4];
pub type DWord = [Byte; 8];
pub type QWord = [Byte; 16];

pub enum PrimitiveType {
    Byte,
    Bool,
    Char,
    Int8,
    Int16,
    Int32,
    Int64,
    Int128,
    UInt8,
    UInt16,
    UInt32,
    UInt64,
    UInt128,
    Float32,
    Float64,
    String,
}
