use std::io::{Write, stdout};

use crate::{
    sim::{Sim86, utils::get_sp_memory_operand},
    types::{
        instruction::{Instruction, InstructionData},
        width::Width,
    },
};

use crossterm::{
    event::{self, Event, KeyCode},
    terminal::{disable_raw_mode, enable_raw_mode},
};

impl Sim86 {
    pub(super) fn execute_instruction(&mut self, instruction: Instruction) {
        match instruction.data {
            InstructionData::MOV {
                destination,
                source,
                width,
            } => self.set_value(destination, self.get_value(&source, &width), &width),
            InstructionData::PUSH { operand } => {
                let value = self.get_value(&operand, &Width::Word);
                self.registers[0b100] = self.registers[0b100].wrapping_sub(2);
                self.set_value(get_sp_memory_operand(), value, &Width::Word);
            }
            InstructionData::POP { operand } => {
                self.set_value(
                    operand,
                    self.get_value(&get_sp_memory_operand(), &Width::Word),
                    &Width::Word,
                );
                self.registers[0b100] = self.registers[0b100].wrapping_add(2);
            }
            InstructionData::CALL(jump) => {
                let return_ip = self.ip;
                let old_cs = self.segment_registers[0b01];

                let (ip, cs) = self.get_new_ip_cs(jump);

                if cs.is_some() {
                    self.registers[0b100] = self.registers[0b100].wrapping_sub(2);
                    self.set_value(get_sp_memory_operand(), old_cs, &Width::Word);
                }

                self.registers[0b100] = self.registers[0b100].wrapping_sub(2);
                self.set_value(get_sp_memory_operand(), return_ip, &Width::Word);
                self.ip = ip;

                if let Some(cs) = cs {
                    self.segment_registers[0b01] = cs;
                }
            }
            InstructionData::RET(sp_inc) => {
                self.ip = self.get_value(&get_sp_memory_operand(), &Width::Word);
                self.registers[0b100] = self.registers[0b100].wrapping_add(2 + sp_inc);
            }
            InstructionData::RETF(sp_inc) => {
                self.ip = self.get_value(&get_sp_memory_operand(), &Width::Word);
                self.registers[0b100] = self.registers[0b100].wrapping_add(2);
                self.segment_registers[0b01] =
                    self.get_value(&get_sp_memory_operand(), &Width::Word);
                self.registers[0b100] = self.registers[0b100].wrapping_add(2 + sp_inc);
            }
            InstructionData::JMP(jump) => {
                let (ip, cs) = self.get_new_ip_cs(jump);
                self.ip = ip;
                if let Some(cs) = cs {
                    self.segment_registers[0b01] = cs;
                }
            }
            InstructionData::LEA { operand, eac } => {
                self.set_value(operand, self.get_effective_address(&eac), &Width::Word)
            }
            InstructionData::AluBinary {
                op_code,
                destination,
                source,
                width,
            } => self.exec_alu_binary_instruction(op_code, destination, source, width),
            InstructionData::AluUnary {
                op_code,
                width,
                operand,
            } => self.exec_alu_unary_instruction(op_code, operand, width),
            InstructionData::ShiftRotate {
                op_code,
                operand,
                amount,
                width,
            } => self.exec_shift_rotate_instruction(op_code, operand, amount, width),
            InstructionData::ConditionalJump { op_code, target } => {
                self.exec_conditional_jump(op_code, target)
            }
            InstructionData::IN {
                width,
                operand,
                port,
            } => {
                let port = self.get_value(&port, &width);

                if port == 0x00 {
                    enable_raw_mode().unwrap();

                    let result = loop {
                        if let Event::Key(key_event) = event::read().unwrap() {
                            match key_event.code {
                                KeyCode::Enter => break '\n' as u16,
                                KeyCode::Backspace => break '\x08' as u16,
                                KeyCode::Left => break 0x82,
                                KeyCode::Down => break 0x81,
                                KeyCode::Up => break 0x80,
                                KeyCode::Right => break 0x83,
                                KeyCode::Char(c) => break c as u16,
                                _ => {}
                            }
                        }
                    };

                    disable_raw_mode().unwrap();

                    self.set_value(operand, result, &width);
                }
            }
            InstructionData::OUT {
                width,
                operand,
                port,
            } => {
                let port = self.get_value(&port, &width);

                if port == 0x01 {
                    print!("{}", self.get_value(&operand, &width) as u8 as char);
                    if let Width::Word = width {
                        print!("{}", (self.get_value(&operand, &width) >> 8) as u8 as char);
                    }
                    stdout().flush().unwrap();
                }
            }
            InstructionData::CBW => {
                self.registers[0] = (self.registers[0] & 0x00FF) as u8 as i8 as i16 as u16;
            }
            InstructionData::HLT => unreachable!(),
        }
    }
}
