use crate::types::{EAC, Immediate, Memory, Operand, Register, SegmentRegister, Width};

pub struct ModRM {
    pub reg: Operand,
    pub rm: Operand,
    pub width: Width,
    pub size: usize,
}

pub enum ModRMType {
    Register,
    Immediate(bool),
    SegmentRegister,
}

pub fn decode_modrm(
    bytes: &[u8],
    operand_type: ModRMType,
    segment_prefix: Option<SegmentRegister>,
    prefix_offset: usize,
) -> ModRM {
    let width = if let ModRMType::SegmentRegister = operand_type {
        Width::from(1)
    } else {
        Width::from(bytes[prefix_offset] & 0b00000001)
    };

    let mode = bytes[prefix_offset + 1] >> 6;
    let rm = bytes[prefix_offset + 1] & 0b00000111;

    let (rm_value, displacement_size): (Operand, usize) = match mode {
        0b00 => {
            if rm == 0b110 {
                (
                    Operand::Memory(Memory {
                        address: EAC::DirectAccess(u16::from_le_bytes([
                            bytes[prefix_offset + 2],
                            bytes[prefix_offset + 3],
                        ])),
                        segment_prefix,
                    }),
                    2,
                )
            } else {
                (
                    Operand::Memory(Memory {
                        address: get_effective_address_expression(rm, 0),
                        segment_prefix,
                    }),
                    0,
                )
            }
        }
        0b01 => (
            Operand::Memory(Memory {
                address: get_effective_address_expression(
                    rm,
                    bytes[prefix_offset + 2] as i8 as i16,
                ),
                segment_prefix,
            }),
            1,
        ),
        0b10 => (
            Operand::Memory(Memory {
                address: get_effective_address_expression(
                    rm,
                    u16::from_le_bytes([bytes[prefix_offset + 2], bytes[prefix_offset + 3]]) as i16,
                ),
                segment_prefix,
            }),
            2,
        ),
        0b11 => (Operand::Register(Register { width, value: rm }), 0),
        _ => unreachable!(),
    };

    return match operand_type {
        ModRMType::Register => ModRM {
            rm: rm_value,
            reg: Operand::Register(Register {
                width,
                value: (bytes[prefix_offset + 1] & 0b00111000) >> 3,
            }),
            width,
            size: prefix_offset + displacement_size + 2,
        },
        ModRMType::Immediate(sign_extend) => {
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
                1 => bytes[prefix_offset + offset] as u16,
                2 => u16::from_le_bytes([
                    bytes[prefix_offset + offset],
                    bytes[prefix_offset + offset + 1],
                ]),
                _ => unreachable!(),
            };

            let immediate = if sign_extend {
                Operand::Immediate(Immediate::SignExtended(raw_value as u8 as i8 as i16))
            } else {
                Operand::Immediate(Immediate::Unsigned(raw_value))
            };

            ModRM {
                rm: rm_value,
                reg: immediate,
                width,
                size: prefix_offset + offset + immediate_size,
            }
        }
        ModRMType::SegmentRegister => ModRM {
            rm: rm_value,
            reg: Operand::SegmentRegistry(SegmentRegister::from((bytes[1] & 0b00011000) >> 3)),
            width,
            size: prefix_offset + displacement_size + 2,
        },
    };
}

fn get_effective_address_expression(rm: u8, displacement: i16) -> EAC {
    match rm {
        0b000 => EAC::Calculation {
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
        0b001 => EAC::Calculation {
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
        0b010 => EAC::Calculation {
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
        0b011 => EAC::Calculation {
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
        0b100 => EAC::Calculation {
            base: Register {
                width: Width::Word,
                value: 0b110,
            },
            index: None,
            displacement,
        },
        0b101 => EAC::Calculation {
            base: Register {
                width: Width::Word,
                value: 0b111,
            },
            index: None,
            displacement,
        },
        0b110 => EAC::Calculation {
            base: Register {
                width: Width::Word,
                value: 0b101,
            },
            index: None,
            displacement,
        },
        0b111 => EAC::Calculation {
            base: Register {
                width: Width::Word,
                value: 0b011,
            },
            index: None,
            displacement,
        },
        _ => unreachable!(),
    }
}
