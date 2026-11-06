use crate::sim::Sim86;

macro_rules! set_bit {
    ($x:expr, $bit:expr, $value:expr) => {
        $x = ($x & !(1 << $bit)) | (($value as u16) << $bit)
    };
}

impl Sim86 {
    pub(super) fn get_carry_flag(&self) -> bool {
        self.flags & 0b1 != 0
    }

    pub(super) fn set_carry_flag(&mut self, value: bool) {
        set_bit!(self.flags, 0, value)
    }

    pub(super) fn get_parity_flag(&self) -> bool {
        self.flags & 0b100 != 0
    }

    pub(super) fn set_parity_flag(&mut self, value: bool) {
        set_bit!(self.flags, 2, value)
    }

    pub(super) fn get_auxiliary_carry_flag(&self) -> bool {
        self.flags & 0b1_0000 != 0
    }

    pub(super) fn set_auxiliary_carry_flag(&mut self, value: bool) {
        set_bit!(self.flags, 4, value)
    }

    pub(super) fn get_zero_flag(&self) -> bool {
        self.flags & 0b100_0000 != 0
    }

    pub(super) fn set_zero_flag(&mut self, value: bool) {
        set_bit!(self.flags, 6, value)
    }

    pub(super) fn get_sign_flag(&self) -> bool {
        self.flags & 0b1000_0000 != 0
    }

    pub(super) fn set_sign_flag(&mut self, value: bool) {
        set_bit!(self.flags, 7, value)
    }

    pub(super) fn get_trap_flag(&self) -> bool {
        self.flags & 0b1_0000_0000 != 0
    }

    pub(super) fn _set_trap_flag(&mut self, value: bool) {
        set_bit!(self.flags, 8, value)
    }

    pub(super) fn get_interrupt_flag(&self) -> bool {
        self.flags & 0b10_0000_0000 != 0
    }

    pub(super) fn _set_interrupt_flag(&mut self, value: bool) {
        set_bit!(self.flags, 9, value)
    }

    pub(super) fn get_direction_flag(&self) -> bool {
        self.flags & 0b100_0000_0000 != 0
    }

    pub(super) fn _set_direction_flag(&mut self, value: bool) {
        set_bit!(self.flags, 10, value)
    }

    pub(super) fn get_overflow_flag(&self) -> bool {
        self.flags & 0b1000_0000_0000 != 0
    }

    pub(super) fn set_overflow_flag(&mut self, value: bool) {
        set_bit!(self.flags, 11, value)
    }
}
