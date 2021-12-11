#[repr(u8)]
#[allow(non_camel_case_types)]
pub enum Mnemonic {
    /// **0x00** - No operation.
    ///
    /// ```no_run
    /// nop
    /// ```
    NOP = 0x00,

    /// **0x01** - Load register with byte from memory address.
    ///
    /// ```no_run
    /// ldr b r1 r2 // r1 = heap[r2]
    /// ```
    LDR_B = 0x01,

    /// **0x02** - Load register with half word from memory address.
    ///
    /// ```no_run
    /// ldr h r1 r2 // r1 = heap[r2..r2 + 2]
    /// ```
    LDR_H = 0x02,

    /// **0x03** - Load register with word from memory address.
    ///
    /// ```no_run
    /// ldr w r1 r2 // r1 = heap[r2..r2 + 4]
    /// ```
    LDR_W = 0x03,

    /// **0x04** - Load register with double word from memory address.
    ///
    /// ```no_run
    /// ldr d r1 r2 // r1 = heap[r2..r2 + 8]
    /// ```
    LDR_D = 0x04,

    /// **0x05** - Load register with quad word from memory address.
    ///
    /// ```no_run
    /// ldr q r1 r2 r3 // r1 = heap[r3..r3 + 8], r2 = heap[r3 + 8..r3 + 16]
    /// ```
    LDR_Q = 0x05,

    /// **0x06** - Load register with byte from memory address.
    ///
    /// ```no_run
    /// lda b r1 0050 // r1 = heap[0050]
    /// ```
    LDA_B = 0x06,

    /// **0x07** - Load register with half word from memory address.
    ///
    /// ```no_run
    /// lda h r1 0050 // r1 = heap[0050..0050 + 2]
    /// ```
    LDA_H = 0x07,

    /// **0x08** - Load register with word from memory address.
    ///
    /// ```no_run
    /// lda w r1 0050 // r1 = heap[0050..0050 + 4]
    /// ```
    LDA_W = 0x08,

    /// **0x09** - Load register with double word from memory address.
    ///
    /// ```no_run
    /// lda d r1 0050 // r1 = heap[0050..0050 + 8]
    /// ```
    LDA_D = 0x09,

    /// **0x0A** - Load register with quad word from memory address.
    ///
    /// ```no_run
    /// lda q r1 r2 0050 // r1 = heap[0050..0050 + 8], r2 = heap[0050 + 8..0050 + 16]
    /// ```
    LDA_Q = 0x0A,

    /// **0x0B** - Load register with byte from imediate value.
    ///
    /// ```no_run
    /// ldi b r1 1 // r1 = 1
    /// ```
    LDI_B = 0x0B,

    /// **0x0C** - Load register with half word from imediate value.
    ///
    /// ```no_run
    /// ldi h r1 1 // r1 = 1
    /// ```
    LDI_H = 0x0C,

    /// **0x0D** - Load register with word from imediate value.
    ///
    /// ```no_run
    /// ldi w r1 1 // r1 = 1
    /// ```
    LDI_W = 0x0D,

    /// **0x0E** - Load register with double word from imediate value.
    ///
    /// ```no_run
    /// ldi d r1 1 // r1 = 1
    /// ```
    LDI_D = 0x0E,

    /// **0x0F** - Load register with quad word from imediate value.
    ///
    /// ```no_run
    /// ldi q r1 r2 11 // r1 = 1, r2 = 1
    /// ```
    LDI_Q = 0x0F,

    /// **0x10** - Store byte in memory address from register.
    ///
    /// ```no_run
    /// str b r1 r2 // heap[r1] = r2
    /// ```
    STR_B = 0x10,

    /// **0x11** - Store half word in memory address from register.
    ///
    /// ```no_run
    /// str h r1 r2 // heap[r1..r1 + 2] = r2
    /// ```
    STR_H = 0x11,

    /// **0x12** - Store word in memory address from register.
    ///
    /// ```no_run
    /// str w r1 r2 // heap[r1..r1 + 4] = r2
    /// ```
    STR_W = 0x12,

