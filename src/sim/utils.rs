use crate::types::{
    operand::{EffectiveAddressCalculation, Memory, Operand, Register, SegmentRegister},
    width::Width,
};

pub(super) fn get_sp_memory_operand() -> Operand {
    Operand::Memory(Memory {
        address: EffectiveAddressCalculation::Calculation {
            base: Register {
                width: Width::Word,
                value: 0b100,
            },
            index: None,
            displacement: 0,
        },
        segment_override: Some(SegmentRegister::from(0b10)),
    })
}
