use crate::Instructions::*;
use crate::Operations::*;
use crate::Types::*;
use crate::{Memory, Registry, Stack, HEAP_LIMIT, STACK_SIZE};

#[allow(dead_code)]
pub struct Machine {
    pub(crate) program_counter: usize,
    pub(crate) registry: Registry,
    pub(crate) stack_pointer: usize,
    pub(crate) link_register: usize,
    pub(crate) stack: Stack,
    pub(crate) memory_pointer: usize,
    pub(crate) heap: Memory,
}

#[allow(dead_code)]
impl Machine {
    pub fn New(address: Word) -> Self {
        Self {
            program_counter: u32::from_be_bytes(address) as usize,
            registry: Registry::default(),
            stack_pointer: 0,
            link_register: 0,
            stack: Stack::New(STACK_SIZE),
            memory_pointer: 0,
            heap: Memory::New(HEAP_LIMIT),
        }
    }

    pub fn LoadProgram(&mut self, buffer: &[u8]) {
        for (addr, data) in buffer.to_vec().drain(..).enumerate() {
            self.heap.WriteByte(addr as u32, data);
        }
    }

    pub fn Execute(mut self) {
        let mut running = true;

        while running {
            self.registry.Print();

            let opcode = self.heap.ReadByte(self.program_counter as u32);
            self.memory_pointer = self.program_counter + 1;

            running = match opcode {
                NOP => Nothing(&mut self),
                LDR => LoadRegister(&mut self),
                SVR => SaveRegister(&mut self),
                //MOV
                ADD => Add(&mut self),
                ADD_ASSIGN => AddAssign(&mut self),
                SUB => Subtract(&mut self),
                SUB_ASSIGN => SubtractAssign(&mut self),
                MUL => Multiply(&mut self),
                MUL_ASSIGN => MultiplyAssign(&mut self),
                DIV => Divide(&mut self),
                DIV_ASSIGN => DivideAssign(&mut self),
                REM => Remainder(&mut self),
                REM_ASSIGN => RemainderAssign(&mut self),
                //NEG
                AND => And(&mut self),
                AND_ASSIGN => AndAssign(&mut self),
                OR => Or(&mut self),
                OR_ASSIGN => OrAssign(&mut self),
                XOR => Xor(&mut self),
                XOR_ASSIGN => XorAssign(&mut self),
                NOT => Not(&mut self),
                SHL => ShiftLeft(&mut self),
                SHL_ASSIGN => ShiftLeftAssign(&mut self),
                SHR => ShiftRight(&mut self),
                SHR_ASSIGN => ShiftRightAssign(&mut self),
                EQ => Equals(&mut self),
                NEQ => NotEquals(&mut self),
                LT => LessThan(&mut self),
                LE => LessEquals(&mut self),
                GT => GreaterThan(&mut self),
                GE => GreaterEquals(&mut self),
                HLT => Halt(&mut self),
                _ => Nothing(&mut self),
            }
        }
    }

    pub(crate) fn ReadByte(&mut self, addr: Option<Word>) -> Byte {
        let mut mp = self.memory_pointer as u32;

        if let Some(addr) = addr {
            mp = u32::from_be_bytes(addr);
        }

        self.memory_pointer = (mp + 1) as usize;
        self.heap.ReadByte(mp)
    }

    pub(crate) fn WriteByte(&mut self, addr: Option<Word>, value: Byte) {
        let mut mp = self.memory_pointer as u32;

        if let Some(addr) = addr {
            mp = u32::from_be_bytes(addr);
        };

        self.memory_pointer = (mp + 1) as usize;
        self.heap.WriteByte(mp, value);
    }

    pub(crate) fn ReadWord(&mut self, addr: Option<Word>) -> Word {
        let mut mp = self.memory_pointer as u32;

        if let Some(addr) = addr {
            mp = u32::from_be_bytes(addr);
        }

        self.memory_pointer = (mp + 4) as usize;
        self.heap.ReadWord(mp as u32)
    }

    pub(crate) fn WriteWord(&mut self, addr: Option<Word>, value: Word) {
        let mut mp = self.memory_pointer as u32;

        if let Some(addr) = addr {
            mp = u32::from_be_bytes(addr);
        }

        self.memory_pointer = (mp + 4) as usize;
        self.heap.WriteWord(mp as u32, value);
    }

    pub(crate) fn Next(&mut self) {
        self.program_counter += 1;
    }

    pub(crate) fn Walk(&mut self, bytes: Byte) {
        self.program_counter += bytes as usize;
    }
}
