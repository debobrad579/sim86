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
    pub op_code: OpCode,
    pub width: Width,
    pub operand1: Operand,
    pub operand2: Operand,
    pub size: usize,
}

impl Display for Instruction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut size = " ";
        if !matches!(self.operand1, Operand::Register(_))
            && !matches!(self.operand2, Operand::Register(_))
        {
            match self.width {
                Width::Byte => size = " BYTE ",
                Width::Word => size = " WORD ",
            }
        }

        write!(
            f,
            "{}{}{},{}",
            self.op_code, size, self.operand1, self.operand2
        )
    }
}

pub enum Operand {
    None,
    JumpTarget(i8),
    Register(Register),
    Memory(Memory),
    Immediate(Immediate),
}

impl Display for Operand {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Operand::Register(register) => write!(f, "{}", register),
            Operand::Memory(memory) => write!(f, "{}", memory),
            Operand::Immediate(immediate) => write!(f, "{}", immediate),
            Operand::JumpTarget(target) => write!(f, "${:+}", target),
            Operand::None => write!(f, ""),
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
pub enum OpCode {
    MOV,
    ADD,
    SUB,
    ADC,
    SBB,
    CMP,
    AND,
    OR,
    XOR,
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

impl Display for OpCode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}
