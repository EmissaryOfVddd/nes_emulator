use std::collections::HashMap;

use super::cpu::AddressingMode;
use lazy_static::lazy_static;

pub(super) struct Opcode {
    pub code: u8,
    pub mnemonic: &'static str,
    pub bytes: u8,
    pub _cycles: u8,
    pub mode: AddressingMode,
}

impl Opcode {
    pub fn new(
        _code: u8,
        mnemonic: &'static str,
        bytes: u8,
        _cycles: u8,
        mode: AddressingMode,
    ) -> Self {
        Self {
            code: _code,
            mnemonic,
            bytes,
            _cycles,
            mode,
        }
    }
}

fn op(
    opcode: u8,
    mnemonic: &'static str,
    bytes: u8,
    cycles: u8,
    mode: AddressingMode,
) -> (u8, Opcode) {
    (opcode, Opcode::new(opcode, mnemonic, bytes, cycles, mode))
}

lazy_static! {
    pub(super) static ref OPCODES: HashMap<u8, Opcode> = {
        HashMap::from([
            op(0x00, "BRK", 1, 7, AddressingMode::NoneAddressing),
            op(0x69, "ADC", 2, 2, AddressingMode::Immediate),
            op(0x65, "ADC", 2, 3, AddressingMode::ZeroPage),
            op(0x75, "ADC", 2, 4, AddressingMode::ZeroPage_X),
            op(0x6D, "ADC", 3, 4, AddressingMode::Absolute),
            op(0x7D, "ADC", 3, 4, AddressingMode::Absolute_X),
            op(0x79, "ADC", 3, 4, AddressingMode::Absolute_Y),
            op(0x61, "ADC", 2, 6, AddressingMode::Indirect_X),
            op(0x71, "ADC", 2, 5, AddressingMode::Indirect_Y),
            op(0x29, "AND", 2, 2, AddressingMode::Immediate),
            op(0x25, "AND", 2, 3, AddressingMode::ZeroPage),
            op(0x35, "AND", 2, 4, AddressingMode::ZeroPage_X),
            op(0x2D, "AND", 3, 4, AddressingMode::Absolute),
            op(0x3D, "AND", 3, 4, AddressingMode::Absolute_X),
            op(0x39, "AND", 3, 4, AddressingMode::Absolute_Y),
            op(0x21, "AND", 2, 6, AddressingMode::Indirect_X),
            op(0x31, "AND", 2, 5, AddressingMode::Indirect_Y),
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
            (
                0x20,
                Opcode::new(0x20, "JSR", 3, 6, AddressingMode::Absolute),
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
            (
                0xA2,
                Opcode::new(0xA2, "LDX", 2, 2, AddressingMode::Immediate),
            ),
            (
                0xA6,
                Opcode::new(0xA6, "LDX", 2, 3, AddressingMode::ZeroPage),
            ),
            (
                0xB6,
                Opcode::new(0xB6, "LDX", 2, 4, AddressingMode::ZeroPage_Y),
            ),
            (
                0xAE,
                Opcode::new(0xAE, "LDX", 3, 4, AddressingMode::Absolute),
            ),
            (
                0xBE,
                Opcode::new(0xBE, "LDX", 3, 4, AddressingMode::Absolute_Y),
            ),
            (
                0xA0,
                Opcode::new(0xA0, "LDY", 2, 2, AddressingMode::Immediate),
            ),
            (
                0xA4,
                Opcode::new(0xA4, "LDY", 2, 3, AddressingMode::ZeroPage),
            ),
            (
                0xB4,
                Opcode::new(0xB4, "LDY", 2, 4, AddressingMode::ZeroPage_X),
            ),
            (
                0xAC,
                Opcode::new(0xAC, "LDY", 3, 4, AddressingMode::Absolute),
            ),
            (
                0xBC,
                Opcode::new(0xBC, "LDY", 3, 4, AddressingMode::Absolute_X),
            ),
            (
                0x4A,
                Opcode::new(0x4A, "LSR", 1, 2, AddressingMode::Accumulator),
            ),
            (
                0x46,
                Opcode::new(0x46, "LSR", 2, 5, AddressingMode::ZeroPage),
            ),
            (
                0x56,
                Opcode::new(0x56, "LSR", 2, 6, AddressingMode::ZeroPage_Y),
            ),
            (
                0x4E,
                Opcode::new(0x4E, "LSR", 3, 6, AddressingMode::Absolute),
            ),
            (
                0x5E,
                Opcode::new(0x5E, "LSR", 3, 7, AddressingMode::Absolute_X),
            ),
            (
                0xEA,
                Opcode::new(0xEA, "NOP", 1, 2, AddressingMode::Implied),
            ),
            (
                0x09,
                Opcode::new(0x09, "ORA", 2, 2, AddressingMode::Immediate),
            ),
            (
                0x05,
                Opcode::new(0x05, "ORA", 2, 2, AddressingMode::ZeroPage),
            ),
            (
                0x15,
                Opcode::new(0x15, "ORA", 2, 2, AddressingMode::ZeroPage_X),
            ),
            (
                0x0D,
                Opcode::new(0x0D, "ORA", 2, 2, AddressingMode::Absolute),
            ),
            (
                0x1D,
                Opcode::new(0x1D, "ORA", 2, 2, AddressingMode::Absolute_X),
            ),
            (
                0x19,
                Opcode::new(0x19, "ORA", 2, 2, AddressingMode::Absolute_Y),
            ),
            (
                0x01,
                Opcode::new(0x01, "ORA", 2, 2, AddressingMode::Indirect_X),
            ),
            (
                0x11,
                Opcode::new(0x11, "ORA", 2, 2, AddressingMode::Indirect_Y),
            ),
            (
                0x48,
                Opcode::new(0x48, "PHA", 1, 3, AddressingMode::Implied),
            ),
            (
                0x08,
                Opcode::new(0x08, "PHP", 1, 3, AddressingMode::Implied),
            ),
            (
                0x68,
                Opcode::new(0x68, "PLA", 1, 4, AddressingMode::Implied),
            ),
            (
                0x28,
                Opcode::new(0x28, "PLP", 1, 4, AddressingMode::Implied),
            ),
            op(0x2A, "ROL", 1, 2, AddressingMode::Accumulator),
            op(0x26, "ROL", 2, 5, AddressingMode::ZeroPage),
            op(0x36, "ROL", 2, 6, AddressingMode::ZeroPage_X),
            op(0x2E, "ROL", 3, 6, AddressingMode::Absolute),
            op(0x3E, "ROL", 3, 7, AddressingMode::Absolute_X),
            op(0x6A, "ROR", 1, 2, AddressingMode::Accumulator),
            op(0x66, "ROR", 2, 5, AddressingMode::ZeroPage),
            op(0x76, "ROR", 2, 6, AddressingMode::ZeroPage_X),
            op(0x6E, "ROR", 3, 6, AddressingMode::Absolute),
            op(0x7E, "ROR", 3, 7, AddressingMode::Absolute_X),
            op(0x40, "RTI", 1, 6, AddressingMode::Implied),
            op(0x60, "RTS", 1, 6, AddressingMode::Implied),
            op(0xE9, "SBC", 2, 2, AddressingMode::Immediate),
            op(0xE5, "SBC", 2, 3, AddressingMode::ZeroPage),
            op(0xF5, "SBC", 2, 4, AddressingMode::ZeroPage_X),
            op(0xED, "SBC", 3, 4, AddressingMode::Absolute),
            op(0xFD, "SBC", 3, 4, AddressingMode::Absolute_X),
            op(0xF9, "SBC", 3, 4, AddressingMode::Absolute_Y),
            op(0xE1, "SBC", 2, 6, AddressingMode::Indirect_X),
            op(0xF1, "SBC", 2, 5, AddressingMode::Indirect_Y),
            op(0x38, "SEC", 1, 2, AddressingMode::Implied),
            op(0xF8, "SED", 1, 2, AddressingMode::Implied),
            op(0x78, "SEI", 1, 2, AddressingMode::Implied),
            op(0x85, "STA", 2, 3, AddressingMode::ZeroPage),
            op(0x95, "STA", 2, 4, AddressingMode::ZeroPage_X),
            op(0x8D, "STA", 3, 4, AddressingMode::Absolute),
            op(0x9D, "STA", 3, 5, AddressingMode::Absolute_X),
            op(0x99, "STA", 3, 5, AddressingMode::Absolute_Y),
            op(0x81, "STA", 2, 6, AddressingMode::Indirect_X),
            op(0x91, "STA", 2, 6, AddressingMode::Indirect_Y),
            op(0x86, "STX", 2, 3, AddressingMode::ZeroPage),
            op(0x96, "STX", 2, 4, AddressingMode::ZeroPage_Y),
            op(0x8E, "STX", 3, 4, AddressingMode::Absolute),
            op(0x84, "STY", 2, 3, AddressingMode::ZeroPage),
            op(0x94, "STY", 2, 4, AddressingMode::ZeroPage_X),
            op(0x8C, "STY", 3, 4, AddressingMode::Absolute),
            op(0xAA, "TAX", 1, 2, AddressingMode::Implied),
            op(0xA8, "TAY", 1, 2, AddressingMode::Implied),
            op(0xBA, "TSX", 1, 2, AddressingMode::Implied),
            op(0x8A, "TXA", 1, 2, AddressingMode::Implied),
            op(0x9A, "TXS", 1, 2, AddressingMode::Implied),
            op(0x98, "TYA", 1, 2, AddressingMode::Implied),
        ])
    };
}
