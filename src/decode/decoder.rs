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

        let bytes = &bytes[offset..];

        let instruction = if bytes[0] >> 2 == 0b100010 {
            mov::reg_to_from_rm(bytes, segment_override)
        } else if bytes[0] >> 1 == 0b1100011 {
            mov::immediate_to_rm(bytes, segment_override)
        } else if bytes[0] >> 4 == 0b1011 {
            mov::immediate_to_register(bytes)
        } else if bytes[0] >> 2 == 0b101000 {
            mov::memory_to_from_accumulator(bytes, segment_override)
        } else if bytes[0] >> 2 == 0b100011 {
            mov::sr_to_from_rm(bytes, segment_override)
        } else if bytes[0] & 0b11000100 == 0 {
            alu::rm_with_reg_to_either(bytes, segment_override)
        } else if bytes[0] >> 1 == 0b1111111 || bytes[0] >> 1 == 0b1111011 {
            alu::unary_rm(bytes, segment_override)
        } else if bytes[0] >> 6 == 0b10 && bytes[0] & 0b00000100 == 0 {
            alu::immediate_to_rm(bytes, segment_override)
        } else if bytes[0] >> 6 == 0b00 && (bytes[0] & 0b00000110) >> 1 == 0b10 {
            alu::immediate_to_accumulator(bytes)
        } else if let Some(op_code) = get_conditional_jump_op_code(bytes[0]) {
            misc::conditional_jump(bytes, op_code)
        } else if bytes[0] >> 6 == 0b01 {
            alu::unary_reg(bytes)
        } else if bytes[0] == 0b11110100 {
            misc::halt()
        } else {
            unimplemented!("operation not found: {:08b}", bytes[0])
        };

        Self {
            data: instruction.data,
            size: offset + instruction.size,
        }
    }
}
