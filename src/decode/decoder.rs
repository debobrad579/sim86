use crate::{
    decode::{alu, io, jumps, load, misc, mov, processor, shift_rotate, stack, xchg},
    types::{
        instruction::{AluBinaryOpCode, Instruction},
        operand::SegmentRegister,
    },
};

impl From<&[u8]> for Instruction {
    fn from(bytes: &[u8]) -> Self {
        let mut offset = 0;
        let mut segment_override: Option<SegmentRegister> = None;
        let mut lock = false;

        loop {
            if bytes[offset] == 0b11110000 {
                lock = true;
            } else if bytes[offset] >> 5 == 0b001 && bytes[offset] & 0b00000111 == 0b110 {
                segment_override = Some(SegmentRegister::from((bytes[offset] & 0b00011000) >> 3));
            } else {
                break;
            }

            offset += 1;
        }

        let bytes = &bytes[offset..];

        let instruction = match bytes[0] {
            0x88..=0x8B => mov::reg_to_from_rm(bytes, segment_override),
            0x8C | 0x8E => mov::sr_to_from_rm(bytes, segment_override),
            0xC6 | 0xC7 => mov::immediate_to_rm(bytes, segment_override),
            0xB0..=0xBF => mov::immediate_to_register(bytes),
            0xA0..=0xA3 => mov::memory_to_from_accumulator(bytes, segment_override),
            0x86 | 0x87 => xchg::rm_with_register(bytes, segment_override),
            0x90..=0x97 => xchg::register_with_accumulator(bytes),
            0xD7 => misc::xlat(segment_override),
            0x8D => load::effective_address(bytes),
            0xC5 => load::pointer_to_ds(bytes),
            0xC4 => load::pointer_to_es(bytes),
            0x9F => load::ah_with_flags(),
            0x9E => load::ah_into_flags(),
            0xFF if (bytes[1] >> 3) & 0b111 == 0b110 => stack::push_rm(bytes, segment_override),
            0x50..=0x57 => stack::push_reg(bytes),
            0x06 | 0x0E | 0x16 | 0x1E => stack::push_sr(bytes),
            0x8F if (bytes[1] >> 3) & 0b111 == 0b000 => stack::pop_rm(bytes, segment_override),
            0x58..=0x5F => stack::pop_reg(bytes),
            0x07 | 0x0F | 0x17 | 0x1F => stack::pop_sr(bytes),
            0x9C => stack::pushf(),
            0x9D => stack::popf(),
            0xE8 => stack::call_direct_within_segment(bytes),
            0xFF if (bytes[1] >> 3) & 0b111 == 0b010 => {
                stack::call_indirect_within_segment(bytes, segment_override)
            }
            0x9A => stack::call_direct_intersegment(bytes),
            0xFF if (bytes[1] >> 3) & 0b111 == 0b011 => {
                stack::call_indirect_intersegment(bytes, segment_override)
            }
            0xC3 => stack::ret(),
            0xC2 => stack::ret_adding_immediate_to_sp(bytes),
            0xCB => stack::retf(),
            0xCA => stack::retf_adding_immediate_to_sp(bytes),
            0xE9 => jumps::direct_within_segment(bytes),
            0xEB => jumps::direct_within_segment_short(bytes),
            0xFF if (bytes[1] >> 3) & 0b111 == 0b100 => {
                jumps::indirect_within_segment(bytes, segment_override)
            }
            0xEA => jumps::direct_intersegment(bytes),
            0xFF if (bytes[1] >> 3) & 0b111 == 0b101 => {
                jumps::indirect_intersegment(bytes, segment_override)
            }
            0x80..=0x83 => alu::immediate_to_rm(bytes, segment_override, None),
            0x84 | 0x85 => {
                alu::rm_with_reg_to_either(bytes, segment_override, Some(AluBinaryOpCode::TEST))
            }
            0xA8 | 0xA9 => alu::immediate_to_accumulator(bytes, Some(AluBinaryOpCode::TEST)),
            0xF6 | 0xF7 if (bytes[1] >> 3) & 0b111 == 0b000 => {
                alu::immediate_to_rm(bytes, segment_override, Some(AluBinaryOpCode::TEST))
            }
            0xF6 | 0xF7 | 0xFE | 0xFF => alu::unary_rm(bytes, segment_override),
            0x40..=0x5F => alu::unary_reg(bytes),
            0xD0..=0xD3 => shift_rotate::shift_rotate_rm(bytes, segment_override),
            0x70..=0x7F | 0xE0..=0xE3 => jumps::conditional_jump(bytes),
            0xE4 | 0xE5 => io::in_fixed(bytes),
            0xEC | 0xED => io::in_variable(bytes),
            0xE6 | 0xE7 => io::out_fixed(bytes),
            0xEE | 0xEF => io::out_variable(bytes),
            0x37 => misc::aaa(),
            0x27 => misc::daa(),
            0x3F => misc::aas(),
            0x2F => misc::das(),
            0xD4 if bytes[1] == 0x0A => misc::aam(),
            0xD5 if bytes[1] == 0x0A => misc::aad(),
            0x98 => misc::cbw(),
            0x99 => misc::cwd(),
            0xF8 => processor::clear_carry(),
            0xF5 => processor::complement_carry(),
            0xF9 => processor::set_carry(),
            0xFC => processor::clear_direction(),
            0xFD => processor::set_direction(),
            0xFA => processor::clear_interrupt(),
            0xFB => processor::set_interrupt(),
            0xF4 => processor::halt(),
            0x9B => processor::wait(),
            byte if byte & 0b11000100 == 0 => {
                alu::rm_with_reg_to_either(bytes, segment_override, None)
            }
            byte if byte & 0b11000110 == 0b00000100 => alu::immediate_to_accumulator(bytes, None),
            byte => unimplemented!("operation not found: {:08b}", byte),
        };

        Self {
            data: instruction.data,
            lock,
            size: offset as u16 + instruction.size,
        }
    }
}
