use crate::instruction::{EAC, Memory, Operand, Register, Width};

pub struct ModRM {
    pub reg: Operand,
    pub rm: Operand,
    pub width: Width,
    pub size: usize,
}

pub fn decode_modrm(bytes: [u8; 6], immediate: bool) -> ModRM {
    let width = Width::from(bytes[0] & 0b00000001);
    let mode = bytes[1] >> 6;
    let reg = (bytes[1] & 0b00111000) >> 3;
    let rm = bytes[1] & 0b00000111;

    let (displacement, displacement_size): (i16, usize) = match mode {
        0b00 => {
            if rm == 0b110 {
                (u16::from_le_bytes([bytes[2], bytes[3]]) as i16, 2)
            } else {
                (0, 0)
            }
        }
        0b01 => (bytes[2] as i8 as i16, 1),
        0b10 => (u16::from_le_bytes([bytes[2], bytes[3]]) as i16, 2),
        0b11 => {
            return ModRM {
                reg: Operand::Register(Register { width, value: reg }),
                rm: Operand::Register(Register { width, value: rm }),
                width,
                size: 2,
            };
        }
        _ => unreachable!(),
    };

    let rm_value = if mode == 0b00 && rm == 0b110 {
        Operand::Memory(Memory::DirectAccess(displacement as u16))
    } else {
        Operand::Memory(get_effective_address_expression(rm, displacement))
    };

    if immediate {
        return match width {
            Width::Byte => ModRM {
                rm: rm_value,
                reg: Operand::Immediate(bytes[2 + displacement_size] as u16),
                width,
                size: displacement_size + 3,
            },
            Width::Word => ModRM {
                rm: rm_value,
                reg: Operand::Immediate(u16::from_le_bytes([
                    bytes[2 + displacement_size],
                    bytes[3 + displacement_size],
                ])),
                width,
                size: displacement_size + 4,
            },
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
