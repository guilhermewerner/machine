use crate::Frame;

pub struct Stack {
    frames: Vec<Frame>,
    lenght: usize,
}

impl Stack {
    pub fn New(size: usize) -> Self {
        Self {
            frames: Vec::with_capacity(size),
            lenght: size,
        }
    }

    pub fn IsEmpty(&self) -> bool {
        self.frames.is_empty()
    }

    pub fn Flush(&mut self) {
        self.frames.clear();
    }

    pub fn Push(&mut self, frame: Frame) {
        if self.frames.len() + frame.GetSize() < self.lenght {
            self.frames.push(frame);
        }
    }

    pub fn Pop(&mut self) -> Frame {
        self.frames.pop().unwrap_or(Frame::Null())
    }
}
