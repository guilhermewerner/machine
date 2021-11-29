use crate::Types::*;
use std::mem;

pub enum Frame {
    Byte(Byte),
    Half(Half),
    Word(Word),
}

impl Frame {
    pub fn Null() -> Self {
        Self::Byte(0)
    }

    pub fn GetSize(&self) -> usize {
        match self {
            Self::Byte(_) => mem::size_of::<Byte>(),
            Self::Half(_) => mem::size_of::<Half>(),
            Self::Word(_) => mem::size_of::<Word>(),
        }
    }

    pub fn IsByte(&self) -> bool {
        if let Self::Byte(_) = self {
            true
        } else {
            false
        }
    }

    pub fn GetByte(&self) -> Option<Byte> {
        if let Self::Byte(data) = *self {
            Some(data)
        } else {
            None
        }
    }

    pub fn IsHalf(&self) -> bool {
        if let Self::Half(_) = self {
            true
        } else {
            false
        }
    }

    pub fn GetHalf(&self) -> Option<Half> {
        if let Self::Half(data) = *self {
            Some(data)
        } else {
            None
        }
    }

    pub fn IsWord(&self) -> bool {
        if let Self::Word(_) = self {
            true
        } else {
            false
        }
    }

    pub fn GetWord(&self) -> Option<Word> {
        if let Self::Word(data) = *self {
            Some(data)
        } else {
            None
        }
    }
}
