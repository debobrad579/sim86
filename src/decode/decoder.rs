use crate::{
    decode::{alu, misc, mov, op_code::get_conditional_jump_op_code},
    types::{Instruction, SegmentRegister},
};

impl From<&[u8]> for Instruction {
    fn from(bytes: &[u8]) -> Self {
        let mut offset = 0;
        let mut segment_override: Option<SegmentRegister> = None;

        loop {
            if bytes[offset] >> 5 == 0b001 && bytes[offset] & 0b00000111 == 0b110 {
                segment_override = Some(SegmentRegister::from((bytes[offset] & 0b00011000) >> 3));
            } else {
                break;
            }

            offset += 1;
        }

        if bytes[offset] >> 2 == 0b100010 {
            mov::reg_to_from_rm(bytes, offset, segment_override)
        } else if bytes[offset] >> 1 == 0b1100011 {
            mov::immediate_to_rm(bytes, offset, segment_override)
        } else if bytes[offset] >> 4 == 0b1011 {
            mov::immediate_to_register(bytes, offset)
        } else if bytes[offset] >> 2 == 0b101000 {
            mov::memory_to_from_accumulator(bytes, offset, segment_override)
        } else if bytes[offset] >> 2 == 0b100011 {
            mov::sr_to_from_rm(bytes, offset, segment_override)
        } else if bytes[offset] & 0b11000100 == 0 {
            alu::rm_with_reg_to_either(bytes, offset, segment_override)
        } else if bytes[offset] >> 1 == 0b1111111 || bytes[offset] >> 1 == 0b1111011 {
            alu::unary_rm(bytes, offset, segment_override)
        } else if bytes[offset] >> 6 == 0b10 && bytes[offset] & 0b00000100 == 0 {
            alu::immediate_from_rm(bytes, offset, segment_override)
        } else if bytes[offset] >> 6 == 0b00 && (bytes[offset] & 0b00000110) >> 1 == 0b10 {
            alu::immediate_from_accumulator(bytes, offset)
        } else if let Some(op_code) = get_conditional_jump_op_code(bytes[offset]) {
            misc::conditional_jump(bytes, offset, op_code)
        } else if bytes[offset] >> 6 == 0b01 {
            alu::unary_reg(bytes, offset)
        } else if bytes[offset] == 0b11110100 {
            misc::halt(offset)
        } else {
            unreachable!("operation not found: {:08b}", bytes[0])
        }
    }
}
