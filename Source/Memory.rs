use crate::Types::*;
use std::mem;

pub struct Memory {
    storage: Vec<u8>,
    capacity: usize,
}

impl Memory {
    pub fn New(limit: usize) -> Self {
        Self {
            storage: Vec::with_capacity(limit),
            capacity: limit,
        }
    }

    pub fn Flush(&mut self) {
        self.storage.clear();
    }

    pub fn ReadByte(&mut self, addr: u32) -> Byte {
        let index = addr as usize;
        *self.storage.get(index).unwrap()
    }

    pub fn WriteByte(&mut self, addr: u32, value: Byte) {
        let index = addr as usize;

        if index < self.capacity {
            self.storage.push(value);
        }
    }

    pub fn ReadWord(&mut self, addr: u32) -> Word {
        let mut index = addr as usize;

        const BYTES: usize = mem::size_of::<Word>();
        let mut buffer = [0; BYTES];

        for i in 0..BYTES {
            buffer[i] = *self.storage.get(index).unwrap_or(&0);
            index += 1;
        }

        buffer
    }

    pub fn WriteWord(&mut self, addr: u32, value: Word) {
        let index = addr as usize;

        if index < self.capacity {
            self.storage.extend(value);
        }
    }
}