    /// **0x13** - Store double word in memory address from register.
    ///
    /// ```no_run
    /// str d r1 r2 // heap[r1..r1 + 8] = r2
    /// ```
    STR_D = 0x13,

    /// **0x14** - Store quad word in memory address from register.
    ///
    /// ```no_run
    /// str q r1 r2 r3 // heap[r1..r1 + 8] = r2, heap[r1 + 8..r1 + 16] = r3
    /// ```
    STR_Q = 0x14,

    /// **0x15** - Store byte in memory address from register.
    ///
    /// ```no_run
    /// sta b 0050 r1 // heap[0050] = r1
    /// ```
    STA_B = 0x15,

    /// **0x16** - Store half word in memory address from register.
    ///
    /// ```no_run
    /// sta h 0050 r1 // heap[0050..0050 + 2] = r1
    /// ```
    STA_H = 0x16,

    /// **0x17** - Store word in memory address from register.
    ///
    /// ```no_run
    /// sta w 0050 r1 // heap[0050..0050 + 4] = r1
    /// ```
    STA_W = 0x17,

    /// **0x18** - Store double word in memory address from register.
    ///
    /// ```no_run
    /// sta d 0050 r1 // heap[0050..0050 + 8] = r1
    /// ```
    STA_D = 0x18,

    /// **0x19** - Store quad word in memory address from register.
    ///
    /// ```no_run
    /// sta q 0050 r1 r2 // heap[0050..0050 + 8] = r1, heap[0050 + 8..0050 + 16] = r2
    /// ```
    STA_Q = 0x19,

    /// **0x1A** - Store byte in memory address from imediate value.
    ///
    /// ```no_run
    /// sti b 0050 1 // heap[0050] = 1
    /// ```
    STI_B = 0x1A,

    /// **0x1B** - Store half word in memory address from imediate value.
    ///
    /// ```no_run
    /// sti h 0050 1 // heap[0050..0050 + 2] = 1
    /// ```
    STI_H = 0x1B,

    /// **0x1C** - Store word in memory address from imediate value.
    ///
    /// ```no_run
    /// sti w 0050 1 // heap[0050..0050 + 4] = 1
    /// ```
    STI_W = 0x1C,

    /// **0x1D** - Store double word in memory address from imediate value.
    ///
    /// ```no_run
    /// sti d 0050 1 // heap[0050..0050 + 8] = 1
    /// ```
    STI_D = 0x1D,

    /// **0x1E** - Store quad word in memory address from imediate value.
    ///
    /// ```no_run
    /// sti q 0050 1 // heap[0050..0050 + 16] = 1
    /// ```
    STI_Q = 0x1E,

    /// **0x1F** - Increment register value.
    ///
    /// ```no_run
    /// inc r1 // r1 += 1
    /// ```
    INC = 0x1F,

    /// **0x20** - Decrement register value.
    ///
    /// ```no_run
    /// dec r1 // r1 -= 1
    /// ```
    DEC = 0x20,

    /// **0x21** - Move register value to other register.
    ///
    /// ```no_run
    /// mov r1 r2
    /// ```
    MOV = 0x21,

    /// **0x22** - Swap values of two registers.
    ///
    /// ```no_run
    /// swp r1 r2
    /// ```
    SWP = 0x22,

    /// **0x23** - Jump to instruction.
    ///
    /// ```no_run
    /// jmp 0050 // pc = 0050
    /// ```
    JMP = 0x23,

    /// **0x24** - Jump to instruction, if stack.peek() == 0.
    ///
    /// ```no_run
    /// jmz 0050 // pc = 0050
    /// ```
    JMZ = 0x24,

    /// **0x25** - Jump to instruction, if last comparation was equals.
    ///
    /// ```no_run
    /// jeq 0050 // pc = 0050
    /// ```
    JEQ = 0x25,

    /// **0x26** - Jump to instruction, if last comparation was not equals.
    ///
    /// ```no_run
    /// jne 0050 // pc = 0050
    /// ```
    JNE = 0x26,

    /// **0x27** - Jump to instruction, if last comparation was less than.
    ///
    /// ```no_run
    /// jlt 0050 // pc = 0050
    /// ```
    JLT = 0x27,

