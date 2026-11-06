use crate::{
    sim::Sim86,
    types::{AluBinaryOpCode, Operand, Width},
};

struct AluFlags {
    pub carry: Option<bool>,
    pub auxiliary_carry: Option<bool>,
    pub overflow: Option<bool>,
}

fn add(a: u16, b: u16, carry_in: bool, width: Width) -> (u16, AluFlags) {
    let mask = width.mask();
    let a = a & mask;
    let b = b & mask;
    let carry_in = u16::from(carry_in);

    let wide = u32::from(a) + u32::from(b) + u32::from(carry_in);
    let result = (wide as u16) & mask;

    (
        result,
        AluFlags {
            carry: Some(wide > u32::from(mask)),
            auxiliary_carry: Some(((a & 0x0F) + (b & 0x0F) + carry_in) > 0x0F),
            overflow: Some((!(a ^ b) & (a ^ result) & width.sign_bit()) != 0),
        },
    )
}

fn sub(a: u16, b: u16, borrow_in: bool, width: Width) -> (u16, AluFlags) {
    let mask = width.mask();
    let a = a & mask;
    let b = b & mask;
    let borrow_in = u16::from(borrow_in);

    let result = a.wrapping_sub(b).wrapping_sub(borrow_in) & mask;

    (
        result,
        AluFlags {
            carry: Some(u32::from(a) < u32::from(b) + u32::from(borrow_in)),
            auxiliary_carry: Some((a & 0x0F) < (b & 0x0F) + borrow_in),
            overflow: Some(((a ^ b) & (a ^ result) & width.sign_bit()) != 0),
        },
    )
}

fn and(a: u16, b: u16) -> (u16, AluFlags) {
    (
        a & b,
        AluFlags {
            carry: Some(false),
            auxiliary_carry: None,
            overflow: Some(false),
        },
    )
}

fn or(a: u16, b: u16) -> (u16, AluFlags) {
    (
        a | b,
        AluFlags {
            carry: Some(false),
            auxiliary_carry: None,
            overflow: Some(false),
        },
    )
}

fn xor(a: u16, b: u16) -> (u16, AluFlags) {
    (
        a ^ b,
        AluFlags {
            carry: Some(false),
            auxiliary_carry: None,
            overflow: Some(false),
        },
    )
}

impl Sim86 {
    pub(super) fn exec_alu_binary_instruction(
        &mut self,
        op_code: AluBinaryOpCode,
        destination: Operand,
        source: Operand,
        width: Width,
    ) {
        let (a, b) = (
            self.get_value(&destination, &width),
            self.get_value(&source, &width),
        );

        let (result, flags, write_result) = match op_code {
            AluBinaryOpCode::ADD => {
                let (result, flags) = add(a, b, false, width);
                (result, flags, true)
            }
            AluBinaryOpCode::ADC => {
                let (result, flags) = add(a, b, true, width);
                (result, flags, true)
            }
            AluBinaryOpCode::SUB => {
                let (result, flags) = sub(a, b, false, width);
                (result, flags, true)
            }
            AluBinaryOpCode::SBB => {
                let (result, flags) = sub(a, b, true, width);
                (result, flags, true)
            }
            AluBinaryOpCode::CMP => {
                let (result, flags) = sub(a, b, false, width);
                (result, flags, false)
            }
            AluBinaryOpCode::AND => {
                let (result, flags) = and(a, b);
                (result, flags, true)
            }
            AluBinaryOpCode::OR => {
                let (result, flags) = or(a, b);
                (result, flags, true)
            }
            AluBinaryOpCode::XOR => {
                let (result, flags) = xor(a, b);
                (result, flags, true)
            }
            AluBinaryOpCode::TEST => {
                let (result, flags) = and(a, b);
                (result, flags, false)
            }
            AluBinaryOpCode::SHR => todo!(),
            AluBinaryOpCode::SAR => todo!(),
            AluBinaryOpCode::SHL => todo!(),
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
