use crate::Instruction;
use crate::Instructions::*;
use crate::Operations::*;
use crate::Payload::PayloadType;
use crate::Types::Byte;
use std::collections::HashMap;

pub struct InstructionTable {
    table: HashMap<Byte, Instruction>,
}

impl Default for InstructionTable {
    fn default() -> Self {
        Self {
            table: HashMap::new(),
        }
    }
}

impl InstructionTable {
    #[inline]
    pub fn New() -> Self {
        let mut table = InstructionTable::default();

        table.Insert(Instruction::New(NOP, "nop", PayloadType::Nothing, Nothing));
        table.Insert(Instruction::New(HLT, "hlt", PayloadType::Nothing, Halt));

        table
    }

    #[inline]
    pub fn IsEmpty(&self) -> bool {
        self.table.is_empty()
    }

    #[inline]
    pub fn Get(&self, code: Byte) -> Option<&Instruction> {
        self.table.get(&code)
    }

    #[inline]
    pub fn Insert(&mut self, instr: Instruction) {
        self.table.insert(instr.code, instr);
    }
}
