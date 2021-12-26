use crate::Types::Byte;
use crate::{Machine, PayloadType};

pub struct Instruction {
    pub code: Byte,
    pub name: String,
    pub payload: PayloadType,
    pub function: InstructionFunction,
}

pub type InstructionFunction = fn(machine: &mut Machine) -> bool;

impl Instruction {
    #[inline]
    pub fn New(
        code: Byte,
        name: &str,
        payload: PayloadType,
        function: InstructionFunction,
    ) -> Self {
        Self {
            code,
            name: name.to_string(),
            payload,
            function,
        }
    }
}
