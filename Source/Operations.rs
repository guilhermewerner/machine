/// **0x00** - No operation.
///
/// ```no_run
/// nop
/// ```
pub const NOP: u8 = 0x00;

/// **0x01** - Load register from memory.
///
/// ```no_run
/// ldr r1 0050 // r1 = mem[0050]
/// ```
pub const LDR: u8 = 0x01;

/// **0x02** - Save register in memory.
///
/// ```no_run
/// svr 0050 r1 // mem[0050] = r1
/// ```
pub const SVR: u8 = 0x02;

/// **0x03** - Move register.
///
/// ```no_run
/// mov r1 r2 // r1 = r2
/// ```
pub const MOV: u8 = 0x03;

/// **0x04** - The addition operator `+`.
///
/// ```no_run
/// add r1 r2 r3 // r1 = r2 + r3
/// ```
pub const ADD: u8 = 0x04;

/// **0x05** - The addition assignment operator `+=`.
///
/// ```no_run
/// add r1 r2 // r1 += r2
/// ```
pub const ADD_ASSIGN: u8 = 0x05;

/// **0x06** - The subtraction operator `-`.
///
/// ```no_run
/// sub r1 r2 r3 // r1 = r2 - r3
/// ```
pub const SUB: u8 = 0x06;

/// **0x07** - The subtraction assignment operator `-=`.
///
/// ```no_run
/// sub r1 r2 // r1 -= r2
/// ```
pub const SUB_ASSIGN: u8 = 0x07;

/// **0x08** - The multiplication operator `*`.
///
/// ```no_run
/// mul r1 r2 r3 // r1 = r2 * r3
/// ```
pub const MUL: u8 = 0x08;

/// **0x09** - The multiplication assignment operator `*=`.
///
/// ```no_run
/// mul r1 r2 // r1 *= r2
/// ```
pub const MUL_ASSIGN: u8 = 0x09;

/// **0x0A** - The division operator `/`.
///
/// ```no_run
/// div r1 r2 r3 // r1 = r2 / r3
/// ```
pub const DIV: u8 = 0x0A;

/// **0x0B** - The division assignment operator `/=`.
///
/// ```no_run
/// div r1 r2 // r1 /= r2
/// ```
pub const DIV_ASSIGN: u8 = 0x0B;

/// **0x0C** - The remainder operator `%`.
///
/// ```no_run
/// rem r1 r2 r3 // r1 = r2 % r3
/// ```
pub const REM: u8 = 0x0C;

/// **0x0D** - The remainder assignment operator `%=`.
///
/// ```no_run
/// rem r1 r2 // r1 %= r2
/// ```
pub const REM_ASSIGN: u8 = 0x0D;

/// **0x0E** - Arithmetic negation `-`.
///
/// ```no_run
/// not r1 r2 // r1 = -r2
/// ```
pub const NEG: u8 = 0x0E;

/// **0x0F** -The bitwise AND operator `&`.
///
/// ```no_run
/// and r1 r2 r3 // r1 = r2 & r3
/// ```
pub const AND: u8 = 0x0F;

/// **0x10** - The bitwise AND assignment operator `&=`.
///
/// ```no_run
/// and r1 r2 // r1 &= r2
/// ```
pub const AND_ASSIGN: u8 = 0x10;

/// **0x11** - The bitwise OR operator `|`.
///
/// ```no_run
/// or r1 r2 r3 // r1 = r2 | r3
/// ```
pub const OR: u8 = 0x11;

/// **0x12** - The bitwise OR assignment operator `|=`.
///
/// ```no_run
/// or r1 r2 // r1 |= r2
/// ```
pub const OR_ASSIGN: u8 = 0x12;

/// **0x13** - The bitwise XOR operator `^`.
///
/// ```no_run
/// xor r1 r2 r3 // r1 = r2 ^ r3
/// ```
pub const XOR: u8 = 0x13;

/// **0x14** - The bitwise XOR assignment operator `^=`.
///
/// ```no_run
/// xor r1 r2 // r1 ^= r2
/// ```
pub const XOR_ASSIGN: u8 = 0x14;

/// **0x15** - Logical negation `!`
///
/// ```no_run
/// not r1 r2 // r1 = !r2
/// ```
pub const NOT: u8 = 0x15;

/// **0x16** - The left shift operator `<<`.
///
/// ```no_run
/// shl r1 r2 // r1 = r2 << 1
/// ```
pub const SHL: u8 = 0x16;

/// **0x17** - The left shift assignment operator `<<=`.
///
/// ```no_run
/// shl r1 // r1 <<= 1
/// ```
pub const SHL_ASSIGN: u8 = 0x17;

/// **0x18** - The right shift operator `>>`.
///
/// ```no_run
/// shr r1 r2 // r1 = r2 >> 1
/// ```
pub const SHR: u8 = 0x18;

/// **0x19** - The right shift assignment operator `>>=`.
///
/// ```no_run
/// shr r1 // r1 >>= 1
/// ```
pub const SHR_ASSIGN: u8 = 0x19;

/// **0x1A** - The equality operator `==`.
///
/// ```no_run
/// eq r1 r2 r3 // r1 = r2 == r3
/// ```
pub const EQ: u8 = 0x1A;

/// **0x1B** - The inequality operator `!=`.
///
/// ```no_run
/// neq r1 r2 r3 // r1 = r2 != r3
/// ```
pub const NEQ: u8 = 0x1B;

/// **0x1C** - The less than operator `<`.
///
/// ```no_run
/// lt r1 r2 r3 // r1 = r2 < r3
/// ```
pub const LT: u8 = 0x1C;

/// **0x1D** - The less than or equal to operator `<=`.
///
/// ```no_run
/// le r1 r2 r3 // r1 = r2 <= r3
/// ```
pub const LE: u8 = 0x1D;

/// **0x1E** - The greater than operator `>`.
///
/// ```no_run
/// gt r1 r2 r3 // r1 = r2 > r3
/// ```
pub const GT: u8 = 0x1E;

/// **0x1F** - The greater than or equal to operator `>=`.
///
/// ```no_run
/// ge r1 r2 r3 // r1 = r2 >= r3
/// ```
pub const GE: u8 = 0x1F;

/// **0xFF** - Halt execution.
///
/// ```no_run
/// hlt
/// ```
pub const HLT: u8 = 0xFF;
