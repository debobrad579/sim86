use std::fmt::Display;

use crate::types::width::Width;

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

impl SegmentRegister {
    pub fn value(&self) -> u8 {
        self.0
    }
}

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

pub struct Memory {
    pub address: EffectiveAddressCalculation,
    pub segment_override: Option<SegmentRegister>,
}

impl Display for Memory {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self.segment_override {
            Some(segment_override) => write!(f, "{}:[{}]", segment_override, self.address),
            None => write!(f, "[{}]", self.address),
        }
    }
}

pub enum EffectiveAddressCalculation {
    Calculation {
        base: Register,
        index: Option<Register>,
        displacement: i16,
    },
    DirectAccess(u16),
}

impl Display for EffectiveAddressCalculation {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            EffectiveAddressCalculation::Calculation {
                base,
                index,
                displacement,
            } => match index {
                Some(index) => write!(f, "{}+{}{:+}", base, index, displacement),
                None => write!(f, "{}{:+}", base, displacement),
            },
            EffectiveAddressCalculation::DirectAccess(address) => write!(f, "{}", address),
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
