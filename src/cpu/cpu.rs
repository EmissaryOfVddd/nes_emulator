use crate::{bus::bus::Bus, cpu::constants::{DECIMAL_MODE, INTERRUPT_DISABLE}};

use self::constants::{CARRY_FLAG, NEGATIVE_FLAG, OVERFLOW_FLAG, ZERO_FLAG};

use super::opcodes::OPCODES;

pub struct CPU {
    pub register_a: u8,
    pub register_x: u8,
    pub register_y: u8,
    pub stack_pointer: u8,
    pub flags: u8,
    pub program_counter: u16,
    pub bus: Bus,
    // memory: [u8; 0xFFFF],
}

pub trait Mem {
    fn mem_read(&self, addr: u16) -> u8;
    fn mem_write(&mut self, addr: u16, data: u8);

    fn mem_read_u16(&self, pos: u16) -> u16 {
        let lo = self.mem_read(pos);
        let hi = self.mem_read(pos + 1);

        u16::from_le_bytes([lo, hi])
    }

    fn mem_write_u16(&mut self, pos: u16, data: u16) {
        let [lo, hi] = data.to_le_bytes();
        self.mem_write(pos, lo);
        self.mem_write(pos + 1, hi);
    }
}

#[derive(Debug)]
#[allow(non_camel_case_types)]
pub enum AddressingMode {
    Accumulator,
    Immediate,
    ZeroPage,
    ZeroPage_X,
    ZeroPage_Y,
    Absolute,
    Absolute_X,
    Absolute_Y,
    Indirect,
    Indirect_X,
    Indirect_Y,
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
            stack_pointer: 0,
            flags: 0,
            program_counter: 0,
            bus: Bus::new(),
        }
    }

    pub fn run_with_callback<F>(&mut self, mut callback: F)
    where
        F: FnMut(&mut CPU),
    {
        loop {
            callback(self);

            let opscode = self.mem_read(self.program_counter);
            self.inc_prg();
            let pc_state = self.program_counter;

            let opcode = OPCODES
                .get(&opscode)
                .expect("That is really fucked up opcode");

            match opcode.mnemonic {
                "BRK" => return,
                "ADC" => self.adc(&opcode.mode),
                "AND" => self.and(&opcode.mode),
                "ASL" => self.asl(&opcode.mode),
                "BCC" => self.branch(!self.check_flag(CARRY_FLAG)),
                "BCS" => self.branch(self.check_flag(CARRY_FLAG)),
                "BEQ" => self.branch(self.check_flag(ZERO_FLAG)),
                "BIT" => self.bit(&opcode.mode),
                "BMI" => self.branch(self.check_flag(NEGATIVE_FLAG)),
                "BNE" => self.branch(!self.check_flag(ZERO_FLAG)),
                "BPL" => self.branch(!self.check_flag(NEGATIVE_FLAG)),
                "BVC" => self.branch(!self.check_flag(OVERFLOW_FLAG)),
                "BVS" => self.branch(self.check_flag(OVERFLOW_FLAG)),
                "CLC" => self.remove_flag(CARRY_FLAG),
                "CLD" => self.remove_flag(DECIMAL_MODE),
                "CLI" => self.remove_flag(INTERRUPT_DISABLE),
                "CLV" => self.remove_flag(OVERFLOW_FLAG),
                "CMP" => self.compare(&opcode.mode, self.register_a),
                "CPX" => self.compare(&opcode.mode, self.register_x),
                "CPY" => self.compare(&opcode.mode, self.register_y),
                "DEC" => self.dec(&opcode.mode),
                "DEX" => self.dex(),
                "DEY" => self.dey(),
                "EOR" => self.eor(&opcode.mode),
                "INC" => self.inc(&opcode.mode),
                "INX" => self.inx(),
                "INY" => self.iny(),
                "JMP" => self.jmp(&opcode.mode),
                "JSR" => self.jsr(&opcode.mode),
                "LDA" => self.lda(&opcode.mode),
                "LDX" => self.ldx(&opcode.mode),
                "LDY" => self.ldy(&opcode.mode),
                "LSR" => self.lsr(&opcode.mode),
                "NOP" => (),
                "ORA" => self.ora(&opcode.mode),
                "PHA" => self.push(self.register_a),
                "PHP" => self.push(self.flags),
                "PLA" => self.pla(),
                "PLP" => self.plp(),
                "ROL" => self.rol(&opcode.mode),
                "ROR" => self.ror(&opcode.mode),
                "RTI" => self.rti(),
                "RTS" => self.rts(),
                "SBC" => self.sbc(&opcode.mode),
                "SEC" => self.set_flag(CARRY_FLAG),
                "SED" => self.set_flag(DECIMAL_MODE),
                "SEI" => self.set_flag(INTERRUPT_DISABLE),
                "STA" => self.sta(&opcode.mode),
                "STX" => self.stx(&opcode.mode),
                "STY" => self.sty(&opcode.mode),
                "TAX" => self.tax(),
                "TAY" => self.tay(),
                "TSX" => self.tsx(),
                "TXA" => self.txa(),
                "TXS" => self.txs(),
                "TYA" => self.tya(),
                _ => unreachable!(),
            }

            if self.program_counter == pc_state {
                self.inc_prg_by(opcode.bytes);
            }
        }
    }

    fn adc(&mut self, mode: &AddressingMode) {
        let addr = self.get_operand_address(mode);
        let data = self.mem_read(addr);

        self.register_a = self.add_with_carry(self.register_a, data);
    }

    fn add_with_carry(&mut self, a: u8, b: u8) -> u8 {
        let a_u16 = a as u16;
        let b_u16 = b as u16;
        let carry = self.get_carry() as u16;

        let res = a_u16 + b_u16 + carry;
        let set_carry = res > 0xFF;
        let res_u8 = res as u8;

        let sign_a = a >> 7;
        let sign_b = b >> 7;
        let sign_res = res_u8 >> 7;

        if sign_a == sign_b && sign_res != sign_a {
            self.set_flag(OVERFLOW_FLAG);
        }

        if set_carry {
            self.set_flag(CARRY_FLAG);
        } else {
            self.remove_flag(CARRY_FLAG);
        }

        res_u8
    }

    fn and(&mut self, mode: &AddressingMode) {
        let addr = self.get_operand_address(mode);
        let data = self.mem_read(addr);

        self.register_a &= data;

        self.update_neg_and_zero_status(self.register_a);
    }

    fn asl(&mut self, mode: &AddressingMode) {
        let addr = match mode {
            AddressingMode::Accumulator => {
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

    fn eor(&mut self, mode: &AddressingMode) {
        let addr = self.get_operand_address(mode);
        let data = self.mem_read(addr);

        self.register_a = self.register_a ^ data;
        self.update_neg_and_zero_status(self.register_a);
    }

    fn inc(&mut self, mode: &AddressingMode) {
        let addr = self.get_operand_address(mode);
        let data = self.mem_read(addr);

        let data = data.wrapping_add(1);
        self.mem_write(addr, data);

        self.update_neg_and_zero_status(data);
    }

    fn inx(&mut self) {
        self.register_x = self.register_x.wrapping_add(1);
        self.update_neg_and_zero_status(self.register_x);
    }

    fn iny(&mut self) {
        self.register_y = self.register_y.wrapping_add(1);
        self.update_neg_and_zero_status(self.register_y);
    }

    fn jmp(&mut self, mode: &AddressingMode) {
        let addr = self.get_operand_address(mode);
        // let address = self.mem_read_u16(addr);

        self.program_counter = addr;
    }

    fn jsr(&mut self, mode: &AddressingMode) {
        let return_point = self.program_counter + 2;
        self.push_u16(return_point);

        self.jmp(mode);
    }

    fn lda(&mut self, mode: &AddressingMode) {
        let addr = self.get_operand_address(mode);
        let value = self.mem_read(addr);

        self.register_a = value;
        self.update_neg_and_zero_status(self.register_a);
    }

    fn ldx(&mut self, mode: &AddressingMode) {
        let addr = self.get_operand_address(mode);
        let value = self.mem_read(addr);

        self.register_x = value;
        self.update_neg_and_zero_status(self.register_x);
    }

    fn ldy(&mut self, mode: &AddressingMode) {
        let addr = self.get_operand_address(mode);
        let value = self.mem_read(addr);

        self.register_y = value;
        self.update_neg_and_zero_status(self.register_y);
    }

    fn lsr(&mut self, mode: &AddressingMode) {
        match mode {
            AddressingMode::Accumulator => {
                let res = self.logical_shift_right(self.register_a);
                self.register_a = res;
            }
            _ => {
                let addr = self.get_operand_address(mode);
                let data = self.mem_read(addr);
                let data = self.logical_shift_right(data);
                self.mem_write(addr, data);
            }
        };
    }

    fn ora(&mut self, mode: &AddressingMode) {
        let addr = self.get_operand_address(mode);
        let data = self.mem_read(addr);

        self.register_a = self.register_a | data;
        self.update_neg_and_zero_status(self.register_a);
    }

    fn pla(&mut self) {
        let a = self.pop();
        self.register_a = a;
        self.update_neg_and_zero_status(self.register_a);
    }

    fn plp(&mut self) {
        self.flags = self.pop();
    }

    fn rol(&mut self, mode: &AddressingMode) {
        let addr = match mode {
            AddressingMode::Accumulator => {
                self.register_a = self.rotate_left(self.register_a);
                return;
            }
            _ => self.get_operand_address(mode),
        };

        let data = self.mem_read(addr);
        let data = self.rotate_left(data);
        self.mem_write(addr, data);
    }

    fn ror(&mut self, mode: &AddressingMode) {
        let addr = match mode {
            AddressingMode::Accumulator => {
                self.register_a = self.rotate_right(self.register_a);
                return;
            }
            _ => self.get_operand_address(mode),
        };

        let data = self.mem_read(addr);
        let data = self.rotate_right(data);
        self.mem_write(addr, data);
    }

    fn rti(&mut self) {
        self.flags = self.pop();
        self.program_counter = self.pop_u16();
    }

    fn rts(&mut self) {
        let pc = self.pop_u16();
        self.program_counter = pc;
    }

    fn sbc(&mut self, mode: &AddressingMode) {
        let addr = self.get_operand_address(mode);
        let data = self.mem_read(addr);

        self.register_a = self.add_with_carry(
            self.register_a,
            (data as i8).wrapping_neg().wrapping_sub(1) as u8,
        );
    }

    fn stx(&mut self, mode: &AddressingMode) {
        let addr = self.get_operand_address(mode);
        self.mem_write(addr, self.register_x);
    }

    fn sty(&mut self, mode: &AddressingMode) {
        let addr = self.get_operand_address(mode);
        self.mem_write(addr, self.register_y);
    }

    fn sta(&mut self, mode: &AddressingMode) {
        let addr = self.get_operand_address(mode);
        self.mem_write(addr, self.register_a);
    }

    fn tax(&mut self) {
        self.register_x = self.register_a;
        self.update_neg_and_zero_status(self.register_x);
    }

    fn tay(&mut self) {
        self.register_y = self.register_a;
        self.update_neg_and_zero_status(self.register_y);
    }

    fn tsx(&mut self) {
        self.register_x = self.stack_pointer;
        self.update_neg_and_zero_status(self.register_x);
    }

    fn txa(&mut self) {
        self.register_a = self.register_x;
        self.update_neg_and_zero_status(self.register_a);
    }

    fn txs(&mut self) {
        self.stack_pointer = self.register_x;
    }

    fn tya(&mut self) {
        self.register_a = self.register_y;
        self.update_neg_and_zero_status(self.register_a);
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
            AddressingMode::Indirect => {
                let ptr = self.mem_read_u16(self.program_counter);

                let lo = self.mem_read(ptr);
                let hi = self.mem_read(ptr + 1);

                u16::from_le_bytes([lo, hi])
            }
            AddressingMode::Indirect_X => {
                let base = self.mem_read(self.program_counter);

                let ptr: u8 = base.wrapping_add(self.register_x);
                let lo = self.mem_read(ptr as u16);
                let hi = self.mem_read(ptr as u16 + 1);
                u16::from_le_bytes([lo, hi])
            }
            AddressingMode::Indirect_Y => {
                let base = self.mem_read(self.program_counter);

                let ptr: u8 = base.wrapping_add(self.register_y);
                let lo = self.mem_read(ptr as u16);
                let hi = self.mem_read(ptr as u16 + 1);
                u16::from_le_bytes([lo, hi])
            }
            _ => panic!("mode {mode:?} is not supported"),
        }
    }

    fn push(&mut self, data: u8) {
        let stack_top = 0x100 + self.stack_pointer as u16;
        self.stack_pointer = self.stack_pointer.checked_sub(1).expect("Stack overflow");
        self.mem_write(stack_top, data);
    }

    fn pop(&mut self) -> u8 {
        let stack_top = 0x100 + self.stack_pointer as u16 + 1;
        self.stack_pointer = self.stack_pointer.checked_add(1).expect("Stack underflow");
        self.mem_read(stack_top)
    }

    fn push_u16(&mut self, data: u16) {
        let [lo, hi] = data.to_le_bytes();
        self.push(hi);
        self.push(lo);
    }

    fn pop_u16(&mut self) -> u16 {
        let lo = self.pop();
        let hi = self.pop();
        u16::from_le_bytes([lo, hi])
    }

    fn get_carry(&self) -> u8 {
        if self.check_flag(CARRY_FLAG) {
            1
        } else {
            0
        }
    }

    fn rotate_left(&mut self, data: u8) -> u8 {
        let res = (data << 1) | self.get_carry();

        if data & 0x80 != 0 {
            self.set_flag(CARRY_FLAG);
        } else {
            self.remove_flag(CARRY_FLAG);
        }

        res
    }

    fn rotate_right(&mut self, data: u8) -> u8 {
        let res = (data >> 1) | self.get_carry().rotate_right(1);

        if data & 0x01 != 0 {
            self.set_flag(CARRY_FLAG);
        } else {
            self.remove_flag(CARRY_FLAG);
        }

        res
    }

    #[allow(unused)]
    pub(super) fn get_stack_top(&self) -> u8 {
        self.mem_read(0x100 + self.stack_pointer as u16 + 1)
    }

    #[allow(unused)]
    pub(super) fn get_stack_top_u16(&self) -> u16 {
        self.mem_read_u16(0x100 + self.stack_pointer as u16)
    }

    fn logical_shift_right(&mut self, data: u8) -> u8 {
        if data & 0x01 != 0x00 {
            self.set_flag(CARRY_FLAG);
        } else {
            self.remove_flag(CARRY_FLAG);
        }

        let res = data >> 1;
        self.update_neg_and_zero_status(res);

        res
    }

    pub fn run(&mut self) {
        self.run_with_callback(|_| {});
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
        self.stack_pointer = 0xFF;
        self.flags = 0;

        self.program_counter = self.mem_read_u16(0xFFFC);
    }

    pub fn load(&mut self, program: Vec<u8>) {
        self.memory[0x0600..(0x0600 + program.len())].copy_from_slice(&program[..]);
        self.mem_write_u16(0xFFFC, 0x0600);
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

impl Mem for CPU {
    fn mem_read(&self, addr: u16) -> u8 {
        self.bus.mem_read(addr)
    }

    fn mem_write(&mut self, addr: u16, data: u8) {
        self.bus.mem_write(addr, data)
    }

    fn mem_read_u16(&self, pos: u16) -> u16 {
        self.bus.mem_read_u16(pos)
    }

    fn mem_write_u16(&mut self, pos: u16, data: u16) {
        self.bus.mem_write_u16(pos, data)
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
