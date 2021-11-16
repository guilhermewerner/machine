#![allow(non_snake_case)]

#[repr(u8)]
#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash, TryFromPrimitive)]
pub enum Instruction {
    /// No operation.
    ///
    /// ```no_run
    /// nop
    /// ```
    Nop = 0x00,

    /// Load register from memory.
    ///
    /// ```no_run
    /// ldr r1 0050 // r1 = mem[0050]
    /// ```
    Ldr = 0x01,

    /// Store register in memory.
    ///
    /// ```no_run
    /// str 0050 r1 // mem[0050] = r1
    /// ```
    Str = 0x02,

    /// Move register.
    ///
    /// ```no_run
    /// mov r1 r2 // r1 = r2
    /// ```
    Mov = 0x03,

    /// Arithmetic addition `+`.
    ///
    /// ```no_run
    /// add r1 r2 r3 // r1 = r2 + r3
    /// ```
    Add = 0x04,

    /// Add with carry.
    ///
    /// ```no_run
    /// adc r1 r2 r3 r4 // r1 = r2 + r3 + r4
    /// ```
    Adc,

    /// Arithmetic subtraction `-`.
    ///
    /// ```no_run
    /// sub r1 r2 r3 // r1 = r2 - r3
    /// ```
    Sub,

    /// Subtract with carry.
    ///
    /// ```no_run
    /// sbc r1 r2 r3 r4 // r1 = r2 - r3 - 1 - r4
    /// ```
    Sbc,

    /// Arithmetic multiplication `*`.
    ///
    /// ```no_run
    /// mul r1 r2 r3 // r1 = r2 * r3
    /// ```
    Mul,

    /// Arithmetic division `/`.
    ///
    /// ```no_run
    /// div r1 r2 r3 // r1 = r2 / r3
    /// ```
    Div,

    /// Arithmetic remainder `/`.
    ///
    /// ```no_run
    /// rem r1 r2 r3 // r1 = r2 % r3
    /// ```
    Rem,

    /// Arithmetic negation `-`.
    ///
    /// ```no_run
    /// not r1 r2 // r1 = -r2
    /// ```
    Neg,

    /// Bitwise AND `&`.
    ///
    /// ```no_run
    /// and r1 r2 r3 // r1 = r2 & r3
    /// ```
    And,

    /// Bitwise OR `|`.
    ///
    /// ```no_run
    /// or r1 r2 r3 // r1 = r2 | r3
    /// ```
    Or,

    /// Bitwise XOR `^`.
    ///
    /// ```no_run
    /// xor r1 r2 r3 // r1 = r2 ^ r3
    /// ```
    Xor,

    /// Logical negation `!`
    ///
    /// ```no_run
    /// not r1 r2 // r1 = !r2
    /// ```
    Not,

    /// Bitwise NAND.
    ///
    /// ```no_run
    /// nand r1 r2 r3 // r1 = !(r2 & r3)
    /// ```
    Nand,

    /// Bitwise NOR.
    ///
    /// ```no_run
    /// nor r1 r2 r3 // r1 = !(r2 | r3)
    /// ```
    Nor,

    /// Bitwise XNOR.
    ///
    /// ```no_run
    /// xnor r1 r2 r3 // r1 = !(r2 ^ r3)
    /// ```
    Xnor,

    /// Shift left `<<`.
    ///
    /// ```no_run
    /// shl r1 r2 r3 // r1 = r2 << r3
    /// ```
    Shl,

    /// Shift right `>>`.
    ///
    /// ```no_run
    /// shr r1 r2 r3 // r1 = r2 >> r3
    /// ```
    Shr,

    /// Equality comparation.
    ///
    /// ```no_run
    /// eq r1 r2 r3 // r1 = r2 == r3
    /// ```
    Eq,

    /// Inequality comparation.
    ///
    /// ```no_run
    /// neq r1 r2 r3 // r1 = r2 != r3
    /// ```
    Neq,

    /// Less comparation.
    ///
    /// ```no_run
    /// lt r1 r2 r3 // r1 = r2 < r3
    /// ```
    Lt,

    /// Less-equal comparation.
    ///
    /// ```no_run
    /// le r1 r2 r3 // r1 = r2 <= r3
    /// ```
    Le,

    /// Greater comparation.
    ///
    /// ```no_run
    /// gt r1 r2 r3 // r1 = r2 > r3
    /// ```
    Gt,

    /// Greater-equal comparation.
    ///
    /// ```no_run
    /// ge r1 r2 r3 // r1 = r2 >= r3
    /// ```
    Ge,

    /// Halt execution.
    ///
    /// ```no_run
    /// hlt
    /// ```
    Hlt = 0xFF,
}
