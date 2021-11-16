#![allow(non_snake_case)]

#[repr(u8)]
#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash, TryFromPrimitive)]
pub enum Instruction {
    /// **0x00** - No operation.
    ///
    /// ```no_run
    /// nop
    /// ```
    Nop = 0x00,

    /// **0x01** - Load register from memory.
    ///
    /// ```no_run
    /// ldr r1 0050 // r1 = mem[0050]
    /// ```
    Ldr = 0x01,

    /// **0x02** - Store register in memory.
    ///
    /// ```no_run
    /// str 0050 r1 // mem[0050] = r1
    /// ```
    Str = 0x02,

    /// **0x03** - Move register.
    ///
    /// ```no_run
    /// mov r1 r2 // r1 = r2
    /// ```
    Mov = 0x03,

    /// **0x04** - Arithmetic addition `+`.
    ///
    /// ```no_run
    /// add r1 r2 r3 // r1 = r2 + r3
    /// ```
    Add = 0x04,

    /// **0x05** - Add with carry.
    ///
    /// ```no_run
    /// adc r1 r2 r3 r4 // r1 = r2 + r3 + r4
    /// ```
    Adc = 0x05,

    /// **0x06** - Arithmetic subtraction `-`.
    ///
    /// ```no_run
    /// sub r1 r2 r3 // r1 = r2 - r3
    /// ```
    Sub = 0x06,

    /// **0x07** - Subtract with carry.
    ///
    /// ```no_run
    /// sbc r1 r2 r3 r4 // r1 = r2 - r3 - 1 - r4
    /// ```
    Sbc = 0x07,

    /// **0x08** - Arithmetic multiplication `*`.
    ///
    /// ```no_run
    /// mul r1 r2 r3 // r1 = r2 * r3
    /// ```
    Mul = 0x08,

    /// **0x09** - Arithmetic division `/`.
    ///
    /// ```no_run
    /// div r1 r2 r3 // r1 = r2 / r3
    /// ```
    Div = 0x09,

    /// **0x0A** - Arithmetic remainder `/`.
    ///
    /// ```no_run
    /// rem r1 r2 r3 // r1 = r2 % r3
    /// ```
    Rem = 0x0A,

    /// **0x0B** - Arithmetic negation `-`.
    ///
    /// ```no_run
    /// not r1 r2 // r1 = -r2
    /// ```
    Neg = 0x0B,

    /// **0x0C** - Bitwise AND `&`.
    ///
    /// ```no_run
    /// and r1 r2 r3 // r1 = r2 & r3
    /// ```
    And = 0x0C,

    /// **0x0D** - Bitwise OR `|`.
    ///
    /// ```no_run
    /// or r1 r2 r3 // r1 = r2 | r3
    /// ```
    Or = 0x0D,

    /// **0x0E** - Bitwise XOR `^`.
    ///
    /// ```no_run
    /// xor r1 r2 r3 // r1 = r2 ^ r3
    /// ```
    Xor = 0x0E,

    /// **0x0F** - Logical negation `!`
    ///
    /// ```no_run
    /// not r1 r2 // r1 = !r2
    /// ```
    Not = 0x0F,

    /// **0x10** - Bitwise NAND.
    ///
    /// ```no_run
    /// nand r1 r2 r3 // r1 = !(r2 & r3)
    /// ```
    Nand = 0x10,

    /// **0x11** - Bitwise NOR.
    ///
    /// ```no_run
    /// nor r1 r2 r3 // r1 = !(r2 | r3)
    /// ```
    Nor = 0x11,

    /// **0x12** - Bitwise XNOR.
    ///
    /// ```no_run
    /// xnor r1 r2 r3 // r1 = !(r2 ^ r3)
    /// ```
    Xnor = 0x12,

    /// **0x13** - Shift left `<<`.
    ///
    /// ```no_run
    /// shl r1 r2 r3 // r1 = r2 << r3
    /// ```
    Shl = 0x13,

    /// **0x14** - Shift right `>>`.
    ///
    /// ```no_run
    /// shr r1 r2 r3 // r1 = r2 >> r3
    /// ```
    Shr = 0x14,

    /// **0x15** - Equality comparation.
    ///
    /// ```no_run
    /// eq r1 r2 r3 // r1 = r2 == r3
    /// ```
    Eq = 0x15,

    /// **0x16** - Inequality comparation.
    ///
    /// ```no_run
    /// neq r1 r2 r3 // r1 = r2 != r3
    /// ```
    Neq = 0x16,

    /// **0x17** - Less comparation.
    ///
    /// ```no_run
    /// lt r1 r2 r3 // r1 = r2 < r3
    /// ```
    Lt = 0x17,

    /// **0x18** - Less-equal comparation.
    ///
    /// ```no_run
    /// le r1 r2 r3 // r1 = r2 <= r3
    /// ```
    Le = 0x18,

    /// **0x19** - Greater comparation.
    ///
    /// ```no_run
    /// gt r1 r2 r3 // r1 = r2 > r3
    /// ```
    Gt = 0x19,

    /// **0x1A** - Greater-equal comparation.
    ///
    /// ```no_run
    /// ge r1 r2 r3 // r1 = r2 >= r3
    /// ```
    Ge = 0x1A,

    /// **0xFF** - Halt execution.
    ///
    /// ```no_run
    /// hlt
    /// ```
    Hlt = 0xFF,
}
