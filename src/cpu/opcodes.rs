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
    pub fn new(
        code: u8,
        mnemonic: &'static str,
        bytes: u8,
        cycles: u8,
        mode: AddressingMode,
    ) -> Self {
        Self {
            code,
            mnemonic,
            bytes,
            cycles,
            mode,
        }
    }
}

lazy_static! {
    pub(super) static ref OPCODES: HashMap<u8, Opcode> = {
        HashMap::from([
            (
                0x00,
                Opcode::new(0x00, "BRK", 1, 7, AddressingMode::NoneAddressing),
            ),
            (
                0x69,
                Opcode::new(0x69, "ADC", 2, 2, AddressingMode::Immediate),
            ),
            (
                0x65,
                Opcode::new(0x65, "ADC", 2, 3, AddressingMode::ZeroPage),
            ),
            (
                0x75,
                Opcode::new(0x75, "ADC", 2, 4, AddressingMode::ZeroPage_X),
            ),
            (
                0x6D,
                Opcode::new(0x6D, "ADC", 3, 4, AddressingMode::Absolute),
            ),
            (
                0x7D,
                Opcode::new(0x7D, "ADC", 3, 4, AddressingMode::Absolute_X),
            ),
            (
                0x79,
                Opcode::new(0x79, "ADC", 3, 4, AddressingMode::Absolute_Y),
            ),
            (
                0x61,
                Opcode::new(0x61, "ADC", 2, 6, AddressingMode::Indirect_X),
            ),
            (
                0x71,
                Opcode::new(0x71, "ADC", 2, 5, AddressingMode::Indirect_Y),
            ),
            (
                0x29,
                Opcode::new(0x29, "AND", 2, 2, AddressingMode::Immediate),
            ),
            (
                0x25,
                Opcode::new(0x25, "AND", 2, 3, AddressingMode::ZeroPage),
            ),
            (
                0x35,
                Opcode::new(0x35, "AND", 2, 4, AddressingMode::ZeroPage_X),
            ),
            (
                0x2D,
                Opcode::new(0x2D, "AND", 3, 4, AddressingMode::Absolute),
            ),
            (
                0x3D,
                Opcode::new(0x3D, "AND", 3, 4, AddressingMode::Absolute_X),
            ),
            (
                0x39,
                Opcode::new(0x39, "AND", 3, 4, AddressingMode::Absolute_Y),
            ),
            (
                0x21,
                Opcode::new(0x21, "AND", 2, 6, AddressingMode::Indirect_X),
            ),
            (
                0x31,
                Opcode::new(0x31, "AND", 2, 5, AddressingMode::Indirect_Y),
            ),
            (
                0x0A,
                Opcode::new(0x0A, "ASL", 1, 2, AddressingMode::Accumulator),
            ),
            (
                0x06,
                Opcode::new(0x06, "ASL", 2, 5, AddressingMode::ZeroPage),
            ),
            (
                0x16,
                Opcode::new(0x16, "ASL", 2, 6, AddressingMode::ZeroPage_X),
            ),
            (
                0x0E,
                Opcode::new(0x0E, "ASL", 3, 6, AddressingMode::Absolute),
            ),
            (
                0x1E,
                Opcode::new(0x1E, "ASL", 3, 7, AddressingMode::Absolute_X),
            ),
            (
                0x90,
                Opcode::new(0x90, "BCC", 2, 2, AddressingMode::Relative),
            ),
            (
                0xB0,
                Opcode::new(0xB0, "BCS", 2, 2, AddressingMode::Relative),
            ),
            (
                0xF0,
                Opcode::new(0xF0, "BEQ", 2, 2, AddressingMode::Relative),
            ),
            (
                0x24,
                Opcode::new(0x24, "BIT", 2, 3, AddressingMode::ZeroPage),
            ),
            (
                0x2C,
                Opcode::new(0x2C, "BIT", 3, 4, AddressingMode::Absolute),
            ),
            (
                0x30,
                Opcode::new(0x30, "BMI", 2, 2, AddressingMode::Relative),
            ),
            (
                0xD0,
                Opcode::new(0xD0, "BNE", 2, 2, AddressingMode::Relative),
            ),
            (
                0x10,
                Opcode::new(0x10, "BPL", 2, 2, AddressingMode::Relative),
            ),
            (
                0x50,
                Opcode::new(0x50, "BVC", 2, 2, AddressingMode::Relative),
            ),
            (
                0x70,
                Opcode::new(0x70, "BVS", 2, 2, AddressingMode::Relative),
            ),
            (
                0x18,
                Opcode::new(0x18, "CLC", 1, 2, AddressingMode::Implied),
            ),
            (
                0xD8,
                Opcode::new(0xD8, "CLD", 1, 2, AddressingMode::Implied),
            ),
            (
                0x58,
                Opcode::new(0x58, "CLI", 1, 2, AddressingMode::Implied),
            ),
            (
                0xB8,
                Opcode::new(0xB8, "CLV", 1, 2, AddressingMode::Implied),
            ),
            (
                0xC9,
                Opcode::new(0xC9, "CMP", 2, 2, AddressingMode::Immediate),
            ),
            (
                0xC5,
                Opcode::new(0xC5, "CMP", 2, 3, AddressingMode::ZeroPage),
            ),
            (
                0xD5,
                Opcode::new(0xD5, "CMP", 2, 4, AddressingMode::ZeroPage_X),
            ),
            (
                0xCD,
                Opcode::new(0xCD, "CMP", 3, 4, AddressingMode::Absolute),
            ),
            (
                0xDD,
                Opcode::new(0xDD, "CMP", 3, 4, AddressingMode::Absolute_X),
            ),
            (
                0xD9,
                Opcode::new(0xD9, "CMP", 3, 4, AddressingMode::Absolute_Y),
            ),
            (
                0xC1,
                Opcode::new(0xC1, "CMP", 2, 6, AddressingMode::Indirect_X),
            ),
            (
                0xD1,
                Opcode::new(0xD1, "CMP", 2, 5, AddressingMode::Indirect_Y),
            ),
            (
                0xE0,
                Opcode::new(0xE0, "CPX", 2, 2, AddressingMode::Immediate),
            ),
            (
                0xE4,
                Opcode::new(0xE4, "CPX", 2, 3, AddressingMode::ZeroPage),
            ),
            (
                0xEC,
                Opcode::new(0xEC, "CPX", 3, 4, AddressingMode::Absolute),
            ),
            (
                0xC0,
                Opcode::new(0xC0, "CPY", 2, 2, AddressingMode::Immediate),
            ),
            (
                0xC4,
                Opcode::new(0xC4, "CPY", 2, 3, AddressingMode::ZeroPage),
            ),
            (
                0xCC,
                Opcode::new(0xCC, "CPY", 3, 4, AddressingMode::Absolute),
            ),
            (
                0xC6,
                Opcode::new(0xC6, "DEC", 2, 5, AddressingMode::ZeroPage),
            ),
            (
                0xD6,
                Opcode::new(0xD6, "DEC", 2, 6, AddressingMode::ZeroPage_X),
            ),
            (
                0xCE,
                Opcode::new(0xCE, "DEC", 3, 6, AddressingMode::Absolute),
            ),
            (
                0xDE,
                Opcode::new(0xDE, "DEC", 3, 7, AddressingMode::Absolute_X),
            ),
            (
                0xCA,
                Opcode::new(0xCA, "DEX", 1, 2, AddressingMode::Implied),
            ),
            (
                0x88,
                Opcode::new(0x88, "DEY", 1, 2, AddressingMode::Implied),
            ),
            (
                0x49,
                Opcode::new(0x49, "EOR", 2, 2, AddressingMode::Immediate),
            ),
            (
                0x45,
                Opcode::new(0x45, "EOR", 2, 3, AddressingMode::ZeroPage),
            ),
            (
                0x55,
                Opcode::new(0x55, "EOR", 2, 4, AddressingMode::ZeroPage_X),
            ),
            (
                0x4D,
                Opcode::new(0x4D, "EOR", 3, 4, AddressingMode::Absolute),
            ),
            (
                0x5D,
                Opcode::new(0x5D, "EOR", 3, 4, AddressingMode::Absolute_X),
            ),
            (
                0x59,
                Opcode::new(0x59, "EOR", 3, 4, AddressingMode::Absolute_Y),
            ),
            (
                0x41,
                Opcode::new(0x41, "EOR", 2, 6, AddressingMode::Indirect_X),
            ),
            (
                0x51,
                Opcode::new(0x51, "EOR", 2, 5, AddressingMode::Indirect_Y),
            ),
            (
                0xE6,
                Opcode::new(0xE6, "INC", 2, 5, AddressingMode::ZeroPage),
            ),
            (
                0xF6,
                Opcode::new(0xF6, "INC", 2, 6, AddressingMode::ZeroPage_X),
            ),
            (
                0xEE,
                Opcode::new(0xEE, "INC", 3, 6, AddressingMode::Absolute),
            ),
            (
                0xFE,
                Opcode::new(0xFE, "INC", 3, 7, AddressingMode::Absolute_X),
            ),
            (
                0xE8,
                Opcode::new(0xE8, "INX", 1, 2, AddressingMode::Implied),
            ),
            (
                0xC8,
                Opcode::new(0xC8, "INY", 1, 2, AddressingMode::Implied),
            ),
            (
                0x4C,
                Opcode::new(0x4C, "JMP", 3, 3, AddressingMode::Absolute),
            ),
            (
                0x6C,
                Opcode::new(0x6C, "JMP", 3, 5, AddressingMode::Indirect),
            ),
            (0x20, Opcode::new(0x20, "JSR", 3, 6, AddressingMode::Absolute)),
            (
                0xAA,
                Opcode::new(0xAA, "TAX", 1, 2, AddressingMode::NoneAddressing),
            ),
            (
                0xA9,
                Opcode::new(0xA9, "LDA", 2, 2, AddressingMode::Immediate),
            ),
            (
                0xA5,
                Opcode::new(0xA5, "LDA", 2, 3, AddressingMode::ZeroPage),
            ),
            (
                0xB5,
                Opcode::new(0xB5, "LDA", 2, 4, AddressingMode::ZeroPage_X),
            ),
            (
                0xAD,
                Opcode::new(0xAD, "LDA", 3, 4, AddressingMode::Absolute),
            ),
            (
                0xBD,
                Opcode::new(0xBD, "LDA", 3, 4, AddressingMode::Absolute_X),
            ),
            (
                0xB9,
                Opcode::new(0xB9, "LDA", 3, 4, AddressingMode::Absolute_Y),
            ),
            (
                0xA1,
                Opcode::new(0xA1, "LDA", 2, 6, AddressingMode::Indirect_X),
            ),
            (
                0xB1,
                Opcode::new(0xB1, "LDA", 2, 5, AddressingMode::Indirect_Y),
            ),

            (0xA2, Opcode::new(0xA2, "LDX", 2, 2, AddressingMode::Immediate)),
            (0xA6, Opcode::new(0xA6, "LDX", 2, 3, AddressingMode::ZeroPage)),
            (0xB6, Opcode::new(0xB6, "LDX", 2, 4, AddressingMode::ZeroPage_Y)),
            (0xAE, Opcode::new(0xAE, "LDX", 3, 4, AddressingMode::Absolute)),
            (0xBE, Opcode::new(0xBE, "LDX", 3, 4, AddressingMode::Absolute_Y)),

            (0xA0, Opcode::new(0xA0, "LDY", 2, 2, AddressingMode::Immediate)),
            (0xA4, Opcode::new(0xA4, "LDY", 2, 3, AddressingMode::ZeroPage)),
            (0xB4, Opcode::new(0xB4, "LDY", 2, 4, AddressingMode::ZeroPage_X)),
            (0xAC, Opcode::new(0xAC, "LDY", 3, 4, AddressingMode::Absolute)),
            (0xBC, Opcode::new(0xBC, "LDY", 3, 4, AddressingMode::Absolute_X)),

            (0x4A, Opcode::new(0x4A, "LSR", 1, 2, AddressingMode::Accumulator)),
            (0x46, Opcode::new(0x46, "LSR", 2, 5, AddressingMode::ZeroPage)),
            (0x56, Opcode::new(0x56, "LSR", 2, 6, AddressingMode::ZeroPage_Y)),
            (0x4E, Opcode::new(0x4E, "LSR", 3, 6, AddressingMode::Absolute)),
            (0x5E, Opcode::new(0x5E, "LSR", 3, 7, AddressingMode::Absolute_X)),

            (0xEA, Opcode::new(0xEA, "NOP", 1, 2, AddressingMode::Implied)),

            (0x09, Opcode::new(0x09, "ORA", 2, 2, AddressingMode::Immediate)),
            (0x05, Opcode::new(0x05, "ORA", 2, 2, AddressingMode::ZeroPage)),
            (0x15, Opcode::new(0x15, "ORA", 2, 2, AddressingMode::ZeroPage_X)),
            (0x0D, Opcode::new(0x0D, "ORA", 2, 2, AddressingMode::Absolute)),
            (0x1D, Opcode::new(0x1D, "ORA", 2, 2, AddressingMode::Absolute_X)),
            (0x19, Opcode::new(0x19, "ORA", 2, 2, AddressingMode::Absolute_Y)),
            (0x01, Opcode::new(0x01, "ORA", 2, 2, AddressingMode::Indirect_X)),
            (0x11, Opcode::new(0x11, "ORA", 2, 2, AddressingMode::Indirect_Y)),

            (0x48, Opcode::new(0x48, "PHA", 1, 3, AddressingMode::Implied)),
            (0x08, Opcode::new(0x08, "PHP", 1, 3, AddressingMode::Implied)),
            (0x68, Opcode::new(0x68, "PLA", 1, 4, AddressingMode::Implied)),
            (0x28, Opcode::new(0x28, "PLP", 1, 4, AddressingMode::Implied)),

            (
                0x85,
                Opcode::new(0x85, "STA", 2, 3, AddressingMode::ZeroPage),
            ),
            (
                0x95,
                Opcode::new(0x95, "STA", 2, 4, AddressingMode::ZeroPage_X),
            ),
            (
                0x8D,
                Opcode::new(0x8D, "STA", 2, 4, AddressingMode::Absolute),
            ),
            (
                0x9D,
                Opcode::new(0x9D, "STA", 2, 5, AddressingMode::Absolute_X),
            ),
            (
                0x99,
                Opcode::new(0x99, "STA", 2, 5, AddressingMode::Absolute_Y),
            ),
            (
                0x81,
                Opcode::new(0x81, "STA", 2, 6, AddressingMode::Indirect_X),
            ),
            (
                0x91,
                Opcode::new(0x91, "STA", 2, 6, AddressingMode::Indirect_Y),
            ),
        ])
    };
}
