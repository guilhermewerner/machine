use crate::Types::*;
use crate::REGISTER_COUNT;

#[derive(Default)]
pub struct Registry {
    registers: [Word; REGISTER_COUNT],
}

impl Registry {
    pub fn Get(&mut self, index: u8) -> Word {
        let index = index as usize;

        if index < REGISTER_COUNT {
            self.registers[index]
        } else {
            panic!("Invalid Register!")
        }
    }

    pub fn Set(&mut self, index: u8, value: Word) {
        let index = index as usize;

        if index < REGISTER_COUNT {
            self.registers[index] = value;
        }
    }

    pub(crate) fn Print(&self) {
        for r in self.registers.iter() {
            print!("{:02X}{:02X}{:02X}{:02X} ", r[0], r[1], r[2], r[3]);
        }

        println!();
    }
}
