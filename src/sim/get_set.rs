use crate::{
    sim::Sim86,
    types::{EffectiveAddressCalculation, Immediate, Memory, Operand, Register, Width},
};

impl Sim86 {
    pub(super) fn get_value(&self, operand: &Operand, width: &Width) -> u16 {
        match operand {
            Operand::Register(register) => self.get_register(register),
            Operand::Memory(memory) => {
                let memory_address = self.get_memory_address(memory);
                match width {
                    Width::Byte => self.memory[memory_address] as u16,
                    Width::Word => u16::from_le_bytes([
                        self.memory[memory_address],
                        self.memory[memory_address + 1],
                    ]),
                }
            }
            Operand::Immediate(immediate) => match immediate {
                Immediate::SignExtended(value) => *value as u16,
                Immediate::Unsigned(value) => *value,
            },
            Operand::SegmentRegistry(sr) => self.segment_registers[sr.value() as usize],
        }
    }

    pub(super) fn set_value(&mut self, operand: Operand, value: u16, width: &Width) {
        match operand {
            Operand::Register(register) => self.set_register(register, value),
            Operand::Memory(memory) => {
                let memory_address = self.get_memory_address(&memory);
                match width {
                    Width::Byte => self.memory[memory_address] = value as u8,
                    Width::Word => {
                        self.memory[memory_address] = value as u8;
                        self.memory[memory_address + 1] = ((value & 0xFF00) >> 8) as u8;
                    }
                }
            }
            Operand::SegmentRegistry(sr) => self.segment_registers[sr.value() as usize] = value,
            Operand::Immediate(_) => unreachable!(),
        }
    }

    pub(super) fn get_effective_address(&self, eac: &EffectiveAddressCalculation) -> u16 {
        match eac {
            EffectiveAddressCalculation::Calculation {
                base,
                index,
                displacement,
            } => {
                let index_value = index
                    .as_ref()
                    .map(|index| self.get_register(index))
                    .unwrap_or(0);

                (self.get_register(base))
                    .wrapping_add(index_value)
                    .wrapping_add(*displacement as u16)
            }
            EffectiveAddressCalculation::DirectAccess(address) => *address,
        }
    }

    pub(super) fn get_memory_address(&self, memory: &Memory) -> usize {
        let sr_value = self.segment_registers[match &memory.segment_override {
            Some(segment_prefix) => segment_prefix.value(),
            None => {
                if let EffectiveAddressCalculation::Calculation { ref base, .. } = memory.address
                    && base.value == 0b101
                {
                    0b10
                } else {
                    0b11
                }
            }
        } as usize] as usize;

        (sr_value << 4) + self.get_effective_address(&memory.address) as usize
    }

    pub(super) fn get_register(&self, register: &Register) -> u16 {
        match register.width {
            Width::Byte => {
                if register.value < 4 {
                    self.registers[register.value as usize] & 0x00FF
                } else {
                    self.registers[register.value as usize - 4] >> 8
                }
            }
            Width::Word => self.registers[register.value as usize],
        }
    }

    pub(super) fn set_register(&mut self, register: Register, value: u16) {
        match register.width {
            Width::Byte => {
                if register.value < 4 {
                    self.registers[register.value as usize] =
                        (self.registers[register.value as usize] & 0xFF00) | (value & 0x00FF);
                } else {
                    let index = register.value as usize - 4;
                    self.registers[index] = (self.registers[index] & 0x00FF) | value << 8;
                }
            }
            Width::Word => self.registers[register.value as usize] = value,
        }
    }
}
