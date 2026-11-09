use std::fmt::Display;

use crate::types::{
    jump::{ConditionalJumpOpCode, Jump},
    operand::{EffectiveAddressCalculation, Operand},
    width::Width,
};

pub struct Instruction {
    pub data: InstructionData,
    pub size: usize,
}

pub enum InstructionData {
    MOV {
        width: Width,
        destination: Operand,
        source: Operand,
    },
    PUSH {
        operand: Operand,
    },
    POP {
        operand: Operand,
    },
    LEA {
        operand: Operand,
        eac: EffectiveAddressCalculation,
    },
    AluBinary {
        op_code: AluBinaryOpCode,
        width: Width,
        destination: Operand,
        source: Operand,
    },
    AluUnary {
        op_code: AluUnaryOpCode,
        width: Width,
        operand: Operand,
    },
    ShiftRotate {
        op_code: ShiftRotateOpCode,
        width: Width,
        operand: Operand,
        amount: Operand,
    },
    CALL(Jump),
    JMP(Jump),
    ConditionalJump {
        op_code: ConditionalJumpOpCode,
        target: i8,
    },
    HLT,
}

impl Display for Instruction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self.data {
            InstructionData::MOV {
                width,
                destination,
                source,
            } => {
                write!(f, "MOV {} {},{}", width, destination, source)
            }
            InstructionData::PUSH { operand } => write!(f, "PUSH {}", operand),
            InstructionData::POP { operand } => write!(f, "POP {}", operand),
            InstructionData::CALL(jump) => write!(f, "CALL {}", jump),
            InstructionData::LEA { operand, eac } => write!(f, "LEA {},[{}]", operand, eac),
            InstructionData::AluBinary {
                op_code,
                width,
                destination,
                source,
            } => {
                write!(f, "{} {} {},{}", op_code, width, destination, source)
            }
            InstructionData::AluUnary {
                op_code,
                width,
                operand,
            } => {
                write!(f, "{} {} {}", op_code, width, operand)
            }
            InstructionData::ShiftRotate {
                op_code,
                width,
                operand,
                amount,
            } => {
                write!(f, "{} {} {},{}", op_code, width, operand, amount)
            }
            InstructionData::JMP(jump) => write!(f, "JMP {}", jump),
            InstructionData::ConditionalJump { op_code, target } => {
                write!(f, "{} ${:+}", op_code, target + 2)
            }
            InstructionData::HLT => write!(f, "HLT"),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AluUnaryOpCode {
    NOT,
    INC,
    DEC,
    NEG,
    MUL,
    IMUL,
    DIV,
    IDIV,
}

impl Display for AluUnaryOpCode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AluBinaryOpCode {
    ADD,
    SUB,
    ADC,
    SBB,
    CMP,
    AND,
    OR,
    XOR,
    TEST,
}

impl Display for AluBinaryOpCode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ShiftRotateOpCode {
    SHL,
    SHR,
    SAR,
    ROL,
    ROR,
    RCL,
    RCR,
}

impl Display for ShiftRotateOpCode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}