    /// **0x28** - Jump to instruction, if last comparation was less tan or equals.
    ///
    /// ```no_run
    /// jle 0050 // pc = 0050
    /// ```
    JLE = 0x28,

    /// **0x29** - Jump to instruction, if last comparation was greater than.
    ///
    /// ```no_run
    /// jgt 0050 // pc = 0050
    /// ```
    JGT = 0x29,

    /// **0x2A** - Jump to instruction, if last comparation was greater than or equals.
    ///
    /// ```no_run
    /// jge 0050 // pc = 0050
    /// ```
    JGE = 0x2A,

    /// **0x2B** - Call function with label.
    ///
    /// ```no_run
    /// call 0050 // pc = 0050
    /// ```
    CALL = 0x2B,

    /// **0x2C** - Return from the current function.
    ///
    /// ```no_run
    /// ret // pc = lr
    /// ```
    RET = 0x2C,

    /// **0x2D** - Push byte from register into stack.
    ///
    /// ```no_run
    /// push b r1
    /// ```
    PUSH_B = 0x2D,

    /// **0x2E** - Push half word from register into stack.
    ///
    /// ```no_run
    /// push h r1
    /// ```
    PUSH_H = 0x2E,

    /// **0x2F** - Push word from register into stack.
    ///
    /// ```no_run
    /// push w r1
    /// ```
    PUSH_W = 0x2F,

    /// **0x30** - Push double word from register into stack.
    ///
    /// ```no_run
    /// push d r1
    /// ```
    PUSH_D = 0x30,

    /// **0x31** - Push quad word from register into stack.
    ///
    /// ```no_run
    /// push q r1 r2
    /// ```
    PUSH_Q = 0x31,

    /// **0x32** - Peek byte from stack into register.
    ///
    /// ```no_run
    /// peek b r1 // r1 = stack[sp]
    /// ```
    PEEK_B = 0x32,

    /// **0x33** - Peek half word from stack into register.
    ///
    /// ```no_run
    /// peek h r1 // r1 = stack[sp..sp + 2]
    /// ```
    PEEK_H = 0x33,

    /// **0x34** - Peek word from stack into register.
    ///
    /// ```no_run
    /// peek w r1 // r1 = stack[sp..sp + 4]
    /// ```
    PEEK_W = 0x34,

    /// **0x35** - Peek double word from stack into register.
    ///
    /// ```no_run
    /// peek d r1 // r1 = stack[sp..sp + 8]
    /// ```
    PEEK_D = 0x35,

    /// **0x36** - Peek quad from stack into register.
    ///
    /// ```no_run
    /// peek q r1 r2 // r1 = stack[sp..sp + 8], r2 = stack[sp + 8..sp + 16]
    /// ```
    PEEK_Q = 0x36,

    /// **0x37** - Pop byte from stack into register.
    ///
    /// ```no_run
    /// pop b r1 // r1 = stack[sp]
    /// ```
    POP_B = 0x37,

    /// **0x38** - Pop half word from stack into register.
    ///
    /// ```no_run
    /// pop h r1 // r1 = stack[sp..sp + 2]
    /// ```
    POP_H = 0x38,

    /// **0x39** - Pop word from stack into register.
    ///
    /// ```no_run
    /// pop w r1 // r1 = stack[sp..sp + 4]
    /// ```
    POP_W = 0x39,

    /// **0x3A** - Pop double word from stack into register.
    ///
    /// ```no_run
    /// pop d r1 // r1 = stack[sp..sp + 8]
    /// ```
    POP_D = 0x3A,

    /// **0x3B** - Pop quad word from stack into register.
    ///
    /// ```no_run
    /// pop q r1 r2 // r1 = stack[sp..sp + 8], r2 = stack[sp + 8..sp + 16]
    /// ```
    POP_Q = 0x3B,

    /// **0x3C** - Arithmetic negation `-`.
    ///
    /// ```no_run
    /// neg r1 r2 // r1 = -r2
    /// ```
    NEG = 0x3C,

