use std::fmt::Debug;

pub mod exec;
pub mod flags;
pub mod get_set;
pub mod run;

pub struct Sim86 {
    program: Box<[u8]>,
    registers: [u16; 8],
    instruction_pointer: usize,
    flags: u16,
}

impl Sim86 {
    pub fn new(program: Box<[u8]>) -> Self {
        Self {
            program,
            registers: [0; 8],
            instruction_pointer: 0,
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
CF:{} | PF:{} | AF:{}
ZF:{} | SF:{} | OF:{}
TF:{} | IF:{} | DF:{}",
            self.registers[0],
            self.registers[4],
            self.registers[3],
            self.registers[5],
            self.registers[1],
            self.registers[6],
            self.registers[2],
            self.registers[7],
            self.get_carry_flag(),
            self.get_parity_flag(),
            self.get_auxiliary_carry_flag(),
            self.get_zero_flag(),
            self.get_sign_flag(),
            self.get_overflow_flag(),
            self.get_trap_flag(),
            self.get_interrupt_flag(),
            self.get_direction_flag(),
        )
    }
}
