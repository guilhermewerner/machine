use crate::{Byte, DWord, QWord, Half,Word};
use std::mem;

pub struct Stack {
    inner: Vec<Byte>,
    lenght: usize,
}

impl Stack {
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

    pub fn PushByte(&mut self, value: Byte) {
        if self.inner.len() + 1 < self.lenght {
            self.inner.push(value);
        }
    }

    pub fn PeekByte(&self) -> Byte {
        *self.inner.last().unwrap()
    }

    pub fn PopByte(&mut self) -> Byte {
        self.inner.pop().unwrap()
    }

    pub fn PushHalf(&mut self, value: Half) {
        if self.inner.len() + 1 < self.lenght {
            self.inner.extend(value);
        }
    }

    pub fn PeekHalf(&self) -> Half {
        const BYTES: usize = mem::size_of::<Half>();
        let mut buffer = [0; BYTES];

        for i in 0..BYTES {
            buffer[i] = *self.inner.last().unwrap_or(&0);
        }

        buffer
    }

    pub fn PopHalf(&mut self) -> Half {
        const BYTES: usize = mem::size_of::<Half>();
        let mut buffer = [0; BYTES];

        for i in 0..BYTES {
            buffer[i] = self.inner.pop().unwrap_or(0);
        }

        buffer
    }

    pub fn PushWord(&mut self, value: Word) {
        if self.inner.len() + 1 < self.lenght {
            self.inner.extend(value);
        }
    }

    pub fn PeekWord(&self) -> Word {
        const BYTES: usize = mem::size_of::<Word>();
        let mut buffer = [0; BYTES];

        for i in 0..BYTES {
            buffer[i] = *self.inner.last().unwrap_or(&0);
        }

        buffer
    }

    pub fn PopWord(&mut self) -> Word {
        const BYTES: usize = mem::size_of::<Word>();
        let mut buffer = [0; BYTES];

        for i in 0..BYTES {
            buffer[i] = self.inner.pop().unwrap_or(0);
        }

        buffer
    }

    pub fn PushDWord(&mut self, value: DWord) {
        if self.inner.len() + 1 < self.lenght {
            self.inner.extend(value);
        }
    }

    pub fn PeekDWord(&self) -> DWord {
        const BYTES: usize = mem::size_of::<DWord>();
        let mut buffer = [0; BYTES];

        for i in 0..BYTES {
            buffer[i] = *self.inner.last().unwrap_or(&0);
        }

        buffer
    }

    pub fn PopDWord(&mut self) -> DWord {
        const BYTES: usize = mem::size_of::<DWord>();
        let mut buffer = [0; BYTES];

        for i in 0..BYTES {
            buffer[i] = self.inner.pop().unwrap_or(0);
        }

        buffer
    }

    pub fn PushQWord(&mut self, value: QWord) {
        if self.inner.len() + 1 < self.lenght {
            self.inner.extend(value);
        }
    }

    pub fn PeekQWord(&self) -> QWord {
        const BYTES: usize = mem::size_of::<QWord>();
        let mut buffer = [0; BYTES];

        for i in 0..BYTES {
            buffer[i] = *self.inner.last().unwrap_or(&0);
        }

        buffer
    }

    pub fn PopQWord(&mut self) -> QWord {
        const BYTES: usize = mem::size_of::<QWord>();
        let mut buffer = [0; BYTES];

        for i in 0..BYTES {
            buffer[i] = self.inner.pop().unwrap_or(0);
        }

        buffer
    }
}
