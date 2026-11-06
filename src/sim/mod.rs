use std::fmt::Debug;

pub mod alu;
pub mod exec;
pub mod flags;
pub mod get_set;
pub mod jump;
pub mod run;

pub struct Sim86 {
    memory: [u8; 1024 * 1024],
    registers: [u16; 8],
    segment_registers: [u16; 4],
    ip: usize,
    flags: u16,
}

impl Sim86 {
    pub fn new(program: &[u8]) -> Self {
        let mut memory = [0; 1024 * 1024];
        memory[..program.len()].copy_from_slice(program);

        Self {
            memory,
            registers: [0; 8],
            segment_registers: [0; 4],
            ip: 0,
            flags: 0,
        }
    }
}

impl Debug for Sim86 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "AX:{:04X} | SP:{:04X} | CS:{:04X}
BX:{:04X} | BP:{:04X} | DS:{:04X}
CX:{:04X} | SI:{:04X} | SS:{:04X}
DX:{:04X} | DI:{:04X} | ES:{:04X}
IP:{:04X}
Flags:{}{}{}{}{}{}{}{}{}",
            self.registers[0],
            self.registers[4],
            self.segment_registers[1],
            self.registers[3],
            self.registers[5],
            self.segment_registers[3],
            self.registers[1],
            self.registers[6],
            self.segment_registers[2],
            self.registers[2],
            self.registers[7],
            self.segment_registers[0],
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
