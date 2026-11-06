use crate::{
    sim::Sim86,
    types::{ALUOpCode, Operand, Width},
};

struct ALUFlags {
    pub carry: Option<bool>,
    pub auxiliary_carry: Option<bool>,
    pub overflow: Option<bool>,
}

fn add(a: u16, b: u16, width: Width) -> (u16, ALUFlags) {
    let mask = width.mask();
    let a = a & mask;
    let b = b & mask;

    let wide = u32::from(a) + u32::from(b);
    let result = (wide as u16) & mask;

    (
        result,
        ALUFlags {
            carry: Some(wide > u32::from(mask)),
            auxiliary_carry: Some(((a ^ b ^ result) & 0x10) != 0),
            overflow: Some((!(a ^ b) & (a ^ result) & width.sign_bit()) != 0),
        },
    )
}

fn sub(a: u16, b: u16, width: Width) -> (u16, ALUFlags) {
    let mask = width.mask();
    let a = a & mask;
    let b = b & mask;

    let result = a.wrapping_sub(b) & mask;

    (
        result,
        ALUFlags {
            carry: Some(a < b),
            auxiliary_carry: Some(((a ^ b ^ result) & 0x10) != 0),
            overflow: Some(((a ^ b) & (a ^ result) & width.sign_bit()) != 0),
        },
    )
}

fn and(a: u16, b: u16) -> (u16, ALUFlags) {
    (
        a & b,
        ALUFlags {
            carry: Some(false),
            auxiliary_carry: None,
            overflow: Some(false),
        },
    )
}

fn or(a: u16, b: u16) -> (u16, ALUFlags) {
    (
        a | b,
        ALUFlags {
            carry: Some(false),
            auxiliary_carry: None,
            overflow: Some(false),
        },
    )
}

fn xor(a: u16, b: u16) -> (u16, ALUFlags) {
    (
        a ^ b,
        ALUFlags {
            carry: Some(false),
            auxiliary_carry: None,
            overflow: Some(false),
        },
    )
}

impl Sim86 {
    pub(super) fn exec_alu_instruction(
        &mut self,
        op_code: ALUOpCode,
        destination: Operand,
        source: Operand,
        width: Width,
    ) {
        let (a, b) = (
            self.get_value(&destination, &width),
            self.get_value(&source, &width),
        );

        let (result, flags, write_result) = match op_code {
            ALUOpCode::ADD => {
                let (result, flags) = add(a, b, width);
                (result, flags, true)
            }
            ALUOpCode::SUB => {
                let (result, flags) = sub(a, b, width);
                (result, flags, true)
            }
            ALUOpCode::CMP => {
                let (result, flags) = sub(a, b, width);
                (result, flags, false)
            }
            ALUOpCode::ADC => todo!(),
            ALUOpCode::SBB => todo!(),
            ALUOpCode::AND => {
                let (result, flags) = and(a, b);
                (result, flags, true)
            }
            ALUOpCode::OR => {
                let (result, flags) = or(a, b);
                (result, flags, true)
            }
            ALUOpCode::XOR => {
                let (result, flags) = xor(a, b);
                (result, flags, true)
            }
            ALUOpCode::TEST => {
                let (result, flags) = and(a, b);
                (result, flags, false)
            }
            ALUOpCode::SHR => todo!(),
            ALUOpCode::SAR => todo!(),
            ALUOpCode::SHL => todo!(),
            ALUOpCode::MUL => todo!(),
            ALUOpCode::IMUL => todo!(),
            ALUOpCode::DIV => todo!(),
            ALUOpCode::IDIV => todo!(),
            ALUOpCode::INC => todo!(),
            ALUOpCode::DEC => todo!(),
        };

        self.set_zero_flag(result == 0);
        self.set_sign_flag(result & width.sign_bit() != 0);
        self.set_parity_flag((result & 0x00FF).count_ones().is_multiple_of(2));

        if let Some(carry) = flags.carry {
            self.set_carry_flag(carry);
        }
        if let Some(auxiliary_carry) = flags.auxiliary_carry {
            self.set_auxiliary_carry_flag(auxiliary_carry);
        }
        if let Some(overflow) = flags.overflow {
            self.set_overflow_flag(overflow);
        }

        if write_result {
            self.set_value(destination, result, &width);
        }
    }
}
