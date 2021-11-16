#![allow(dead_code)]
#![allow(non_snake_case)]
#![allow(unused_imports)]
#![allow(unused_variables)]

pub mod Instructions;
pub mod Operations;
pub mod Registers;

use Instructions::*;
use Operations::*;
use Registers::*;

//#[path = "World.rs"]
//mod _World;
//pub use self::_World::*;

pub const ADDRESS_COUNT: usize = 256;
pub const REGISTER_COUNT: usize = 16;

pub struct Machine {
    pub program: usize,
    pub registers: [u8; REGISTER_COUNT],
    pub memory: [u8; ADDRESS_COUNT],
}

impl Machine {
    /// Create a machine with program counter.
    pub fn New(pc: u16) -> Self {
        Self {
            program: pc as usize,
            registers: [0; REGISTER_COUNT],
            memory: [0; ADDRESS_COUNT],
        }
    }

    pub fn LoadProgram(&mut self, buffer: &[u8], offset: u16) {
        let mut i = offset as usize;

        for elem in buffer.to_vec().drain(..) {
            self.memory[i] = elem;
            i += 1;
        }
    }

    #[allow(unused_mut)]
    pub fn Execute(mut self) {
        let mut running = true;

        while running {
            self.PrintMemory();
            //self.PrintRegisters();

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
                NOT => Xor(&mut self),
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

    fn PrintRegisters(&self) {
        for data in self.registers.iter() {
            print!("{:02x} ", data);
        }

        println!();
    }

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

    fn GetRegister(&self, register: usize) -> u8 {
        self.registers[register]
    }

    fn GetProgramCounter(&self) -> u16 {
        self.program as u16
    }

    fn SetProgramCounter(&mut self, pc: u16) {
        self.program = pc as usize;
    }
}
