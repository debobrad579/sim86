use std::fmt::Display;

use crate::types::operand::{Memory, Operand};

pub enum Jump {
    DirectWithinSegment(i16),
    DirectWithinSegmentShort(i8),
    IndirectWithinSegment(Operand),
    DirectIntersegment { ip: u16, cs: u16 },
    IndirectIntersegment(Memory),
}

impl Display for Jump {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Jump::DirectWithinSegment(target) => write!(f, "${:+}", target + 3),
            Jump::DirectWithinSegmentShort(target) => write!(f, "${:+}", target + 2),
            Jump::IndirectWithinSegment(operand) => write!(f, "{}", operand),
            Jump::DirectIntersegment { ip, cs } => write!(f, "{}:{}", cs, ip),
            Jump::IndirectIntersegment(memory) => write!(f, "far {}", memory),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ConditionalJumpOpCode {
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

impl Display for ConditionalJumpOpCode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}
