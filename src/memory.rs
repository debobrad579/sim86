use crate::instruction::{EAC, Immediate, Memory, Operand, Register, Width};

pub struct ModRM {
    pub reg: Operand,
    pub rm: Operand,
    pub width: Width,
    pub size: usize,
}

pub fn decode_modrm(bytes: [u8; 6], immediate: bool, sign_extend: bool) -> ModRM {
    let width = Width::from(bytes[0] & 0b00000001);
    let mode = bytes[1] >> 6;
    let reg = (bytes[1] & 0b00111000) >> 3;
    let rm = bytes[1] & 0b00000111;

    let (rm_value, displacement_size): (Operand, usize) = match mode {
        0b00 => {
            if rm == 0b110 {
                (
                    Operand::Memory(Memory::DirectAccess(u16::from_le_bytes([
                        bytes[2], bytes[3],
                    ]))),
                    2,
                )
            } else {
                (Operand::Memory(get_effective_address_expression(rm, 0)), 0)
            }
        }
        0b01 => (
            Operand::Memory(get_effective_address_expression(rm, bytes[2] as i8 as i16)),
            1,
        ),
        0b10 => (
            Operand::Memory(get_effective_address_expression(
                rm,
                u16::from_le_bytes([bytes[2], bytes[3]]) as i16,
            )),
            2,
        ),
        0b11 => (Operand::Register(Register { width, value: rm }), 0),
        _ => unreachable!(),
    };

    if immediate {
        let immediate_size = if sign_extend {
            1
        } else {
            match width {
                Width::Byte => 1,
                Width::Word => 2,
            }
        };

        let offset = 2 + displacement_size;

        let raw_value = match immediate_size {
            1 => bytes[offset] as u16,
            2 => u16::from_le_bytes([bytes[offset], bytes[offset + 1]]),
            _ => unreachable!(),
        };

        let immediate = if sign_extend {
            Operand::Immediate(Immediate::SignExtended(raw_value as u8 as i8 as i16))
        } else {
            Operand::Immediate(Immediate::Unsigned(raw_value))
        };

        return ModRM {
            rm: rm_value,
            reg: immediate,
            width,
            size: offset + immediate_size,
        };
    }

    ModRM {
        rm: rm_value,
        reg: Operand::Register(Register { width, value: reg }),
        width,
        size: displacement_size + 2,
    }
}

fn get_effective_address_expression(rm: u8, displacement: i16) -> Memory {
    Memory::EAC(match rm {
        0b000 => EAC {
            base: Register {
                width: Width::Word,
                value: 0b011,
            },
            index: Some(Register {
                width: Width::Word,
                value: 0b110,
            }),
            displacement,
        },
        0b001 => EAC {
            base: Register {
                width: Width::Word,
                value: 0b011,
            },
            index: Some(Register {
                width: Width::Word,
                value: 0b111,
            }),
            displacement,
        },
        0b010 => EAC {
            base: Register {
                width: Width::Word,
                value: 0b101,
            },
            index: Some(Register {
                width: Width::Word,
                value: 0b110,
            }),
            displacement,
        },
        0b011 => EAC {
            base: Register {
                width: Width::Word,
                value: 0b101,
            },
            index: Some(Register {
                width: Width::Word,
                value: 0b111,
            }),
            displacement,
        },
        0b100 => EAC {
            base: Register {
                width: Width::Word,
                value: 0b110,
            },
            index: None,
            displacement,
        },
        0b101 => EAC {
            base: Register {
                width: Width::Word,
                value: 0b111,
            },
            index: None,
            displacement,
        },
        0b110 => EAC {
            base: Register {
                width: Width::Word,
                value: 0b101,
            },
            index: None,
            displacement,
        },
        0b111 => EAC {
            base: Register {
                width: Width::Word,
                value: 0b011,
            },
            index: None,
            displacement,
        },
        _ => unreachable!(),
    })
}
