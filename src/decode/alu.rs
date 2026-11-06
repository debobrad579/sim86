use crate::{
    decode::{
        Direction,
        memory::{ModRMType, decode_modrm},
        op_code::{get_alu_binary_op_code, get_alu_unary_op_code},
    },
    types::{Immediate, Instruction, InstructionData, Operand, Register, SegmentRegister, Width},
};

pub fn rm_with_reg_to_either(
    bytes: &[u8],
    offset: usize,
    segment_override: Option<SegmentRegister>,
) -> Instruction {
    let direction = Direction::from((bytes[offset] & 0b00000010) >> 1);
    let literal = (bytes[offset] & 0b00111000) >> 3;

    let op_code = get_alu_binary_op_code(literal);
    let mod_rm = decode_modrm(bytes, ModRMType::Register, segment_override, offset);

    match direction {
        Direction::RegToRM => Instruction {
            data: InstructionData::AluBinary {
                op_code,
                width: mod_rm.width,
                destination: mod_rm.rm,
                source: mod_rm.reg,
            },
            size: mod_rm.size,
        },
        Direction::RMToReg => Instruction {
            data: InstructionData::AluBinary {
                op_code,
                width: mod_rm.width,
                destination: mod_rm.reg,
                source: mod_rm.rm,
            },
            size: mod_rm.size,
        },
    }
}

pub fn immediate_from_rm(
    bytes: &[u8],
    offset: usize,
    segment_override: Option<SegmentRegister>,
) -> Instruction {
    let sign_extend = bytes[offset] & 0b00000010 != 0;
    let literal = (bytes[offset + 1] & 0b00111000) >> 3;

    let op_code = get_alu_binary_op_code(literal);
    let mod_rm = decode_modrm(
        bytes,
        ModRMType::Immediate(sign_extend),
        segment_override,
        offset,
    );

    Instruction {
        data: InstructionData::AluBinary {
            op_code,
            width: mod_rm.width,
            destination: mod_rm.rm,
            source: mod_rm.reg,
        },
        size: mod_rm.size,
    }
}

pub fn immediate_from_accumulator(bytes: &[u8], offset: usize) -> Instruction {
    let width = Width::from(bytes[offset] & 0b00000001);
    let literal = (bytes[offset] & 0b00111000) >> 3;

    let op_code = get_alu_binary_op_code(literal);

    let (data, size) = match width {
        Width::Byte => (bytes[offset + 1] as u16, 2),
        Width::Word => (
            u16::from_le_bytes([bytes[offset + 1], bytes[offset + 2]]),
            3,
        ),
    };

    let immediate = Operand::Immediate(Immediate::Unsigned(data));
    let register = Operand::Register(Register { width, value: 0 });

    Instruction {
        data: InstructionData::AluBinary {
            op_code,
            width,
            destination: register,
            source: immediate,
        },
        size,
    }
}

pub fn unary_rm(
    bytes: &[u8],
    offset: usize,
    segment_override: Option<SegmentRegister>,
) -> Instruction {
    let literal = (bytes[offset + 1] & 0b00111000) >> 3;

    let op_code = get_alu_unary_op_code(literal);
    let mod_rm = decode_modrm(bytes, ModRMType::Register, segment_override, offset);

    Instruction {
        data: InstructionData::AluUnary {
            op_code,
            width: mod_rm.width,
            operand: mod_rm.rm,
        },
        size: mod_rm.size,
    }
}

pub fn unary_reg(bytes: &[u8], offset: usize) -> Instruction {
    let literal = (bytes[offset] & 0b00111000) >> 3;
    let reg = bytes[offset] & 0b00000111;

    Instruction {
        data: InstructionData::AluUnary {
            op_code: get_alu_unary_op_code(literal),
            width: Width::Word,
            operand: Operand::Register(Register {
                width: Width::Word,
                value: reg,
            }),
        },
        size: offset + 1,
    }
}
