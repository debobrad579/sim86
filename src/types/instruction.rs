use std::fmt::Display;

use crate::types::{
    jump::{ConditionalJumpOpCode, Jump},
    operand::{EffectiveAddressCalculation, Operand, SegmentRegister},
    width::Width,
};

pub struct Instruction {
    pub data: InstructionData,
    pub lock: bool,
    pub size: u16,
}

pub enum InstructionData {
    MOV {
        width: Width,
        destination: Operand,
        source: Operand,
    },
    XCHG {
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
    IN {
        width: Width,
        operand: Operand,
        port: Operand,
    },
    OUT {
        width: Width,
        operand: Operand,
        port: Operand,
    },
    XLAT {
        segment_override: Option<SegmentRegister>,
    },
    LEA {
        operand: Operand,
        eac: EffectiveAddressCalculation,
    },
    LDS {
        operand: Operand,
        eac: EffectiveAddressCalculation,
    },
    LES {
        operand: Operand,
        eac: EffectiveAddressCalculation,
    },
    LAHF,
    SAHF,
    PUSHF,
    POPF,
    CALL(Jump),
    RET(u16),
    RETF(u16),
    JMP(Jump),
    ConditionalJump {
        op_code: ConditionalJumpOpCode,
        target: i8,
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
    AAA,
    DAA,
    AAS,
    DAS,
    AAM,
    AAD,
    CBW,
    CWD,
    CLC,
    CMC,
    STC,
    CLD,
    STD,
    CLI,
    STI,
    HLT,
    WAIT,
    INT(u8),
    INT3,
    INTO,
    IRET,
    String {
        op_code: StringOpCode,
        rep: Option<bool>,
        width: Width,
    },
}

impl Display for Instruction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}{}",
            if self.lock { "LOCK " } else { "" },
            match &self.data {
                InstructionData::MOV {
                    width,
                    destination,
                    source,
                } => format!("MOV {} {},{}", width, destination, source),
                InstructionData::XCHG {
                    width,
                    destination,
                    source,
                } => format!("XCHG {} {},{}", width, destination, source),
                InstructionData::PUSH { operand } => format!("PUSH {}", operand),
                InstructionData::POP { operand } => format!("POP {}", operand),
                InstructionData::IN { operand, port, .. } => format!("IN {},{}", operand, port),
                InstructionData::OUT { operand, port, .. } => format!("OUT {},{}", port, operand),
                InstructionData::CALL(jump) => format!("CALL {}", jump),
                InstructionData::XLAT { segment_override } => match segment_override {
                    Some(segment_override) => format!("{} XLAT", segment_override),
                    None => "XLAT".into(),
                },
                InstructionData::LEA { operand, eac } => format!("LEA {},[{}]", operand, eac),
                InstructionData::LDS { operand, eac } => format!("LDS {},[{}]", operand, eac),
                InstructionData::LES { operand, eac } => format!("LES {},[{}]", operand, eac),
                InstructionData::LAHF => "LAHF".into(),
                InstructionData::SAHF => "SAHF".into(),
                InstructionData::PUSHF => "PUSHF".into(),
                InstructionData::POPF => "POPF".into(),
                InstructionData::RET(immediate) => {
                    if *immediate == 0 {
                        "RET".into()
                    } else {
                        format!("RET {}", immediate)
                    }
                }
                InstructionData::RETF(immediate) => {
                    if *immediate == 0 {
                        "RETF".into()
                    } else {
                        format!("RETF {}", immediate)
                    }
                }
                InstructionData::AluBinary {
                    op_code,
                    width,
                    destination,
                    source,
                } => {
                    format!("{} {} {},{}", op_code, width, destination, source)
                }
                InstructionData::AluUnary {
                    op_code,
                    width,
                    operand,
                } => {
                    format!("{} {} {}", op_code, width, operand)
                }
                InstructionData::ShiftRotate {
                    op_code,
                    width,
                    operand,
                    amount,
                } => {
                    format!("{} {} {},{}", op_code, width, operand, amount)
                }
                InstructionData::JMP(jump) => format!("JMP {}", jump),
                InstructionData::ConditionalJump { op_code, target } => {
                    format!("{} ${:+}", op_code, target + 2)
                }
                InstructionData::AAA => "AAA".into(),
                InstructionData::DAA => "DAA".into(),
                InstructionData::AAS => "AAS".into(),
                InstructionData::DAS => "DAS".into(),
                InstructionData::AAM => "AAM".into(),
                InstructionData::AAD => "AAD".into(),
                InstructionData::CBW => "CBW".into(),
                InstructionData::CWD => "CWD".into(),
                InstructionData::CLC => "CLC".into(),
                InstructionData::CMC => "CMC".into(),
                InstructionData::STC => "STC".into(),
                InstructionData::CLD => "CLD".into(),
                InstructionData::STD => "STD".into(),
                InstructionData::CLI => "CLI".into(),
                InstructionData::STI => "STI".into(),
                InstructionData::HLT => "HLT".into(),
                InstructionData::WAIT => "WAIT".into(),
                InstructionData::INT(data) => format!("INT {}", data),
                InstructionData::INT3 => "INT3".into(),
                InstructionData::INTO => "INTO".into(),
                InstructionData::IRET => "IRET".into(),
                InstructionData::String {
                    op_code,
                    rep,
                    width,
                } => format!(
                    "{}{}{}",
                    match rep {
                        Some(z) =>
                            if *z {
                                "REPZ "
                            } else {
                                "REP "
                            },
                        None => "",
                    },
                    op_code,
                    match width {
                        Width::Byte => "B",
                        Width::Word => "W",
                    }
                ),
            }
        )
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

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum StringOpCode {
    MOVS,
    CMPS,
    SCAS,
    LODS,
    STOS,
}

impl Display for StringOpCode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}