    /// **0x3D** - No operation.
    ///
    /// ```no_run
    /// add u8 r1 r2 r3
    /// ```
    ADD_U8 = 0x3D,

    /// **0x3E** - No operation.
    ///
    /// ```no_run
    /// add u16 r1 r2 r3
    /// ```
    ADD_U16 = 0x3E,

    /// **0x3F** - No operation.
    ///
    /// ```no_run
    /// add u32 r1 r2 r3
    /// ```
    ADD_U32 = 0x3F,

    /// **0x40** - No operation.
    ///
    /// ```no_run
    /// add u64 r1 r2 r3
    /// ```
    ADD_U64 = 0x40,

    /// **0x41** - No operation.
    ///
    /// ```no_run
    /// add u128 r1 r2 r3
    /// ```
    ADD_U128 = 0x41,

    /// **0x42** - No operation.
    ///
    /// ```no_run
    /// add i8 r1 r2 r3
    /// ```
    ADD_I8 = 0x42,

    /// **0x43** - No operation.
    ///
    /// ```no_run
    /// add i16 r1 r2 r3
    /// ```
    ADD_I16 = 0x43,

    /// **0x44** - No operation.
    ///
    /// ```no_run
    /// add i32 r1 r2 r3
    /// ```
    ADD_I32 = 0x44,

    /// **0x45** - No operation.
    ///
    /// ```no_run
    /// add i64 r1 r2 r3
    /// ```
    ADD_I64 = 0x45,

    /// **0x46** - No operation.
    ///
    /// ```no_run
    /// add i128 r1 r2 r3 r4 r5
    /// ```
    ADD_I128 = 0x46,

    /// **0x47** - No operation.
    ///
    /// ```no_run
    /// add f32 r1 r2 r3
    /// ```
    ADD_F32 = 0x47,

    /// **0x48** - No operation.
    ///
    /// ```no_run
    /// add f64 r1 r2 r3
    /// ```
    ADD_F64 = 0x48,

    /// **0x49** - No operation.
    ///
    /// ```no_run
    /// sub u8 r1 r2 r3
    /// ```
    SUB_U8 = 0x49,

    /// **0x4A** - No operation.
    ///
    /// ```no_run
    /// sub u16 r1 r2 r3
    /// ```
    SUB_U16 = 0x4A,

    /// **0x4B** - No operation.
    ///
    /// ```no_run
    /// sub u32 r1 r2 r3
    /// ```
    SUB_U32 = 0x4B,

    /// **0x4C** - No operation.
    ///
    /// ```no_run
    /// sub u64 r1 r2 r3
    /// ```
    SUB_U64 = 0x4C,

    /// **0x4D** - No operation.
    ///
    /// ```no_run
    /// sub u128 r1 r2 r3
    /// ```
    SUB_U128 = 0x4D,

    /// **0x4E** - No operation.
    ///
    /// ```no_run
    /// sub i8 r1 r2 r3
    /// ```
    SUB_I8 = 0x4E,

    /// **0x4F** - No operation.
    ///
    /// ```no_run
    /// sub i16 r1 r2 r3
    /// ```
    SUB_I16 = 0x4F,

    /// **0x50** - No operation.
    ///
    /// ```no_run
    /// sub i32 r1 r2 r3
    /// ```
    SUB_I32 = 0x50,

    /// **0x51** - No operation.
    ///
    /// ```no_run
    /// sub i64 r1 r2 r3
    /// ```
    SUB_I64 = 0x51,

    /// **0x52** - No operation.
    ///
    /// ```no_run
    /// sub i128 r1 r2 r3 r4 r5
    /// ```
    SUB_I128 = 0x52,

    /// **0x53** - No operation.
    ///
    /// ```no_run
    /// sub f32 r1 r2 r3
    /// ```
    SUB_F32 = 0x53,

    /// **0x54** - No operation.
    ///
    /// ```no_run
    /// sub f64 r1 r2 r3
    /// ```
    SUB_F64 = 0x54,

    /// **0x55** - No operation.
    ///
    /// ```no_run
    /// mul u8 r1 r2 r3
    /// ```
    MUL_U8 = 0x55,

