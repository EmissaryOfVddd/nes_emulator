use crate::cpu::constants::{DECIMAL_MODE, INTERRUPT_DISABLE};

use self::constants::{CARRY_FLAG, NEGATIVE_FLAG, OVERFLOW_FLAG, ZERO_FLAG};

use super::opcodes::OPCODES;

pub struct CPU {
    pub register_a: u8,
    pub register_x: u8,
    pub register_y: u8,
    pub flags: u8,
    pub program_counter: u16,
    memory: [u8; 0xFFFF],
}

#[derive(Debug)]
#[allow(non_camel_case_types)]
pub enum AddressingMode {
    Immediate,
    ZeroPage,
    ZeroPage_X,
    ZeroPage_Y,
    Absolute,
    Absolute_X,
    Absolute_Y,
    Inderect_X,
    Inderect_Y,
    NoneAddressing,
    Implied,
    Relative,
}

impl CPU {
    pub fn new() -> Self {
        CPU {
            register_a: 0,
            register_x: 0,
            register_y: 0,
            flags: 0,
            program_counter: 0,
            memory: [0x0; 0xFFFF],
        }
    }

    pub fn run(&mut self) {
        loop {
            let opscode = self.mem_read(self.program_counter);
            println!("{opscode:X}");
            self.inc_prg();

            let opcode = OPCODES
                .get(&opscode)
                .expect("That is really fucked up opcode");

            match opscode {
                // BRK
                0x00 => return,
                // ADC
                0x69 | 0x65 | 0x75 | 0x6D | 0x7D | 0x79 | 0x61 | 0x71 => {
                    self.adc(&opcode.mode);
                }
                // AND
                0x29 | 0x25 | 0x35 | 0x2D | 0x3D | 0x39 | 0x21 | 0x31 => {
                    self.and(&opcode.mode);
                }
                // ASL
                0x0A | 0x06 | 0x16 | 0x0E | 0x1E => {
                    self.asl(&opcode.mode);
                }
                // BCC
                0x90 => self.branch(!self.check_flag(CARRY_FLAG)),
                // BCS
                0xB0 => self.branch(self.check_flag(CARRY_FLAG)),
                // BEQ
                0xF0 => self.branch(self.check_flag(ZERO_FLAG)),
                // BIT
                0x24 | 0x2C => self.bit(&opcode.mode),
                // BMI
                0x30 => self.branch(self.check_flag(NEGATIVE_FLAG)),
                // BNE
                0xD0 => self.branch(!self.check_flag(ZERO_FLAG)),
                // BPL
                0x10 => self.branch(!self.check_flag(NEGATIVE_FLAG)),
                // BVC
                0x50 => self.branch(!self.check_flag(OVERFLOW_FLAG)),
                // BVS
                0x70 => self.branch(self.check_flag(OVERFLOW_FLAG)),
                // CLC
                0x18 => self.remove_flag(CARRY_FLAG),
                // CLD
                0xD8 => self.remove_flag(DECIMAL_MODE),
                // CLI
                0x58 => self.remove_flag(INTERRUPT_DISABLE),
                // CLV
                0xB8 => self.remove_flag(OVERFLOW_FLAG),
                // CMP
                0xC9 | 0xC5 | 0xD5 | 0xCD | 0xDD | 0xD9 | 0xC1 | 0xD1 => self.compare(&opcode.mode, self.register_a),
                // CPX
                0xE0 | 0xE4 | 0xEC => self.compare(&opcode.mode, self.register_x),
                // CPY
                0xC0 | 0xC4 | 0xCC => self.compare(&opcode.mode, self.register_y),
                // DEC
                0xC6 | 0xD6 | 0xCE | 0xDE => self.dec(&opcode.mode),
                // INX
                0xE8 => self.inx(),
                // LDA
                0xA9 | 0xA5 | 0xAD => self.lda(&opcode.mode),
                // STA
                0x85 | 0x95 => self.sta(&opcode.mode),
                // TAX
                0xAA => self.tax(),
                _ => todo!(),
            }

            self.inc_prg_by(opcode.bytes);
        }
    }

