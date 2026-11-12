use crate::{
    decode::{
        DecodeInstruction, Direction,
        memory::{ModRMType, decode_modrm},
    },
    types::{
        instruction::{AluBinaryOpCode, AluUnaryOpCode, InstructionData},
        operand::{Immediate, Operand, Register, SegmentRegister},
        width::Width,
    },
};

fn get_alu_unary_op_code(literal: u8) -> AluUnaryOpCode {
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

fn get_alu_binary_op_code(literal: u8) -> AluBinaryOpCode {
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

pub fn rm_with_reg_to_either(
    bytes: &[u8],
    segment_override: Option<SegmentRegister>,
    op_code_override: Option<AluBinaryOpCode>,
) -> DecodeInstruction {
    let direction = Direction::from((bytes[0] & 0b00000010) >> 1);
    let literal = (bytes[0] & 0b00111000) >> 3;
    let op_code = if let Some(op_code_override) = op_code_override {
        op_code_override
    } else {
        get_alu_binary_op_code(literal)
    };

    let mod_rm = decode_modrm(bytes, ModRMType::Register, segment_override);

    match direction {
        Direction::RegToRM => DecodeInstruction {
            data: InstructionData::AluBinary {
                op_code,
                width: mod_rm.width,
                destination: mod_rm.rm,
                source: mod_rm.reg,
            },
            size: mod_rm.size,
        },
        Direction::RMToReg => DecodeInstruction {
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
) -> DecodeInstruction {
    let sign_extend = bytes[0] & 0b00000010 != 0;
    let literal = (bytes[1] & 0b00111000) >> 3;

    let op_code = if let Some(op_code_override) = op_code_override {
        op_code_override
    } else {
        get_alu_binary_op_code(literal)
    };

    let mod_rm = decode_modrm(bytes, ModRMType::Immediate(sign_extend), segment_override);

    DecodeInstruction {
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
) -> DecodeInstruction {
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

    DecodeInstruction {
        data: InstructionData::AluBinary {
            op_code,
            width,
            destination: Operand::Register(Register { width, value: 0 }),
            source: Operand::Immediate(Immediate::Unsigned(data)),
        },
        size,
    }
}

pub fn unary_rm(bytes: &[u8], segment_override: Option<SegmentRegister>) -> DecodeInstruction {
    let literal = (bytes[1] & 0b00111000) >> 3;

    let op_code = get_alu_unary_op_code(literal);
    let mod_rm = decode_modrm(bytes, ModRMType::Register, segment_override);

    DecodeInstruction {
        data: InstructionData::AluUnary {
            op_code,
            width: mod_rm.width,
            operand: mod_rm.rm,
        },
        size: mod_rm.size,
    }
}

pub fn unary_reg(bytes: &[u8]) -> DecodeInstruction {
    let literal = (bytes[0] & 0b00111000) >> 3;
    let reg = bytes[0] & 0b00000111;

    DecodeInstruction {
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
