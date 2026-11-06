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
        _ => unreachable!("invalid unary alu op code"),
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
        _ => unreachable!("invalid binary alu op code"),
    }
}

pub fn get_conditional_jump_op_code(literal: u8) -> JumpOpCode {
    match literal {
        0b01110000 => JumpOpCode::JO,
        0b01110001 => JumpOpCode::JNO,
        0b01110010 => JumpOpCode::JB,
        0b01110011 => JumpOpCode::JNB,
        0b01110100 => JumpOpCode::JE,
        0b01110101 => JumpOpCode::JNE,
        0b01110110 => JumpOpCode::JBE,
        0b01110111 => JumpOpCode::JNBE,
        0b01111000 => JumpOpCode::JS,
        0b01111001 => JumpOpCode::JNS,
        0b01111010 => JumpOpCode::JP,
        0b01111011 => JumpOpCode::JNP,
        0b01111100 => JumpOpCode::JL,
        0b01111101 => JumpOpCode::JNL,
        0b01111110 => JumpOpCode::JLE,
        0b01111111 => JumpOpCode::JNLE,
        0b11100000 => JumpOpCode::LOOPNZ,
        0b11100001 => JumpOpCode::LOOPZ,
        0b11100010 => JumpOpCode::LOOP,
        0b11100011 => JumpOpCode::JCXZ,
        _ => unreachable!("invalid jump op code"),
    }
}
