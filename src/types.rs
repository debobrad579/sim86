use std::fmt::Display;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Width {
    Byte,
    Word,
}

impl From<u8> for Width {
    fn from(value: u8) -> Self {
        match value {
            0b0 => Self::Byte,
            0b1 => Self::Word,
            _ => unreachable!(),
        }
    }
}

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
    ALU {
        op_code: ALUOpCode,
        width: Width,
        destination: Operand,
        source: Operand,
    },
    Jump {
        op_code: JumpOpCode,
        target: i8,
    },
}

impl Display for Instruction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self.data {
            InstructionData::MOV {
                width,
                destination,
                source,
            } => {
                let mut size = " ";
                if !matches!(destination, Operand::Register(_))
                    && !matches!(source, Operand::Register(_))
                {
                    match width {
                        Width::Byte => size = " BYTE ",
                        Width::Word => size = " WORD ",
                    }
                }

                write!(f, "MOV{}{},{}", size, destination, source)
            }
            InstructionData::ALU {
                op_code,
                width,
                destination,
                source,
            } => {
                let mut size = " ";
                if !matches!(destination, Operand::Register(_))
                    && !matches!(source, Operand::Register(_))
                {
                    match width {
                        Width::Byte => size = " BYTE ",
                        Width::Word => size = " WORD ",
                    }
                }

                write!(f, "{}{}{},{}", op_code, size, destination, source)
            }
            InstructionData::Jump { op_code, target } => write!(f, "{} ${:+}", op_code, target),
        }
    }
}

pub enum Operand {
    Register(Register),
    Memory(Memory),
    Immediate(Immediate),
    SegmentRegistry(SegmentRegister),
}

impl Display for Operand {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Operand::Register(register) => write!(f, "{}", register),
            Operand::Memory(memory) => write!(f, "{}", memory),
            Operand::Immediate(immediate) => write!(f, "{}", immediate),
            Operand::SegmentRegistry(sr) => write!(f, "{}", sr),
        }
    }
}

pub struct Register {
    pub width: Width,
    pub value: u8,
}

impl Display for Register {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self.width {
            Width::Byte => match self.value {
                0b000 => write!(f, "AL"),
                0b001 => write!(f, "CL"),
                0b010 => write!(f, "DL"),
                0b011 => write!(f, "BL"),
                0b100 => write!(f, "AH"),
                0b101 => write!(f, "CH"),
                0b110 => write!(f, "DH"),
                0b111 => write!(f, "BH"),
                _ => unreachable!(),
            },
            Width::Word => match self.value {
                0b000 => write!(f, "AX"),
                0b001 => write!(f, "CX"),
                0b010 => write!(f, "DX"),
                0b011 => write!(f, "BX"),
                0b100 => write!(f, "SP"),
                0b101 => write!(f, "BP"),
                0b110 => write!(f, "SI"),
                0b111 => write!(f, "DI"),
                _ => unreachable!(),
            },
        }
    }
}

pub struct SegmentRegister(u8);

impl From<u8> for SegmentRegister {
    fn from(value: u8) -> Self {
        Self(value)
    }
}

impl Display for SegmentRegister {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self.0 {
            0b00 => write!(f, "ES"),
            0b01 => write!(f, "CS"),
            0b10 => write!(f, "SS"),
            0b11 => write!(f, "DS"),
            _ => unreachable!(),
        }
    }
}

pub enum Memory {
    EAC(EAC),
    DirectAccess(u16),
}

impl Display for Memory {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Memory::EAC(eac) => write!(f, "[{}]", eac),
            Memory::DirectAccess(address) => write!(f, "[{}]", address),
        }
    }
}

pub struct EAC {
    pub base: Register,
    pub index: Option<Register>,
    pub displacement: i16,
}

impl Display for EAC {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self.index {
            Some(index) => write!(f, "{}+{}{:+}", self.base, index, self.displacement),
            None => write!(f, "{}{:+}", self.base, self.displacement),
        }
    }
}

pub enum Immediate {
    SignExtended(i16),
    Unsigned(u16),
}

impl Display for Immediate {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Immediate::SignExtended(value) => write!(f, "{}", value),
            Immediate::Unsigned(value) => write!(f, "{}", value),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ALUOpCode {
    ADD,
    SUB,
    ADC,
    SBB,
    CMP,
    AND,
    OR,
    XOR,
}

impl Display for ALUOpCode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum JumpOpCode {
    JO,
    JNO,
    JB,
    JNB,
    JE,
    JNE,
    JBE,
    JNBE,
    JS,
    JNS,
    JP,
    JNP,
    JL,
    JNL,
    JLE,
    JNLE,
    LOOPNZ,
    LOOPZ,
    LOOP,
    JCXZ,
}

impl Display for JumpOpCode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}
