use crate::Types::*;
use std::mem;

pub struct Heap {
    buffer: Vec<Byte>,
    lenght: usize,
}

impl Heap {
    pub fn New(size: usize) -> Self {
        Self {
            buffer: Vec::with_capacity(size),
            lenght: size,
        }
    }

    pub fn IsEmpty(&self) -> bool {
        self.buffer.is_empty()
    }

    pub fn Flush(&mut self) {
        self.buffer.clear();
    }

    pub fn ReadByte(&mut self, addr: u32) -> Byte {
        let index = addr as usize;
        *self.buffer.get(index).unwrap()
    }

    pub fn WriteByte(&mut self, addr: u32, value: Byte) {
        let index = addr as usize;

        if index < self.lenght {
            self.buffer.push(value);
        }
    }

    pub fn ReadHalf(&mut self, addr: u32) -> Half {
        let mut index = addr as usize;

        const BYTES: usize = mem::size_of::<Half>();
        let mut buffer = [0; BYTES];

        for i in 0..BYTES {
            buffer[i] = *self.buffer.get(index).unwrap_or(&0);
            index += 1;
        }

        buffer
    }

    pub fn WriteHalf(&mut self, addr: u32, value: Half) {
        let index = addr as usize;

        if index < self.lenght {
            self.buffer.extend(value);
        }
    }

    pub fn ReadWord(&mut self, addr: u32) -> Word {
        let mut index = addr as usize;

        const BYTES: usize = mem::size_of::<Word>();
        let mut buffer = [0; BYTES];

        for i in 0..BYTES {
            buffer[i] = *self.buffer.get(index).unwrap_or(&0);
            index += 1;
        }

        buffer
    }

    pub fn WriteWord(&mut self, addr: u32, value: Word) {
        let index = addr as usize;

        if index < self.lenght {
            self.buffer.extend(value);
        }
    }
}
