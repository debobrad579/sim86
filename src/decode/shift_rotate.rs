use crate::{
    decode::memory::{ModRMType, decode_modrm},
    types::{
        Immediate, Instruction, InstructionData, Operand, Register, SegmentRegister,
        ShiftRotateOpCode, Width,
    },
};

pub fn shift_rotate_rm(bytes: &[u8], segment_override: Option<SegmentRegister>) -> Instruction {
    let v = (bytes[0] & 0b00000010) >> 1;
    let literal = (bytes[1] & 0b00111000) >> 3;

    let op_code = get_shift_rotate_op_code(literal);
    let mod_rm = decode_modrm(bytes, ModRMType::Register, segment_override);

    let amount = if v == 0 {
        Operand::Immediate(Immediate::Unsigned(1))
    } else {
        Operand::Register(Register {
            width: Width::Byte,
            value: 1,
        })
    };

    Instruction {
        data: InstructionData::ShiftRotate {
            op_code,
            width: mod_rm.width,
            operand: mod_rm.rm,
            amount,
        },
        size: mod_rm.size,
    }
}

fn get_shift_rotate_op_code(literal: u8) -> ShiftRotateOpCode {
    match literal {
        0b000 => ShiftRotateOpCode::ROL,
        0b001 => ShiftRotateOpCode::ROR,
        0b010 => ShiftRotateOpCode::RCL,
        0b011 => ShiftRotateOpCode::RCR,
        0b100 => ShiftRotateOpCode::SHL,
        0b101 => ShiftRotateOpCode::SHR,
        0b111 => ShiftRotateOpCode::SAR,
        _ => unreachable!("invalid shift/rotate op code"),
    }
}
