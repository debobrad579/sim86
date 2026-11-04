use std::fmt::Display;

pub fn get_alu_op_code(literal: u8) -> OpCode {
    match literal {
        0b000 => OpCode::ADD,
        0b001 => OpCode::OR,
        0b010 => OpCode::ADC,
        0b011 => OpCode::SBB,
        0b100 => OpCode::AND,
        0b101 => OpCode::SUB,
        0b110 => OpCode::XOR,
        0b111 => OpCode::CMP,
        _ => unreachable!(),
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum OpCode {
    MOV,
    ADD,
    SUB,
    ADC,
    SBB,
    CMP,
    AND,
    OR,
    XOR,
}

impl Display for OpCode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}
