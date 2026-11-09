pub mod alu;
pub mod decoder;
pub mod jumps;
pub mod memory;
pub mod misc;
pub mod mov;
pub mod shift_rotate;
pub mod stack;

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
