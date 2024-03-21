use super::opcodes::OPCODES;

pub struct CPU {
    pub register_a: u8,
    pub register_x: u8,
    pub register_y: u8,
    pub status: u8,
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
}

impl CPU {
    pub fn new() -> Self {
        CPU {
            register_a: 0,
            register_x: 0,
            register_y: 0,
            status: 0,
            program_counter: 0,
            memory: [0x0; 0xFFFF],
        }
    }

    pub fn run(&mut self) {
        loop {
            let opscode = self.mem_read(self.program_counter);
            self.inc_prg();

            let opcode = OPCODES.get(&opscode).expect("That is really fucked up opcode");

            match opscode {
                // BRK
                0x00 => return,
                // INX
                0xE8 => self.inx(),
                // LDA
                0xA9 | 0xA5 | 0xAD => {
                    self.lda(&opcode.mode);
                    self.inc_prg_by(opcode.cycles)
                }
                // STA
                0x85 | 0x95 => {
                    self.sta(&opcode.mode);
                    self.inc_prg_by(opcode.cycles)
                }
                // TAX
                0xAA => self.tax(),
                _ => todo!(),
            }
        }
    }

    pub(super) fn inx(&mut self) {
        self.register_x = self.register_x.wrapping_add(1);
        self.update_neg_and_zero_status(self.register_x);
    }

    pub(super) fn lda(&mut self, mode: &AddressingMode) {
        let addr = self.get_operand_address(mode);
        let value = self.mem_read(addr);

        self.register_a = value;
        self.update_neg_and_zero_status(self.register_a);
    }

    pub(super) fn sta(&mut self, mode: &AddressingMode) {
        let addr = self.get_operand_address(mode);
        self.mem_write(addr, self.register_a);
    }

    pub(super) fn tax(&mut self) {
        self.register_x = self.register_a;
        self.update_neg_and_zero_status(self.register_x);
    }

    pub(super) fn get_operand_address(&self, mode: &AddressingMode) -> u16 {
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
        self.status = 0;

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

    fn apply_status(&mut self, flag: u8) {
        self.status |= flag;
    }

    fn remove_status(&mut self, flag: u8) {
        self.status &= !flag;
    }

    fn update_neg_and_zero_status(&mut self, reg: u8) {
        use constants::*;

        if reg == 0 {
            self.apply_status(ZERO_FLAG);
        } else {
            self.remove_status(ZERO_FLAG);
        }

        if reg & NEGATIVE_FLAG != 0 {
            self.apply_status(NEGATIVE_FLAG);
        } else {
            self.remove_status(NEGATIVE_FLAG);
        }
    }
}

pub mod constants {
    pub const ZERO_FLAG: u8 = 0b0000_0010;
    pub const NEGATIVE_FLAG: u8 = 0b1000_0000;
}


