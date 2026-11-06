use crate::{
    decode::{
        Direction,
        memory::{ModRMType, decode_modrm},
        op_code::{get_alu_binary_op_code, get_alu_unary_op_code},
    },
    types::{
        AluBinaryOpCode, Immediate, Instruction, InstructionData, Operand, Register,
        SegmentRegister, Width,
    },
};

pub fn rm_with_reg_to_either(
    bytes: &[u8],
    segment_override: Option<SegmentRegister>,
    op_code_override: Option<AluBinaryOpCode>,
) -> Instruction {
    let direction = Direction::from((bytes[0] & 0b00000010) >> 1);
    let literal = (bytes[0] & 0b00111000) >> 3;

    let op_code = if let Some(op_code_override) = op_code_override {
        op_code_override
    } else {
        get_alu_binary_op_code(literal)
    };

    let mod_rm = decode_modrm(bytes, ModRMType::Register, segment_override);

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

pub fn immediate_to_rm(
    bytes: &[u8],
    segment_override: Option<SegmentRegister>,
    op_code_override: Option<AluBinaryOpCode>,
) -> Instruction {
    let sign_extend = bytes[0] & 0b00000010 != 0;
    let literal = (bytes[1] & 0b00111000) >> 3;

    let op_code = if let Some(op_code_override) = op_code_override {
        op_code_override
    } else {
        get_alu_binary_op_code(literal)
    };

    let mod_rm = decode_modrm(bytes, ModRMType::Immediate(sign_extend), segment_override);

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

pub fn immediate_to_accumulator(
    bytes: &[u8],
    op_code_override: Option<AluBinaryOpCode>,
) -> Instruction {
    let literal = (bytes[0] & 0b00111000) >> 3;

    let op_code = if let Some(op_code_override) = op_code_override {
        op_code_override
    } else {
        get_alu_binary_op_code(literal)
    };

    let width = Width::from(bytes[0] & 0b00000001);

    let (data, size) = match width {
        Width::Byte => (bytes[1] as u16, 2),
        Width::Word => (u16::from_le_bytes([bytes[1], bytes[2]]), 3),
    };

    Instruction {
        data: InstructionData::AluBinary {
            op_code,
            width,
            destination: Operand::Register(Register { width, value: 0 }),
            source: Operand::Immediate(Immediate::Unsigned(data)),
        },
        size,
    }
}

pub fn unary_rm(bytes: &[u8], segment_override: Option<SegmentRegister>) -> Instruction {
    let literal = (bytes[1] & 0b00111000) >> 3;

    let op_code = get_alu_unary_op_code(literal);
    let mod_rm = decode_modrm(bytes, ModRMType::Register, segment_override);

    Instruction {
        data: InstructionData::AluUnary {
            op_code,
            width: mod_rm.width,
            operand: mod_rm.rm,
        },
        size: mod_rm.size,
    }
}

pub fn unary_reg(bytes: &[u8]) -> Instruction {
    let literal = (bytes[0] & 0b00111000) >> 3;
    let reg = bytes[0] & 0b00000111;

    Instruction {
        data: InstructionData::AluUnary {
            op_code: get_alu_unary_op_code(literal),
            width: Width::Word,
            operand: Operand::Register(Register {
                width: Width::Word,
                value: reg,
            }),
        },
        size: 1,
    }
}