    /// **0x56** - No operation.
    ///
    /// ```no_run
    /// mul u16 r1 r2 r3
    /// ```
    MUL_U16 = 0x56,

    /// **0x57** - No operation.
    ///
    /// ```no_run
    /// mul u32 r1 r2 r3
    /// ```
    MUL_U32 = 0x57,

    /// **0x58** - No operation.
    ///
    /// ```no_run
    /// mul u64 r1 r2 r3
    /// ```
    MUL_U64 = 0x58,

    /// **0x59** - No operation.
    ///
    /// ```no_run
    /// mul u128 r1 r2 r3
    /// ```
    MUL_U128 = 0x59,

    /// **0x5A** - No operation.
    ///
    /// ```no_run
    /// mul i8 r1 r2 r3
    /// ```
    MUL_I8 = 0x5A,

    /// **0x5B** - No operation.
    ///
    /// ```no_run
    /// mul i16 r1 r2 r3
    /// ```
    MUL_I16 = 0x5B,

    /// **0x5C** - No operation.
    ///
    /// ```no_run
    /// mul i32 r1 r2 r3
    /// ```
    MUL_I32 = 0x5C,

    /// **0x5D** - No operation.
    ///
    /// ```no_run
    /// mul i64 r1 r2 r3
    /// ```
    MUL_I64 = 0x5D,

    /// **0x5E** - No operation.
    ///
    /// ```no_run
    /// mul i128 r1 r2 r3 r4 r5
    /// ```
    MUL_I128 = 0x5E,

    /// **0x5F** - No operation.
    ///
    /// ```no_run
    /// mul f32 r1 r2 r3
    /// ```
    MUL_F32 = 0x5F,

    /// **0x60** - No operation.
    ///
    /// ```no_run
    /// mul f64 r1 r2 r3
    /// ```
    MUL_F64 = 0x60,

    /// **0x61** - No operation.
    ///
    /// ```no_run
    /// div u8 r1 r2 r3
    /// ```
    DIV_U8 = 0x61,

    /// **0x62** - No operation.
    ///
    /// ```no_run
    /// div u16 r1 r2 r3
    /// ```
    DIV_U16 = 0x62,

    /// **0x63** - No operation.
    ///
    /// ```no_run
    /// div u32 r1 r2 r3
    /// ```
    DIV_U32 = 0x63,

    /// **0x64** - No operation.
    ///
    /// ```no_run
    /// div u64 r1 r2 r3
    /// ```
    DIV_U64 = 0x64,

    /// **0x65** - No operation.
    ///
    /// ```no_run
    /// div u128 r1 r2 r3
    /// ```
    DIV_U128 = 0x65,

    /// **0x66** - No operation.
    ///
    /// ```no_run
    /// div i8 r1 r2 r3
    /// ```
    DIV_I8 = 0x66,

    /// **0x67** - No operation.
    ///
    /// ```no_run
    /// div i16 r1 r2 r3
    /// ```
    DIV_I16 = 0x67,

    /// **0x68** - No operation.
    ///
    /// ```no_run
    /// div i32 r1 r2 r3
    /// ```
    DIV_I32 = 0x68,

    /// **0x69** - No operation.
    ///
    /// ```no_run
    /// div i64 r1 r2 r3
    /// ```
    DIV_I64 = 0x69,

    /// **0x6A** - No operation.
    ///
    /// ```no_run
    /// div i128 r1 r2 r3 r4 r5
    /// ```
    DIV_I128 = 0x6A,

    /// **0x6B** - No operation.
    ///
    /// ```no_run
    /// div f32 r1 r2 r3
    /// ```
    DIV_F32 = 0x6B,

    /// **0x6C** - No operation.
    ///
    /// ```no_run
    /// div f64 r1 r2 r3
    /// ```
    DIV_F64 = 0x6C,

    /// **0x6D** - No operation.
    ///
    /// ```no_run
    /// rem u8 r1 r2 r3
    /// ```
    REM_U8 = 0x6D,

    /// **0x6E** - No operation.
    ///
    /// ```no_run
    /// rem u16 r1 r2 r3
    /// ```
    REM_U16 = 0x6E,

