use crate::types::instruction::InstructionData;

pub mod alu;
pub mod decoder;
pub mod io;
pub mod jumps;
pub mod load;
pub mod memory;
pub mod misc;
pub mod mov;
pub mod processor;
pub mod shift_rotate;
pub mod stack;
pub mod xchg;

pub struct DecodeInstruction {
    data: InstructionData,
    size: u16,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(super) enum Direction {
    RegToRM,
    RMToReg,
}

impl From<u8> for Direction {
    fn from(value: u8) -> Self {
        match value {
            0b0 => Self::RegToRM,
            0b1 => Self::RMToReg,
            _ => unreachable!(),
        }
    }
}
