use std::fmt::Debug;

pub mod exec;

#[derive(Default)]
pub struct Sim86 {
    registers: [u16; 8],
}

impl Debug for Sim86 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "AX:{:04X} | SP:{:04X}\nBX:{:04X} | BP:{:04X}\nCX:{:04X} | SI:{:04X}\nDX:{:04X} | DI:{:04X}",
            self.registers[0],
            self.registers[4],
            self.registers[3],
            self.registers[5],
            self.registers[1],
            self.registers[6],
            self.registers[2],
            self.registers[7]
        )
    }
}
