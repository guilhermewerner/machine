#![allow(dead_code)]
#![allow(non_snake_case)]

mod Instructions;

pub mod Mnemonics;
pub mod Operations;
pub mod Payload;
pub mod Types;

#[path = "Assembly.rs"]
mod _Assembly;
pub use self::_Assembly::*;

#[path = "Frame.rs"]
mod _Frame;
pub use self::_Frame::*;

#[path = "Heap.rs"]
mod _Heap;
pub use self::_Heap::*;

#[path = "Limits.rs"]
mod _Limits;
pub use self::_Limits::*;

#[path = "Machine.rs"]
mod _Machine;
pub use self::_Machine::*;

#[path = "Registry.rs"]
mod _Registry;
pub use self::_Registry::*;

#[path = "Stack.rs"]
mod _Stack;
pub use self::_Stack::*;

#[path = "Instruction.rs"]
mod _Instruction;
pub use self::_Instruction::*;

#[path = "InstructionTable.rs"]
mod _InstructionTable;
pub use self::_InstructionTable::*;