    /// **0x6F** - No operation.
    ///
    /// ```no_run
    /// rem u32 r1 r2 r3
    /// ```
    REM_U32 = 0x6F,

    /// **0x70** - No operation.
    ///
    /// ```no_run
    /// rem u64 r1 r2 r3
    /// ```
    REM_U64 = 0x70,

    /// **0x71** - No operation.
    ///
    /// ```no_run
    /// rem u128 r1 r2 r3
    /// ```
    REM_U128 = 0x71,

    /// **0x72** - No operation.
    ///
    /// ```no_run
    /// rem i8 r1 r2 r3
    /// ```
    REM_I8 = 0x72,

    /// **0x73** - No operation.
    ///
    /// ```no_run
    /// rem i16 r1 r2 r3
    /// ```
    REM_I16 = 0x73,

    /// **0x74** - No operation.
    ///
    /// ```no_run
    /// rem i32 r1 r2 r3
    /// ```
    REM_I32 = 0x74,

    /// **0x75** - No operation.
    ///
    /// ```no_run
    /// rem i64 r1 r2 r3
    /// ```
    REM_I64 = 0x75,

    /// **0x76** - No operation.
    ///
    /// ```no_run
    /// rem i128 r1 r2 r3 r4 r5
    /// ```
    REM_I128 = 0x76,

    /// **0x77** - No operation.
    ///
    /// ```no_run
    /// rem f32 r1 r2 r3
    /// ```
    REM_F32 = 0x77,

    /// **0x78** - No operation.
    ///
    /// ```no_run
    /// rem f64 r1 r2 r3
    /// ```
    REM_F64 = 0x78,

    /// **0x79** - No operation.
    ///
    /// ```no_run
    /// and r1 r2 r3
    /// ```
    AND = 0x79,

    /// **0x7A** - No operation.
    ///
    /// ```no_run
    /// or r1 r2 r3
    /// ```
    OR = 0x7A,

    /// **0x7B** - No operation.
    ///
    /// ```no_run
    /// xor r1 r2 r3
    /// ```
    XOR = 0x7B,

    /// **0x7C** - No operation.
    ///
    /// ```no_run
    /// not r1 r2
    /// ```
    NOT = 0x7C,

    /// **0x7D** - No operation.
    ///
    /// ```no_run
    /// nand r1 r2 r3
    /// ```
    NAND = 0x7D,

    /// **0x7E** - No operation.
    ///
    /// ```no_run
    /// nor r1 r2 r3
    /// ```
    NOR = 0x7E,

    /// **0x7F** - No operation.
    ///
    /// ```no_run
    /// xnor r1 r2 r3
    /// ```
    XNOR = 0x7F,

    /// **0x80** - No operation.
    ///
    /// ```no_run
    /// shl r1 r2
    /// ```
    SHL = 0x80,

    /// **0x81** - No operation.
    ///
    /// ```no_run
    /// shr r1 r2
    /// ```
    SHR = 0x81,

    /// **0x82** - No operation.
    ///
    /// ```no_run
    /// cmp r1 r2
    /// ```
    CMP = 0x82,

    /// **0x83** - No operation.
    ///
    /// ```no_run
    /// eq r1 r2 r3
    /// ```
    EQ = 0x83,

    /// **0x84** - No operation.
    ///
    /// ```no_run
    /// ne r1 r2 r3
    /// ```
    NE = 0x84,

    /// **0x85** - No operation.
    ///
    /// ```no_run
    /// lt r1 r2 r3
    /// ```
    LT = 0x85,

    /// **0x86** - No operation.
    ///
    /// ```no_run
    /// le r1 r2 r3
    /// ```
    LE = 0x86,

    /// **0x87** - No operation.
    ///
    /// ```no_run
    /// gt r1 r2 r3
    /// ```
    GT = 0x87,

    /// **0x88** - No operation.
    ///
    /// ```no_run
    /// ge r1 r2 r3
    /// ```
    GE = 0x88,

    /// **0xFF** - No operation.
    ///
    /// ```no_run
    /// hlt
    /// ```
    HLT = 0xFF,
}
