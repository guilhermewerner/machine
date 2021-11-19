#![allow(non_snake_case)]

mod Instructions;

pub mod Operations;
pub mod Types;

#[path = "Limits.rs"]
mod _Limits;
pub use self::_Limits::*;

#[path = "Machine.rs"]
mod _Machine;
pub use self::_Machine::*;

#[path = "Memory.rs"]
mod _Memory;
pub use self::_Memory::*;

#[path = "Registry.rs"]
mod _Registry;
pub use self::_Registry::*;

#[path = "Stack.rs"]
mod _Stack;
pub use self::_Stack::*;
