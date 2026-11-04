use crate::types::OpCode;

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

pub fn get_conditional_jump_op_code(literal: u8) -> Option<OpCode> {
    match literal {
        0b01110000 => Some(OpCode::JO),
        0b01110001 => Some(OpCode::JNO),
        0b01110010 => Some(OpCode::JB),
        0b01110011 => Some(OpCode::JNB),
        0b01110100 => Some(OpCode::JE),
        0b01110101 => Some(OpCode::JNE),
        0b01110110 => Some(OpCode::JBE),
        0b01110111 => Some(OpCode::JNBE),
        0b01111000 => Some(OpCode::JS),
        0b01111001 => Some(OpCode::JNS),
        0b01111010 => Some(OpCode::JP),
        0b01111011 => Some(OpCode::JNP),
        0b01111100 => Some(OpCode::JL),
        0b01111101 => Some(OpCode::JNL),
        0b01111110 => Some(OpCode::JLE),
        0b01111111 => Some(OpCode::JNLE),
        0b11100000 => Some(OpCode::LOOPNZ),
        0b11100001 => Some(OpCode::LOOPZ),
        0b11100010 => Some(OpCode::LOOP),
        0b11100011 => Some(OpCode::JCXZ),
        _ => None,
    }
}
