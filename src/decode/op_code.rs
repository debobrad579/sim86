use crate::types::{AluBinaryOpCode, AluUnaryOpCode, JumpOpCode};

pub fn get_alu_unary_op_code(literal: u8) -> AluUnaryOpCode {
    match literal {
        0b000 => AluUnaryOpCode::INC,
        0b001 => AluUnaryOpCode::DEC,
        0b010 => AluUnaryOpCode::NOT,
        0b011 => AluUnaryOpCode::NEG,
        0b100 => AluUnaryOpCode::MUL,
        0b101 => AluUnaryOpCode::IMUL,
        0b110 => AluUnaryOpCode::DIV,
        0b111 => AluUnaryOpCode::IDIV,
        _ => unreachable!(),
    }
}

pub fn get_alu_binary_op_code(literal: u8) -> AluBinaryOpCode {
    match literal {
        0b000 => AluBinaryOpCode::ADD,
        0b001 => AluBinaryOpCode::OR,
        0b010 => AluBinaryOpCode::ADC,
        0b011 => AluBinaryOpCode::SBB,
        0b100 => AluBinaryOpCode::AND,
        0b101 => AluBinaryOpCode::SUB,
        0b110 => AluBinaryOpCode::XOR,
        0b111 => AluBinaryOpCode::CMP,
        _ => unreachable!(),
    }
}

pub fn get_conditional_jump_op_code(literal: u8) -> Option<JumpOpCode> {
    match literal {
        0b01110000 => Some(JumpOpCode::JO),
        0b01110001 => Some(JumpOpCode::JNO),
        0b01110010 => Some(JumpOpCode::JB),
        0b01110011 => Some(JumpOpCode::JNB),
        0b01110100 => Some(JumpOpCode::JE),
        0b01110101 => Some(JumpOpCode::JNE),
        0b01110110 => Some(JumpOpCode::JBE),
        0b01110111 => Some(JumpOpCode::JNBE),
        0b01111000 => Some(JumpOpCode::JS),
        0b01111001 => Some(JumpOpCode::JNS),
        0b01111010 => Some(JumpOpCode::JP),
        0b01111011 => Some(JumpOpCode::JNP),
        0b01111100 => Some(JumpOpCode::JL),
        0b01111101 => Some(JumpOpCode::JNL),
        0b01111110 => Some(JumpOpCode::JLE),
        0b01111111 => Some(JumpOpCode::JNLE),
        0b11100000 => Some(JumpOpCode::LOOPNZ),
        0b11100001 => Some(JumpOpCode::LOOPZ),
        0b11100010 => Some(JumpOpCode::LOOP),
        0b11100011 => Some(JumpOpCode::JCXZ),
        _ => None,
    }
}
