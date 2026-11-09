use std::fmt::Display;

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