    fn adc(&mut self, mode: &AddressingMode) {
        let addr = self.get_operand_address(mode);
        let data = self.mem_read(addr);
        let data_u16 = data as u16;

        let reg_a_u16 = self.register_a as u16;
        let carry = self.get_carry() as u16;

        let res = reg_a_u16 + data_u16 + carry;
        let set_carry = res > 0xff;
        let res_u8 = res as u8;

        let sign_reg_a = self.register_a >> 7;
        let sign_data = data >> 7;
        let sign_res = res_u8 >> 7;
        if sign_reg_a == sign_data && sign_res != sign_reg_a {
            self.set_flag(OVERFLOW_FLAG);
        }

        if set_carry {
            self.set_flag(CARRY_FLAG);
        } else {
            self.remove_flag(CARRY_FLAG);
        }

        self.register_a = res_u8;
    }

    fn and(&mut self, mode: &AddressingMode) {
        let addr = self.get_operand_address(mode);
        let data = self.mem_read(addr);

        self.register_a &= data;

        self.update_neg_and_zero_status(self.register_a);
    }

    fn asl(&mut self, mode: &AddressingMode) {
        let addr = match mode {
            AddressingMode::Immediate => {
                if self.register_a >= 0x80 {
                    self.set_flag(CARRY_FLAG);
                } else {
                    self.remove_flag(CARRY_FLAG);
                }

                self.register_a <<= 1;
                self.update_neg_and_zero_status(self.register_a);
                return;
            }
            _ => self.get_operand_address(mode),
        };

        let data = self.mem_read(addr);
        if data >= 0x80 {
            self.set_flag(CARRY_FLAG);
        } else {
            self.remove_flag(CARRY_FLAG);
        }

        let data = data << 1;
        self.update_neg_and_zero_status(data);
        self.mem_write(addr, data);
    }

    fn branch(&mut self, condition: bool) {
        if condition {
            let jmp = self.mem_read(self.program_counter) as i8;
            let jmp_addr = self
                .program_counter
                .wrapping_add(1)
                .wrapping_add(jmp as u16);

            self.program_counter = jmp_addr;
        }
    }

    fn bit(&mut self, mode: &AddressingMode) {
        let addr = self.get_operand_address(mode);
        let data = self.mem_read(addr);

        println!("{:b} {:b}", self.register_a, data);

        let res = self.register_a & data;
        if res == 0 {
            self.set_flag(ZERO_FLAG);
        }

        self.set_flag(data & (OVERFLOW_FLAG | NEGATIVE_FLAG));
    }

    fn compare(&mut self, mode: &AddressingMode, value: u8) {
        let addr = self.get_operand_address(mode);
        let data = self.mem_read(addr);

        if value >= data {
            self.set_flag(CARRY_FLAG);        
        } else {
            self.remove_flag(CARRY_FLAG);
        }

        self.update_neg_and_zero_status(value.wrapping_sub(data));
    }

    fn dec(&mut self, mode: &AddressingMode) {
        let addr = self.get_operand_address(mode);
        let data = self.mem_read(addr);
        
        let data = data.wrapping_sub(1);
        self.mem_write(addr, data);

        self.update_neg_and_zero_status(data);
    }

    fn dex(&mut self) {
        self.register_x = self.register_x.wrapping_sub(1);
        self.update_neg_and_zero_status(self.register_x);
    }

    fn dey(&mut self) {
        self.register_x = self.register_y.wrapping_sub(1);
        self.update_neg_and_zero_status(self.register_y);
    }

    fn get_carry(&self) -> u8 {
        if self.check_flag(CARRY_FLAG) {
            1
        } else {
            0
        }
    }

    fn inx(&mut self) {
        self.register_x = self.register_x.wrapping_add(1);
        self.update_neg_and_zero_status(self.register_x);
    }

    fn lda(&mut self, mode: &AddressingMode) {
        let addr = self.get_operand_address(mode);
        let value = self.mem_read(addr);

        self.register_a = value;
        self.update_neg_and_zero_status(self.register_a);
    }

    fn sta(&mut self, mode: &AddressingMode) {
        let addr = self.get_operand_address(mode);
        self.mem_write(addr, self.register_a);
    }

    fn tax(&mut self) {
        self.register_x = self.register_a;
        self.update_neg_and_zero_status(self.register_x);
    }

