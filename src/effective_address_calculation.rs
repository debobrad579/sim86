use crate::instruction::{Memory, Register, Width};

pub fn get_effective_address_expression(rm: u8, displacement: i16) -> Memory {
    match rm {
        0b000 => Memory {
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
        0b001 => Memory {
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
        0b010 => Memory {
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
        0b011 => Memory {
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
        0b100 => Memory {
            base: Register {
                width: Width::Word,
                value: 0b110,
            },
            index: None,
            displacement,
        },
        0b101 => Memory {
            base: Register {
                width: Width::Word,
                value: 0b111,
            },
            index: None,
            displacement,
        },
        0b110 => Memory {
            base: Register {
                width: Width::Word,
                value: 0b101,
            },
            index: None,
            displacement,
        },
        0b111 => Memory {
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
