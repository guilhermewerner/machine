use crate::{Byte, DWord, Half, QWord, Word};
use std::mem;

pub struct Heap {
    inner: Vec<Byte>,
    lenght: usize,
}

impl Heap {
    pub fn New(size: usize) -> Self {
        Self {
            inner: Vec::with_capacity(size),
            lenght: size,
        }
    }

    pub fn IsEmpty(&self) -> bool {
        self.inner.is_empty()
    }

    pub fn Flush(&mut self) {
        self.inner.clear();
    }

    pub fn ReadByte(&mut self, addr: u32) -> Byte {
        let index = addr as usize;
        *self.inner.get(index).unwrap()
    }

    pub fn WriteByte(&mut self, addr: u32, value: Byte) {
        let index = addr as usize;

        if index < self.lenght {
            self.inner.push(value);
        }
    }

    pub fn ReadHalf(&mut self, addr: u32) -> Half {
        let mut index = addr as usize;

        const BYTES: usize = mem::size_of::<Half>();
        let mut buffer = [0; BYTES];

        for i in 0..BYTES {
            buffer[i] = *self.inner.get(index).unwrap_or(&0);
            index += 1;
        }

        buffer
    }

    pub fn WriteHalf(&mut self, addr: u32, value: Half) {
        let index = addr as usize;

        if index < self.lenght {
            self.inner.extend(value);
        }
    }

    pub fn ReadWord(&mut self, addr: u32) -> Word {
        let mut index = addr as usize;

        const BYTES: usize = mem::size_of::<Word>();
        let mut buffer = [0; BYTES];

        for i in 0..BYTES {
            buffer[i] = *self.inner.get(index).unwrap_or(&0);
            index += 1;
        }

        buffer
    }

    pub fn WriteWord(&mut self, addr: u32, value: Word) {
        let index = addr as usize;

        if index < self.lenght {
            self.inner.extend(value);
        }
    }

    pub fn ReadDWord(&mut self, addr: u32) -> DWord {
        let mut index = addr as usize;

        const BYTES: usize = mem::size_of::<DWord>();
        let mut buffer = [0; BYTES];

        for i in 0..BYTES {
            buffer[i] = *self.inner.get(index).unwrap_or(&0);
            index += 1;
        }

        buffer
    }

    pub fn WriteDWord(&mut self, addr: u32, value: DWord) {
        let index = addr as usize;

        if index < self.lenght {
            self.inner.extend(value);
        }
    }

    pub fn ReadQWord(&mut self, addr: u32) -> QWord {
        let mut index = addr as usize;

        const BYTES: usize = mem::size_of::<QWord>();
        let mut buffer = [0; BYTES];

        for i in 0..BYTES {
            buffer[i] = *self.inner.get(index).unwrap_or(&0);
            index += 1;
        }

        buffer
    }

    pub fn WriteQWord(&mut self, addr: u32, value: QWord) {
        let index = addr as usize;

        if index < self.lenght {
            self.inner.extend(value);
        }
    }
}