    fn get_operand_address(&self, mode: &AddressingMode) -> u16 {
        match mode {
            AddressingMode::Immediate => self.program_counter,
            AddressingMode::ZeroPage => self.mem_read(self.program_counter) as u16,
            AddressingMode::Absolute => self.mem_read_u16(self.program_counter),
            AddressingMode::ZeroPage_X => {
                let pos = self.mem_read(self.program_counter);
                let addr = pos.wrapping_add(self.register_x) as u16;
                addr
            }
            AddressingMode::ZeroPage_Y => {
                let pos = self.mem_read(self.program_counter);
                let addr = pos.wrapping_add(self.register_y) as u16;
                addr
            }
            AddressingMode::Absolute_X => {
                let base = self.mem_read_u16(self.program_counter);
                let addr = base.wrapping_add(self.register_x as _);
                addr
            }
            AddressingMode::Absolute_Y => {
                let base = self.mem_read_u16(self.program_counter);
                let addr = base.wrapping_add(self.register_y as _);
                addr
            }
            AddressingMode::Inderect_X => {
                let base = self.mem_read(self.program_counter);

                let ptr: u8 = base.wrapping_add(self.register_x);
                let lo = self.mem_read(ptr as u16);
                let hi = self.mem_read(ptr as u16 + 1);
                u16::from_le_bytes([lo, hi])
            }
            AddressingMode::Inderect_Y => {
                let base = self.mem_read(self.program_counter);

                let ptr: u8 = base.wrapping_add(self.register_y);
                let lo = self.mem_read(ptr as u16);
                let hi = self.mem_read(ptr as u16 + 1);
                u16::from_le_bytes([lo, hi])
            }
            _ => panic!("mode {mode:?} is not supported"),
        }
    }

    pub(super) fn mem_read(&self, addr: u16) -> u8 {
        self.memory[addr as usize]
    }

    pub(super) fn mem_read_u16(&self, pos: u16) -> u16 {
        let lo = self.mem_read(pos);
        let hi = self.mem_read(pos + 1);
        u16::from_le_bytes([lo, hi])
    }

    pub(super) fn mem_write(&mut self, addr: u16, data: u8) {
        self.memory[addr as usize] = data;
    }

    pub(super) fn mem_write_u16(&mut self, pos: u16, data: u16) {
        let [lo, hi] = data.to_le_bytes();
        self.mem_write(pos, lo);
        self.mem_write(pos + 1, hi);
    }

    pub fn load_and_run(&mut self, program: Vec<u8>) {
        self.load(program);
        self.reset();
        self.run()
    }

    pub fn reset(&mut self) {
        self.register_a = 0;
        self.register_x = 0;
        self.register_y = 0;
        self.flags = 0;

        self.program_counter = self.mem_read_u16(0xFFFC);
    }

    pub fn load(&mut self, program: Vec<u8>) {
        self.memory[0x8000..(0x8000 + program.len())].copy_from_slice(&program[..]);
        self.mem_write_u16(0xFFFC, 0x8000);
    }

    fn inc_prg(&mut self) {
        self.program_counter += 1;
    }

    fn inc_prg_by(&mut self, amount: u8) {
        self.program_counter += (amount - 1) as u16;
    }

    pub(super) fn check_flag(&self, flag: u8) -> bool {
        self.flags & flag != 0
    }

    fn set_flag(&mut self, flag: u8) {
        self.flags |= flag;
    }

    fn remove_flag(&mut self, flag: u8) {
        self.flags &= !flag;
    }

    fn update_neg_and_zero_status(&mut self, value: u8) {
        use constants::*;

        if value == 0 {
            self.set_flag(ZERO_FLAG);
        } else {
            self.remove_flag(ZERO_FLAG);
        }

        if value & NEGATIVE_FLAG != 0 {
            self.set_flag(NEGATIVE_FLAG);
        } else {
            self.remove_flag(NEGATIVE_FLAG);
        }
    }
}

pub mod constants {
    pub const CARRY_FLAG: u8 = 0b0000_0001;
    pub const ZERO_FLAG: u8 = 0b0000_0010;
    pub const INTERRUPT_DISABLE: u8 = 0b0000_0100;
    pub const DECIMAL_MODE: u8 = 0b0000_1000;
    pub const OVERFLOW_FLAG: u8 = 0b0100_0000;
    pub const NEGATIVE_FLAG: u8 = 0b1000_0000;
}
