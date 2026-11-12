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

pub(super) fn get_default_segment(eac: &EffectiveAddressCalculation) -> SegmentRegister {
    SegmentRegister::from(
        if let EffectiveAddressCalculation::Calculation { base, .. } = eac
            && base.value == 0b101
        {
            0b10
        } else {
            0b11
        },
    )
}

pub(super) fn get_memory_address(segment: u16, effective_address: u16) -> usize {
    (((segment as usize) << 4) + effective_address as usize) & 0xFFFFF
}
