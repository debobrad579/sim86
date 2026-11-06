use crate::{
    sim::Sim86,
    types::{Operand, ShiftRotateOpCode, Width},
};

impl Sim86 {
    pub(super) fn exec_shift_rotate_instruction(
        &mut self,
        op_code: ShiftRotateOpCode,
        operand: Operand,
        amount: Operand,
        width: Width,
    ) {
        let (_, _) = (
            self.get_value(&operand, &width),
            self.get_value(&amount, &width),
        );

        match op_code {
            ShiftRotateOpCode::SHL => todo!(),
            ShiftRotateOpCode::SHR => todo!(),
            ShiftRotateOpCode::SAR => todo!(),
            ShiftRotateOpCode::ROL => todo!(),
            ShiftRotateOpCode::ROR => todo!(),
            ShiftRotateOpCode::RCL => todo!(),
            ShiftRotateOpCode::RCR => todo!(),
        }
    }
}
