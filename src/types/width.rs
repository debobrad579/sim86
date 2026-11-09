use std::fmt::Display;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Width {
    Byte,
    Word,
}

impl Width {
    #[inline]
    pub fn mask(self) -> u16 {
        match self {
            Width::Byte => 0x00FF,
            Width::Word => 0xFFFF,
        }
    }

    #[inline]
    pub fn sign_bit(self) -> u16 {
        match self {
            Width::Byte => 0x0080,
            Width::Word => 0x8000,
        }
    }
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

impl Display for Width {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Width::Byte => write!(f, "BYTE"),
            Width::Word => write!(f, "WORD"),
        }
    }
}
