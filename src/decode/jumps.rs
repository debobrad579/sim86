use crate::types::{
    instruction::{Instruction, InstructionData},
    jump::ConditionalJumpOpCode,
};

fn get_conditional_jump_op_code(literal: u8) -> ConditionalJumpOpCode {
    match literal {
        0b01110000 => ConditionalJumpOpCode::JO,
        0b01110001 => ConditionalJumpOpCode::JNO,
        0b01110010 => ConditionalJumpOpCode::JB,
        0b01110011 => ConditionalJumpOpCode::JNB,
        0b01110100 => ConditionalJumpOpCode::JE,
        0b01110101 => ConditionalJumpOpCode::JNE,
        0b01110110 => ConditionalJumpOpCode::JBE,
        0b01110111 => ConditionalJumpOpCode::JNBE,
        0b01111000 => ConditionalJumpOpCode::JS,
        0b01111001 => ConditionalJumpOpCode::JNS,
        0b01111010 => ConditionalJumpOpCode::JP,
        0b01111011 => ConditionalJumpOpCode::JNP,
        0b01111100 => ConditionalJumpOpCode::JL,
        0b01111101 => ConditionalJumpOpCode::JNL,
        0b01111110 => ConditionalJumpOpCode::JLE,
        0b01111111 => ConditionalJumpOpCode::JNLE,
        0b11100000 => ConditionalJumpOpCode::LOOPNZ,
        0b11100001 => ConditionalJumpOpCode::LOOPZ,
        0b11100010 => ConditionalJumpOpCode::LOOP,
        0b11100011 => ConditionalJumpOpCode::JCXZ,
        _ => unreachable!("invalid jump op code"),
    }
}

pub fn conditional_jump(bytes: &[u8]) -> Instruction {
    Instruction {
        data: InstructionData::Jump {
            op_code: get_conditional_jump_op_code(bytes[0]),
            target: bytes[1] as i8,
        },
        size: 2,
    }
}
