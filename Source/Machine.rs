use crate::Instructions::*;
use crate::Operations::*;
use crate::{Byte, Half, Heap, Registry, Stack, Word, HEAP_LIMIT, STACK_SIZE};

#[allow(dead_code)]
pub struct Machine {
    /// Program Counter.
    pub(crate) pc: usize,

    /// General-purpose registers.
    pub(crate) registry: Registry,

    /// Stack Pointer.
    pub(crate) sp: usize,

    /// Stack Memory.
    pub(crate) stack: Stack,

    /// Memory Pointer.
    pub(crate) mp: usize,

    /// Heap Memory.
    pub(crate) heap: Heap,
}

#[allow(dead_code)]
impl Machine {
    pub fn New(address: Word) -> Self {
        Self {
            pc: u32::from_be_bytes(address) as usize,
            registry: Registry::default(),
            sp: 0,
            stack: Stack::New(STACK_SIZE),
            mp: 0,
            heap: Heap::New(HEAP_LIMIT),
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

            let opcode = self.heap.ReadByte(self.pc as u32);
            self.mp = self.pc + 1;

            running = match opcode {
                LDA_W => LoadRegister(&mut self),
                STA_W => StoreRegister(&mut self),
                INC => IncrementRegister(&mut self),
                DEC => DecrementRegister(&mut self),
                NEG => Neg(&mut self),
                AND => And(&mut self),
                OR => Or(&mut self),
                XOR => Xor(&mut self),
                NOT => Not(&mut self),
                NAND => Nand(&mut self),
                NOR => Nor(&mut self),
                XNOR => Xnor(&mut self),
                SHL => ShiftLeft(&mut self),
                SHR => ShiftRight(&mut self),
                EQ => Equals(&mut self),
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
        let mut mp = self.mp as u32;

        if let Some(addr) = addr {
            mp = u32::from_be_bytes(addr);
        }

        self.mp = (mp + 1) as usize;
        self.heap.ReadByte(mp)
    }

    pub(crate) fn WriteByte(&mut self, addr: Option<Word>, value: Byte) {
        let mut mp = self.mp as u32;

        if let Some(addr) = addr {
            mp = u32::from_be_bytes(addr);
        };

        self.mp = (mp + 1) as usize;
        self.heap.WriteByte(mp, value);
    }

    pub(crate) fn ReadHalf(&mut self, addr: Option<Word>) -> Half {
        let mut mp = self.mp as u32;

        if let Some(addr) = addr {
            mp = u32::from_be_bytes(addr);
        }

        self.mp = (mp + 4) as usize;
        self.heap.ReadHalf(mp as u32)
    }

    pub(crate) fn WriteHalf(&mut self, addr: Option<Word>, value: Half) {
        let mut mp = self.mp as u32;

        if let Some(addr) = addr {
            mp = u32::from_be_bytes(addr);
        }

        self.mp = (mp + 4) as usize;
        self.heap.WriteHalf(mp as u32, value);
    }

    pub(crate) fn ReadWord(&mut self, addr: Option<Word>) -> Word {
        let mut mp = self.mp as u32;

        if let Some(addr) = addr {
            mp = u32::from_be_bytes(addr);
        }

        self.mp = (mp + 4) as usize;
        self.heap.ReadWord(mp as u32)
    }

    pub(crate) fn WriteWord(&mut self, addr: Option<Word>, value: Word) {
        let mut mp = self.mp as u32;

        if let Some(addr) = addr {
            mp = u32::from_be_bytes(addr);
        }

        self.mp = (mp + 4) as usize;
        self.heap.WriteWord(mp as u32, value);
    }

    pub(crate) fn Next(&mut self) {
        self.pc += 1;
    }

    pub(crate) fn Walk(&mut self, bytes: Byte) {
        self.pc += bytes as usize;
    }
}
