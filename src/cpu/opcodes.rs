use std::collections::HashMap;

use super::cpu::AddressingMode;
use lazy_static::lazy_static;

pub(super) struct Opcode {
    pub code: u8,
    pub mnemonic: &'static str,
    pub bytes: u8,
    pub cycles: u8,
    pub mode: AddressingMode,
}

impl Opcode {
    pub fn new(code: u8, mnemonic: &'static str, bytes: u8, cycles: u8, mode: AddressingMode) -> Self {
        Self {
            code,
            mnemonic,
            bytes,
            cycles, mode
        }
    }
}

lazy_static! {
    pub(super) static ref OPCODES: HashMap<u8, Opcode> = {
        HashMap::from([
            (0x00, Opcode::new(0x00, "BRK", 1, 7, AddressingMode::NoneAddressing)),
            (0xAA, Opcode::new(0xAA, "TAX", 1, 2, AddressingMode::NoneAddressing)),

            (0xE8, Opcode::new(0xE8, "INX", 1, 2, AddressingMode::Implied)),

            (0xA9, Opcode::new(0xA9, "LDA", 1, 2, AddressingMode::Immediate)),
            (0xA5, Opcode::new(0xA5, "LDA", 1, 3, AddressingMode::ZeroPage)),
            (0xB5, Opcode::new(0xB5, "LDA", 1, 4, AddressingMode::ZeroPage_X)),
            (0xAD, Opcode::new(0xAD, "LDA", 1, 4, AddressingMode::Absolute)),
            (0xBD, Opcode::new(0xBD, "LDA", 1, 4, AddressingMode::Absolute_X)),
            (0xB9, Opcode::new(0xB9, "LDA", 1, 4, AddressingMode::Absolute_Y)),
            (0xA1, Opcode::new(0xA1, "LDA", 1, 6, AddressingMode::Inderect_X)),
            (0xB1, Opcode::new(0xB1, "LDA", 1, 5, AddressingMode::Inderect_Y)),

            (0x85, Opcode::new(0x85, "STA", 2, 3, AddressingMode::ZeroPage)), 
            (0x95, Opcode::new(0x95, "STA", 2, 4, AddressingMode::ZeroPage_X)), 
            (0x8D, Opcode::new(0x8D, "STA", 2, 4, AddressingMode::Absolute)), 
            (0x9D, Opcode::new(0x9D, "STA", 2, 5, AddressingMode::Absolute_X)), 
            (0x99, Opcode::new(0x99, "STA", 2, 5, AddressingMode::Absolute_Y)), 
            (0x81, Opcode::new(0x81, "STA", 2, 6, AddressingMode::Inderect_X)), 
            (0x91, Opcode::new(0x91, "STA", 2, 6, AddressingMode::Inderect_Y)), 
        ])
    };
}
