use std::panic;

use crate::Instructions::*;
use crate::Operations::*;

pub const ADDRESS_COUNT: usize = 256;
pub const REGISTER_COUNT: usize = 16;

pub struct Machine {
    program: usize,
    registers: [u8; REGISTER_COUNT],
    memory: [u8; ADDRESS_COUNT],
}

impl Machine {
    /// Create a machine with program counter.
    pub fn New(pc: u8) -> Self {
        Self {
            program: pc as usize,
            registers: [0; REGISTER_COUNT],
            memory: [0; ADDRESS_COUNT],
        }
    }

    pub fn LoadProgram(&mut self, buffer: &[u8], offset: u8) {
        let mut i = offset as usize;

        for elem in buffer.to_vec().drain(..) {
            self.memory[i] = elem;
            i += 1;
        }
    }

    pub fn Execute(mut self) {
        let mut running = true;

        while running {
            //self.PrintMemory();
            self.PrintRegisters();

            let opcode = self.memory[self.program];

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

    #[allow(dead_code)]
    fn PrintRegisters(&self) {
        for data in self.registers.iter() {
            print!("{:02x} ", data);
        }

        println!();
    }

    #[allow(dead_code)]
    fn PrintMemory(&self) {
        let mut address = 0;

        for data in self.memory.iter() {
            if address % 8 == 0 {
                println!();
            }

            print!("{:02x} ", data);

            address += 1;
        }

        println!();
    }

    pub fn GetMemory(&self, addr: u8) -> u8 {
        let index = addr as usize;

        if index < ADDRESS_COUNT {
            self.memory[index]
        } else {
            panic!("Invalid Address!")
        }
    }

    pub fn SetMemory(&mut self, addr: u8, val: u8) {
        let index = addr as usize;

        if index < ADDRESS_COUNT {
            self.memory[index] = val;
        }
    }

    pub fn GetRegister(&self, reg: u8) -> u8 {
        let index = reg as usize;

        if index < REGISTER_COUNT {
            self.registers[index]
        } else {
            panic!("Invalid Register!")
        }
    }

    pub fn SetRegister(&mut self, reg: u8, val: u8) {
        let index = reg as usize;

        if index < REGISTER_COUNT {
            self.registers[index] = val;
        }
    }

    pub fn GetAddress(&self) -> u8 {
        self.program as u8
    }

    pub fn WalkAddress(&mut self, bytes: i8) {
        self.program += bytes as usize;
    }

    pub fn SetAddress(&mut self, addr: u8) {
        self.program = addr as usize;
    }
}
