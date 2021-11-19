use crate::Types::*;
use std::mem;

pub struct Stack {
    storage: Vec<u8>,
    capacity: usize,
}

impl Stack {
    pub fn New(size: usize) -> Self {
        Self {
            storage: Vec::with_capacity(size),
            capacity: size,
        }
    }

    pub fn Flush(&mut self) {
        self.storage.clear();
    }

    pub fn PushByte(&mut self, item: Byte) {
        if self.storage.len() + 1 < self.capacity {
            self.storage.push(item);
        }
    }

    pub fn PopByte(&mut self) -> Byte {
        self.storage.pop().unwrap_or(0)
    }

    pub fn PushWord(&mut self, item: Word) {
        const BYTES: usize = mem::size_of::<Word>();

        if self.storage.len() + BYTES < self.capacity {
            self.storage.extend(item);
        }
    }

    pub fn PopWord(&mut self) -> Word {
        const BYTES: usize = mem::size_of::<Word>();
        let mut buffer = [0; BYTES];

        for i in 0..BYTES {
            buffer[i] = self.storage.pop().unwrap_or(0);
        }

        buffer
    }
}

