use crate::{
    decode::DecodeInstruction,
    types::{
        instruction::{InstructionData, StringOpCode},
        width::Width,
    },
};

fn get_string_op_code(literal: u8) -> StringOpCode {
    match literal {
        0b010 => StringOpCode::MOVS,
        0b011 => StringOpCode::CMPS,
        0b111 => StringOpCode::SCAS,
        0b110 => StringOpCode::LODS,
        0b101 => StringOpCode::STOS,
        _ => unreachable!("invalid string op code"),
    }
}

pub fn string_instruction(bytes: &[u8], rep: Option<bool>) -> DecodeInstruction {
    DecodeInstruction {
        data: InstructionData::String {
            op_code: get_string_op_code((bytes[0] >> 1) & 0b111),
            rep,
            width: Width::from(bytes[0] & 0b1),
        },
        size: 1,
    }
}
