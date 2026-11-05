use std::fmt::Debug;

pub mod exec;
pub mod flags;
pub mod get_set;
pub mod run;

pub struct Sim86 {
    program: Box<[u8]>,
    _memory: [u8; 1024 * 1024],
    registers: [u16; 8],
    _segment_registers: [u16; 4],
    ip: usize,
    flags: u16,
}

impl Sim86 {
    pub fn new(program: Box<[u8]>) -> Self {
        Self {
            program,
            _memory: [0; 1024 * 1024],
            registers: [0; 8],
            _segment_registers: [0; 4],
            ip: 0,
            flags: 0,
        }
    }
}

impl Debug for Sim86 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "AX:{:04X} | SP:{:04X}
BX:{:04X} | BP:{:04X}
CX:{:04X} | SI:{:04X}
DX:{:04X} | DI:{:04X}
IP:{:04X}
Flags:{}{}{}{}{}{}{}{}{}",
            self.registers[0],
            self.registers[4],
            self.registers[3],
            self.registers[5],
            self.registers[1],
            self.registers[6],
            self.registers[2],
            self.registers[7],
            self.ip,
            if self.get_carry_flag() { "C" } else { "" },
            if self.get_parity_flag() { "P" } else { "" },
            if self.get_auxiliary_carry_flag() {
                "A"
            } else {
                ""
            },
            if self.get_zero_flag() { "Z" } else { "" },
            if self.get_sign_flag() { "S" } else { "" },
            if self.get_trap_flag() { "T" } else { "" },
            if self.get_interrupt_flag() { "I" } else { "" },
            if self.get_direction_flag() { "D" } else { "" },
            if self.get_overflow_flag() { "O" } else { "" },
        )
    }
}
