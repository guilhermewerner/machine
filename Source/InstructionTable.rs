use crate::Instruction;
use crate::Instructions::*;
use crate::Operations::*;
use crate::Payload::PayloadType;
use crate::Types::Byte;
use std::collections::HashMap;

pub struct InstructionTable {
    table: HashMap<Byte, Instruction>,
}

impl Default for InstructionTable {
    fn default() -> Self {
        Self {
            table: HashMap::new(),
        }
    }
}

impl InstructionTable {
    #[inline]
    pub fn New() -> Self {
        let mut table = InstructionTable::default();

        table.Insert(Instruction::New(NOP, "nop",PayloadType::Nothing, Nothing));
        table.Insert(Instruction::New(LDR_B, "ldr b",PayloadType::Nothing, Nothing));
        table.Insert(Instruction::New(LDR_H, "ldr h",PayloadType::Nothing, Nothing));
        table.Insert(Instruction::New(LDR_W, "ldr w",PayloadType::Nothing, Nothing));
        table.Insert(Instruction::New(LDR_D, "ldr d",PayloadType::Nothing, Nothing));
        table.Insert(Instruction::New(LDR_Q, "ldr q",PayloadType::Nothing, Nothing));
        table.Insert(Instruction::New(LDA_B, "lda b",PayloadType::Nothing, Nothing));
        table.Insert(Instruction::New(LDA_H, "lda h",PayloadType::Nothing, Nothing));
        table.Insert(Instruction::New(LDA_W, "lda w",PayloadType::Nothing, Nothing));
        table.Insert(Instruction::New(LDA_D, "lda d",PayloadType::Nothing, Nothing));
        table.Insert(Instruction::New(LDA_Q, "lda q",PayloadType::Nothing, Nothing));
        table.Insert(Instruction::New(LDI_B, "ldi b",PayloadType::Nothing, Nothing));
        table.Insert(Instruction::New(LDI_H, "ldi h",PayloadType::Nothing, Nothing));
        table.Insert(Instruction::New(LDI_W, "ldi w",PayloadType::Nothing, Nothing));
        table.Insert(Instruction::New(LDI_D, "ldi d",PayloadType::Nothing, Nothing));
        table.Insert(Instruction::New(LDI_Q, "ldi q",PayloadType::Nothing, Nothing));
        table.Insert(Instruction::New(STR_B, "str b",PayloadType::Nothing, Nothing));
        table.Insert(Instruction::New(STR_H, "str h",PayloadType::Nothing, Nothing));
        table.Insert(Instruction::New(STR_W, "str w",PayloadType::Nothing, Nothing));
        table.Insert(Instruction::New(STR_D, "str d",PayloadType::Nothing, Nothing));
        table.Insert(Instruction::New(STR_Q, "str q",PayloadType::Nothing, Nothing));
        table.Insert(Instruction::New(STA_B, "sta b",PayloadType::Nothing, Nothing));
        table.Insert(Instruction::New(STA_H, "sta h",PayloadType::Nothing, Nothing));
        table.Insert(Instruction::New(STA_W, "sta w",PayloadType::Nothing, Nothing));
        table.Insert(Instruction::New(STA_D, "sta d",PayloadType::Nothing, Nothing));
        table.Insert(Instruction::New(STA_Q, "sta q",PayloadType::Nothing, Nothing));
        table.Insert(Instruction::New(STI_B, "sti b",PayloadType::Nothing, Nothing));
        table.Insert(Instruction::New(STI_H, "sti h",PayloadType::Nothing, Nothing));
        table.Insert(Instruction::New(STI_W, "sti w",PayloadType::Nothing, Nothing));
        table.Insert(Instruction::New(STI_D, "sti d",PayloadType::Nothing, Nothing));
        table.Insert(Instruction::New(STI_Q, "sti q",PayloadType::Nothing, Nothing));
        table.Insert(Instruction::New(INC, "inc",PayloadType::Nothing, Nothing));
        table.Insert(Instruction::New(DEC, "dec",PayloadType::Nothing, Nothing));
        table.Insert(Instruction::New(MOV, "mov",PayloadType::Nothing, Nothing));
        table.Insert(Instruction::New(SWP, "swp",PayloadType::Nothing, Nothing));
        table.Insert(Instruction::New(JMP, "jmp",PayloadType::Nothing, Nothing));
        table.Insert(Instruction::New(JMZ, "jmz",PayloadType::Nothing, Nothing));
        table.Insert(Instruction::New(JEQ, "jeq",PayloadType::Nothing, Nothing));
        table.Insert(Instruction::New(JNE, "jne",PayloadType::Nothing, Nothing));
        table.Insert(Instruction::New(JLT, "jlt",PayloadType::Nothing, Nothing));
        table.Insert(Instruction::New(JLE, "jle",PayloadType::Nothing, Nothing));
        table.Insert(Instruction::New(JGT, "jgt",PayloadType::Nothing, Nothing));
        table.Insert(Instruction::New(JGE, "jge",PayloadType::Nothing, Nothing));
        table.Insert(Instruction::New(CALL, "all",PayloadType::Nothing, Nothing));
        table.Insert(Instruction::New(RET, "ret",PayloadType::Nothing, Nothing));
        table.Insert(Instruction::New(PUSH_B, "ush b",PayloadType::Nothing, Nothing));
        table.Insert(Instruction::New(PUSH_H, "ush h",PayloadType::Nothing, Nothing));
        table.Insert(Instruction::New(PUSH_W, "ush w",PayloadType::Nothing, Nothing));
        table.Insert(Instruction::New(PUSH_D, "ush d",PayloadType::Nothing, Nothing));
        table.Insert(Instruction::New(PUSH_Q, "ush q",PayloadType::Nothing, Nothing));
        table.Insert(Instruction::New(PEEK_B, "eek b",PayloadType::Nothing, Nothing));
        table.Insert(Instruction::New(PEEK_H, "eek h",PayloadType::Nothing, Nothing));
        table.Insert(Instruction::New(PEEK_W, "eek w",PayloadType::Nothing, Nothing));
        table.Insert(Instruction::New(PEEK_D, "eek d",PayloadType::Nothing, Nothing));
        table.Insert(Instruction::New(PEEK_Q, "eek q",PayloadType::Nothing, Nothing));
        table.Insert(Instruction::New(POP_B, "pop b",PayloadType::Nothing, Nothing));
        table.Insert(Instruction::New(POP_H, "pop h",PayloadType::Nothing, Nothing));
        table.Insert(Instruction::New(POP_W, "pop w",PayloadType::Nothing, Nothing));
        table.Insert(Instruction::New(POP_D, "pop d",PayloadType::Nothing, Nothing));
        table.Insert(Instruction::New(POP_Q, "pop q",PayloadType::Nothing, Nothing));
        table.Insert(Instruction::New(NEG, "neg",PayloadType::Nothing, Nothing));
        table.Insert(Instruction::New(ADD_U8, "add u8",PayloadType::Nothing, Nothing));
        table.Insert(Instruction::New(ADD_U16, "add u16",PayloadType::Nothing, Nothing));
        table.Insert(Instruction::New(ADD_U32, "add u32",PayloadType::Nothing, Nothing));
        table.Insert(Instruction::New(ADD_U64, "add u64",PayloadType::Nothing, Nothing));
        table.Insert(Instruction::New(ADD_U128, "add u128",PayloadType::Nothing, Nothing));
        table.Insert(Instruction::New(ADD_I8, "add i8",PayloadType::Nothing, Nothing));
        table.Insert(Instruction::New(ADD_I16, "add i16",PayloadType::Nothing, Nothing));
        table.Insert(Instruction::New(ADD_I32, "add i32",PayloadType::Nothing, Nothing));
        table.Insert(Instruction::New(ADD_I64, "add i64",PayloadType::Nothing, Nothing));
        table.Insert(Instruction::New(ADD_I128, "add i128",PayloadType::Nothing, Nothing));
        table.Insert(Instruction::New(ADD_F32, "add f32",PayloadType::Nothing, Nothing));
        table.Insert(Instruction::New(ADD_F64, "add f64",PayloadType::Nothing, Nothing));
        table.Insert(Instruction::New(SUB_U8, "sub u8",PayloadType::Nothing, Nothing));
        table.Insert(Instruction::New(SUB_U16, "sub u16",PayloadType::Nothing, Nothing));
        table.Insert(Instruction::New(SUB_U32, "sub u32",PayloadType::Nothing, Nothing));
        table.Insert(Instruction::New(SUB_U64, "sub u64",PayloadType::Nothing, Nothing));
        table.Insert(Instruction::New(SUB_U128, "sub u128",PayloadType::Nothing, Nothing));
        table.Insert(Instruction::New(SUB_I8, "sub i8",PayloadType::Nothing, Nothing));
        table.Insert(Instruction::New(SUB_I16, "sub i16",PayloadType::Nothing, Nothing));
        table.Insert(Instruction::New(SUB_I32, "sub i32",PayloadType::Nothing, Nothing));
        table.Insert(Instruction::New(SUB_I64, "sub i64",PayloadType::Nothing, Nothing));
        table.Insert(Instruction::New(SUB_I128, "sub i128",PayloadType::Nothing, Nothing));
        table.Insert(Instruction::New(SUB_F32, "sub f32",PayloadType::Nothing, Nothing));
        table.Insert(Instruction::New(SUB_F64, "sub f64",PayloadType::Nothing, Nothing));
        table.Insert(Instruction::New(MUL_U8, "mul u8",PayloadType::Nothing, Nothing));
        table.Insert(Instruction::New(MUL_U16, "mul u16",PayloadType::Nothing, Nothing));
        table.Insert(Instruction::New(MUL_U32, "mul u32",PayloadType::Nothing, Nothing));
        table.Insert(Instruction::New(MUL_U64, "mul u64",PayloadType::Nothing, Nothing));
        table.Insert(Instruction::New(MUL_U128, "mul u128",PayloadType::Nothing, Nothing));
        table.Insert(Instruction::New(MUL_I8, "mul i8",PayloadType::Nothing, Nothing));
        table.Insert(Instruction::New(MUL_I16, "mul i16",PayloadType::Nothing, Nothing));
        table.Insert(Instruction::New(MUL_I32, "mul i32",PayloadType::Nothing, Nothing));
        table.Insert(Instruction::New(MUL_I64, "mul i64",PayloadType::Nothing, Nothing));
        table.Insert(Instruction::New(MUL_I128, "mul i128",PayloadType::Nothing, Nothing));
        table.Insert(Instruction::New(MUL_F32, "mul f32",PayloadType::Nothing, Nothing));
        table.Insert(Instruction::New(MUL_F64, "mul f64",PayloadType::Nothing, Nothing));
        table.Insert(Instruction::New(DIV_U8, "div u8",PayloadType::Nothing, Nothing));
        table.Insert(Instruction::New(DIV_U16, "div u16",PayloadType::Nothing, Nothing));
        table.Insert(Instruction::New(DIV_U32, "div u32",PayloadType::Nothing, Nothing));
        table.Insert(Instruction::New(DIV_U64, "div u64",PayloadType::Nothing, Nothing));
        table.Insert(Instruction::New(DIV_U128, "div u128",PayloadType::Nothing, Nothing));
        table.Insert(Instruction::New(DIV_I8, "div i8",PayloadType::Nothing, Nothing));
        table.Insert(Instruction::New(DIV_I16, "div i16",PayloadType::Nothing, Nothing));
        table.Insert(Instruction::New(DIV_I32, "div i32",PayloadType::Nothing, Nothing));
        table.Insert(Instruction::New(DIV_I64, "div i64",PayloadType::Nothing, Nothing));
        table.Insert(Instruction::New(DIV_I128, "div i128",PayloadType::Nothing, Nothing));
        table.Insert(Instruction::New(DIV_F32, "div f32",PayloadType::Nothing, Nothing));
        table.Insert(Instruction::New(DIV_F64, "div f64",PayloadType::Nothing, Nothing));
        table.Insert(Instruction::New(REM_U8, "rem u8",PayloadType::Nothing, Nothing));
        table.Insert(Instruction::New(REM_U16, "rem u16",PayloadType::Nothing, Nothing));
        table.Insert(Instruction::New(REM_U32, "rem u32",PayloadType::Nothing, Nothing));
        table.Insert(Instruction::New(REM_U64, "rem u64",PayloadType::Nothing, Nothing));
        table.Insert(Instruction::New(REM_U128, "rem u128",PayloadType::Nothing, Nothing));
        table.Insert(Instruction::New(REM_I8, "rem i8",PayloadType::Nothing, Nothing));
        table.Insert(Instruction::New(REM_I16, "rem i16",PayloadType::Nothing, Nothing));
        table.Insert(Instruction::New(REM_I32, "rem i32",PayloadType::Nothing, Nothing));
        table.Insert(Instruction::New(REM_I64, "rem i64",PayloadType::Nothing, Nothing));
        table.Insert(Instruction::New(REM_I128, "rem i128",PayloadType::Nothing, Nothing));
        table.Insert(Instruction::New(REM_F32, "rem f32",PayloadType::Nothing, Nothing));
        table.Insert(Instruction::New(REM_F64, "rem f64",PayloadType::Nothing, Nothing));
        table.Insert(Instruction::New(AND, "and",PayloadType::Nothing, Nothing));
        table.Insert(Instruction::New(OR, " or",PayloadType::Nothing, Nothing));
        table.Insert(Instruction::New(XOR, "xor",PayloadType::Nothing, Nothing));
        table.Insert(Instruction::New(NOT, "not",PayloadType::Nothing, Nothing));
        table.Insert(Instruction::New(NAND, "and",PayloadType::Nothing, Nothing));
        table.Insert(Instruction::New(NOR, "nor",PayloadType::Nothing, Nothing));
        table.Insert(Instruction::New(XNOR, "nor",PayloadType::Nothing, Nothing));
        table.Insert(Instruction::New(SHL, "shl",PayloadType::Nothing, Nothing));
        table.Insert(Instruction::New(SHR, "shr",PayloadType::Nothing, Nothing));
        table.Insert(Instruction::New(EQ, " eq",PayloadType::Nothing, Nothing));
        table.Insert(Instruction::New(NE, " ne",PayloadType::Nothing, Nothing));
        table.Insert(Instruction::New(LT, " lt",PayloadType::Nothing, Nothing));
        table.Insert(Instruction::New(LE, " le",PayloadType::Nothing, Nothing));
        table.Insert(Instruction::New(GT, " gt",PayloadType::Nothing, Nothing));
        table.Insert(Instruction::New(GE, " ge",PayloadType::Nothing, Nothing));
        table.Insert(Instruction::New(HLT, "hlt",PayloadType::Nothing, Nothing));

        table
    }

    #[inline]
    pub fn IsEmpty(&self) -> bool {
        self.table.is_empty()
    }

    #[inline]
    pub fn Get(&self, code: Byte) -> Option<&Instruction> {
        self.table.get(&code)
    }

    #[inline]
    pub fn Insert(&mut self, instruction: Instruction) {
        self.table.insert(instruction.code, instruction);
    }
}
