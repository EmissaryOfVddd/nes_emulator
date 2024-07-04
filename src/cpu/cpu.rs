use constants::{BREAK, BREAK_2};
pub const STACK_RESET: u8 = 0xFD;

use crate::{
    bus::bus::Bus,
    cartridge::Rom,
    cpu::constants::{DECIMAL_MODE, INTERRUPT_DISABLE},
};

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
}

pub trait Mem {
    fn mem_read(&self, addr: u16) -> u8;
    fn mem_write(&mut self, addr: u16, data: u8);

    fn mem_read_u16(&self, pos: u16) -> u16 {
        let lo = self.mem_read(pos);
        let hi = self.mem_read(pos.wrapping_add(1));

        u16::from_le_bytes([lo, hi])
    }

    fn mem_write_u16(&mut self, pos: u16, data: u16) {
        let [lo, hi] = data.to_le_bytes();
        self.mem_write(pos, lo);
        self.mem_write(pos.wrapping_add(1), hi);
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
    pub fn load_rom(raw: Vec<u8>) -> Result<Self, String> {
        let rom = Rom::new(&raw)?;

        Ok(CPU {
            register_a: 0,
            register_x: 0,
            register_y: 0,
            stack_pointer: STACK_RESET,
            flags: 0,
            program_counter: 0,
            bus: Bus::new(rom),
        })
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
                .expect(&format!("That is really fucked up opcode: {opscode:02X}"));

            match opcode.mnemonic {
                "BRK" => return,
                "*AAC" => self.aac(&opcode.mode),
                "*SAX" => self.aax(&opcode.mode),
                "ADC" => self.adc(&opcode.mode),
                "AND" => self.and(&opcode.mode),
                "*ARR" => self.arr(&opcode.mode),
                "ASL" => self.asl(&opcode.mode),
                "*ASR" => self.asr(&opcode.mode),
                "*ATX" => self.atx(&opcode.mode),
                "*AXA" => self.axa(&opcode.mode),
                "*AXS" => self.axs(&opcode.mode),
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
                "*DCP" => self.dcp(&opcode.mode),
                "DEC" => self.dec(&opcode.mode),
                "DEX" => self.dex(),
                "DEY" => self.dey(),
                "*DOP" => {
                    let addr = self.get_operand_address(&opcode.mode);
                    let _data = self.mem_read(addr); // Dummy read
                }, // Double NOP
                "EOR" => self.eor(&opcode.mode),
                "INC" => self.inc(&opcode.mode),
                "INX" => self.inx(),
                "INY" => self.iny(),
                "*ISB" => self.isc(&opcode.mode),
                "JMP" => self.jmp(&opcode.mode),
                "JSR" => self.jsr(&opcode.mode),
                "*KIL" => todo!("Я не знаю как оно должно работать"),
                "*LAR" => self.lar(&opcode.mode),
                "*LAX" => self.lax(&opcode.mode),
                "LDA" => self.lda(&opcode.mode),
                "LDX" => self.ldx(&opcode.mode),
                "LDY" => self.ldy(&opcode.mode),
                "LSR" => self.lsr(&opcode.mode),
                "NOP" => (),
                "*NOP" => (),
                "ORA" => self.ora(&opcode.mode),
                "PHA" => self.push(self.register_a),
                "PHP" => {
                    let flags = self.flags;
                    let flags = flags | BREAK | BREAK_2;
                    self.push(flags)
                },
                "PLA" => self.pla(),
                "PLP" => self.plp(),
                "*RLA" => self.rla(&opcode.mode),
                "ROL" => self.rol(&opcode.mode),
                "ROR" => self.ror(&opcode.mode),
                "*RRA" => self.rra(&opcode.mode),
                "RTI" => self.rti(),
                "RTS" => self.rts(),
                "SBC" | "*SBC" => self.sbc(&opcode.mode),
                "SEC" => self.set_flag(CARRY_FLAG),
                "SED" => self.set_flag(DECIMAL_MODE),
                "SEI" => self.set_flag(INTERRUPT_DISABLE),
                "*SLO" => self.slo(&opcode.mode),
                "*SRE" => self.sre(&opcode.mode),
                "STA" => self.sta(&opcode.mode),
                "STX" => self.stx(&opcode.mode),
                "STY" => self.sty(&opcode.mode),
                "*SXA" => self.sxa(&opcode.mode),
                "*SYA" => self.sya(&opcode.mode),
                "TAX" => self.tax(),
                "TAY" => self.tay(),
                "*TOP" => {
                    let addr = self.get_operand_address(&opcode.mode);
                    let _data = self.mem_read(addr); // Dummy read
                }, // Triple NOP
                "TSX" => self.tsx(),
                "TXA" => self.txa(),
                "TXS" => self.txs(),
                "TYA" => self.tya(),
                "*XAS" => self.xas(&opcode.mode),
                _ => unreachable!(),
            }

            if self.program_counter == pc_state {
                self.inc_prg_by(opcode.bytes);
            }
        }
    }

    fn xas(&mut self, mode: &AddressingMode) {
        let addr = self.get_operand_address(mode);
        let [_, hi] = addr.to_le_bytes();

        self.stack_pointer = self.register_a & self.register_x;
        let res = self.stack_pointer & hi.wrapping_add(1);

        self.mem_write(addr, res);
    }

    fn sya(&mut self, mode: &AddressingMode) {
        let addr = self.get_operand_address(mode);
        
        let [_, hi] = addr.to_le_bytes();
        let res = self.register_y & hi.wrapping_add(1);

        self.mem_write(addr, res);
    }

    fn sxa(&mut self, mode: &AddressingMode) {
        let addr = self.get_operand_address(mode);
        
        let [_, hi] = addr.to_le_bytes();
        let res = self.register_x & hi.wrapping_add(1);

        self.mem_write(addr, res);
    }

    fn sre(&mut self, mode: &AddressingMode) {
        let addr = self.get_operand_address(mode);
        let data = self.mem_read(addr);

        if data & 1 != 0 {
            self.set_flag(CARRY_FLAG);
        } else {
            self.remove_flag(CARRY_FLAG);
        }

        let res = data >> 1;
        self.mem_write(addr, res);
        
        self.register_a = self.register_a ^ res;
        self.update_neg_and_zero_status(self.register_a);
    }

    fn slo(&mut self, mode: &AddressingMode) {
        let addr = self.get_operand_address(mode);
        let data = self.mem_read(addr);

        if data & 0x80 != 0 {
            self.set_flag(CARRY_FLAG);
        } else {
            self.remove_flag(CARRY_FLAG);
        }

        let res = data << 1;
        self.mem_write(addr, res);
        
        self.register_a = self.register_a | res;
        self.update_neg_and_zero_status(self.register_a);
    }

    fn rra(&mut self, mode: &AddressingMode) {
        let addr = self.get_operand_address(mode);
        let data = self.mem_read(addr);
        let old_carry = self.get_carry();

        if data & 0x01 != 0 {
            self.set_flag(CARRY_FLAG);
        } else {
            self.remove_flag(CARRY_FLAG);
        }

        let mut res = data >> 1;
        if old_carry == 1 {
            res |= 0x80;
        }

        self.mem_write(addr, res);
        self.register_a = self.add_with_carry(self.register_a, res);
    }

    fn rla(&mut self, mode: &AddressingMode) {
        let addr = self.get_operand_address(mode);
        let mut data = self.mem_read(addr);

        let old_carry = self.get_carry();

        if data >> 7 == 1 {
            self.set_flag(CARRY_FLAG);
        } else {
            self.remove_flag(CARRY_FLAG);
        }
        data = data << 1;
        data = data | old_carry;
        self.mem_write(addr, data);

        self.register_a = self.register_a & data;
        self.update_neg_and_zero_status(self.register_a);
    }

    fn lax(&mut self, mode: &AddressingMode) {
        let addr = self.get_operand_address(mode);
        let data = self.mem_read(addr);

        self.update_neg_and_zero_status(data);
        self.register_a = data;
        self.register_x = data;
    }

    fn lar(&mut self, mode: &AddressingMode) {
        let addr = self.get_operand_address(mode);
        let data = self.mem_read(addr);

        let res = data & self.stack_pointer;
        self.update_neg_and_zero_status(res);
        self.register_a = res;
        self.register_x = res;
        self.stack_pointer = res;
    }

    fn isc(&mut self, mode: &AddressingMode) {
        let addr = self.get_operand_address(mode);
        let data = self.mem_read(addr);

        let data = data.wrapping_add(1);
        self.mem_write(addr, data);
        self.register_a = self.subtract_with_carry(self.register_a, data);
    }

    fn dcp(&mut self, mode: &AddressingMode) {
        let addr = self.get_operand_address(mode);
        let data = self.mem_read(addr);

        let res = data.wrapping_sub(1);

        if res <= self.register_a {
            self.set_flag(CARRY_FLAG);
        } else {
            self.remove_flag(CARRY_FLAG);
        }
        self.update_neg_and_zero_status(self.register_a.wrapping_sub(res));

        self.mem_write(addr, res);
    }

    fn axs(&mut self, mode: &AddressingMode) {
        let addr = self.get_operand_address(mode);
        let data = self.mem_read(addr);

        let x_and_a = self.register_x & self.register_a;
        let result = x_and_a.wrapping_sub(data);

        if data <= x_and_a {
            self.set_flag(CARRY_FLAG);
        } else {
            self.remove_flag(CARRY_FLAG);
        }

        self.register_x = result;
        self.update_neg_and_zero_status(result);
    }

    fn axa(&mut self, mode: &AddressingMode) {
        let addr = self.get_operand_address(mode);
        let data = self.register_a & self.register_x & (addr >> 8) as u8;
        self.mem_write(addr, data);
    }

    fn atx(&mut self, mode: &AddressingMode) {
        let addr = self.get_operand_address(mode);
        let data = self.mem_read(addr);

        self.register_a = self.register_a & data;
        self.register_x = self.register_a;
        self.update_neg_and_zero_status(self.register_x);
    }

    fn aac(&mut self, mode: &AddressingMode) {
        let addr = self.get_operand_address(mode);
        let data = self.mem_read(addr);

        let res = self.register_a & data;
        if data & NEGATIVE_FLAG != 0 {
            self.set_flag(CARRY_FLAG);
        } else {
            self.remove_flag(CARRY_FLAG);
        }

        self.update_neg_and_zero_status(res);
    }

    fn aax(&mut self, mode: &AddressingMode) {
        0xe6;
        0x64;
        let addr = self.get_operand_address(mode);
        
        let res = self.register_a & self.register_x;
        self.mem_write(addr, res);
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
        } else {
            self.remove_flag(OVERFLOW_FLAG);
        }

        if set_carry {
            self.set_flag(CARRY_FLAG);
        } else {
            self.remove_flag(CARRY_FLAG);
        }

        self.update_neg_and_zero_status(res_u8);

        res_u8
    }

    fn and(&mut self, mode: &AddressingMode) {
        let addr = self.get_operand_address(mode);
        let data = self.mem_read(addr);

        self.register_a &= data;

        self.update_neg_and_zero_status(self.register_a);
    }

    fn arr(&mut self, mode: &AddressingMode) {
        let addr = self.get_operand_address(mode);
        let data = self.mem_read(addr);

        let res = self.register_a & data;
        let res = res.rotate_right(1);

        let (a, b) = (res & 0x10, res & 0x20);
        match (a, b) {
            (0, 0) => {
                self.remove_flag(CARRY_FLAG);
                self.remove_flag(OVERFLOW_FLAG);
            }
            (_, 0) => {
                self.set_flag(OVERFLOW_FLAG);
                self.set_flag(CARRY_FLAG);
            }
            (0, _) => {
                self.remove_flag(CARRY_FLAG);
                self.set_flag(OVERFLOW_FLAG);
            }
            (_, _) => {
                self.set_flag(CARRY_FLAG);
                self.remove_flag(OVERFLOW_FLAG);
            }
        }

        self.update_neg_and_zero_status(res);
        self.register_a = res;
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

    fn asr(&mut self, mode: &AddressingMode) {
        let addr = self.get_operand_address(mode);
        let data = self.mem_read(addr);

        let res = self.register_a & data;
        let res = res >> 1;
        
        self.update_neg_and_zero_status(res);
        if res & CARRY_FLAG != 0 {
            self.set_flag(CARRY_FLAG);
        } else {
            self.remove_flag(CARRY_FLAG);
        }

        self.register_a = res;
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
        self.update_neg_and_zero_status(res);

        if data & OVERFLOW_FLAG == 0 {
            self.remove_flag(OVERFLOW_FLAG);
        } else {
            self.set_flag(OVERFLOW_FLAG);
        }

        if data & NEGATIVE_FLAG == 0 {
            self.remove_flag(NEGATIVE_FLAG);
        } else {
            self.set_flag(NEGATIVE_FLAG);
        }
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
        self.register_y = self.register_y.wrapping_sub(1);
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
        self.push_u16(return_point - 1);

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
        self.remove_flag(BREAK);
        self.set_flag(BREAK_2);
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
        self.set_flag(BREAK_2);
        self.program_counter = self.pop_u16();
    }

    fn rts(&mut self) {
        let pc = self.pop_u16() + 1;
        self.program_counter = pc;
    }

    fn subtract_with_carry(&mut self, a: u8, b: u8) -> u8 {
        self.add_with_carry(a, (b as i8).wrapping_neg().wrapping_sub(1) as u8)
    }

    fn sbc(&mut self, mode: &AddressingMode) {
        let addr = self.get_operand_address(mode);
        let data = self.mem_read(addr);

        self.register_a = self.subtract_with_carry(self.register_a, data);
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
        // dbg!(self.bus.mem_read(0x01));
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
                let hi = if ptr & 0xFF == 0xFF {
                    self.mem_read(ptr & 0xFF00)
                } else {
                    self.mem_read(ptr + 1)
                };

                u16::from_le_bytes([lo, hi])
            }
            AddressingMode::Indirect_X => {
                let base = self.mem_read(self.program_counter);

                let ptr: u8 = base.wrapping_add(self.register_x);
                let lo = self.mem_read(ptr as u16);
                let hi = self.mem_read(ptr.wrapping_add(1) as u16);
                u16::from_le_bytes([lo, hi])
            }
            AddressingMode::Indirect_Y => {
                let base = self.mem_read(self.program_counter);

                let lo = self.mem_read(base as u16);
                let hi = self.mem_read(base.wrapping_add(1) as u16);
                let ptr = u16::from_le_bytes([lo, hi]);
                let real_addr = ptr.wrapping_add(self.register_y as u16);
                real_addr
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

        self.update_neg_and_zero_status(res);

        res
    }

    fn rotate_right(&mut self, data: u8) -> u8 {
        let res = (data >> 1) | self.get_carry().rotate_right(1);

        if data & 0x01 != 0 {
            self.set_flag(CARRY_FLAG);
        } else {
            self.remove_flag(CARRY_FLAG);
        }

        self.update_neg_and_zero_status(res);

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

    pub fn _run(&mut self) {
        self.run_with_callback(|_| {});
    }

    pub fn _load_and_run(&mut self, program: Vec<u8>) {
        self._load(program);
        self.reset();
        self._run()
    }

    pub fn reset(&mut self) {
        self.register_a = 0;
        self.register_x = 0;
        self.register_y = 0;
        self.stack_pointer = STACK_RESET;
        self.set_flag(INTERRUPT_DISABLE | BREAK_2);

        self.program_counter = self.mem_read_u16(0xFFFC);
    }

    pub fn _load(&mut self, program: Vec<u8>) {
        for (id, val) in program.iter().enumerate() {
            self.bus.mem_write(0x0600 + id as u16, *val);
        }
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
    pub const BREAK: u8 = 0b0001_0000;
    pub const BREAK_2: u8 = 0b0010_0000;
    pub const OVERFLOW_FLAG: u8 = 0b0100_0000;
    pub const NEGATIVE_FLAG: u8 = 0b1000_0000;
}

pub fn trace(cpu: &CPU) -> String {
    let pc = format!("{:04X}", cpu.program_counter);

    let opscode = cpu.mem_read(cpu.program_counter);
    let opcode = OPCODES
        .get(&opscode)
        .expect(&format!("That is really fucked up opcode: {opscode:02X}"));

    let mut real_addr = String::new();

    let bytes = match opcode.bytes {
        1 => {
            match opcode.mode {
                AddressingMode::Accumulator => real_addr = format!("A"),
                _ => ()
            };
            format!("{:02X}", opcode.code)
        },
        2 => {
            let second_arg = cpu.mem_read(cpu.program_counter + 1);
            real_addr = match opcode.mode {
                AddressingMode::Immediate => format!("#${:02X}", second_arg),
                AddressingMode::ZeroPage => {
                    let val = cpu.mem_read(second_arg as _);
                    format!("${second_arg:02X} = {:02X}", val)
                },
                AddressingMode::ZeroPage_X => {
                    let addr = second_arg.wrapping_add(cpu.register_x);
                    format!("${second_arg:02X},X @ {addr:02X} = {:02X}", cpu.mem_read(addr as u16))
                }
                AddressingMode::ZeroPage_Y => {
                    let addr = second_arg.wrapping_add(cpu.register_y);
                    format!("${second_arg:02X},Y @ {addr:02X} = {:02X}", cpu.mem_read(addr as u16))
                }
                AddressingMode::Relative => {
                    let offset = second_arg as u16;
                    let absolute_addr = cpu.program_counter.wrapping_add(2).wrapping_add((offset as i8) as u16);
                    format!("${:04X}", absolute_addr)
                }
                AddressingMode::Indirect_X => {
                    let base = second_arg;

                    let ptr = base.wrapping_add(cpu.register_x);
                    let lo = cpu.mem_read(ptr as u16);
                    let hi = cpu.mem_read(ptr.wrapping_add(1) as u16);
                    let real_addr = u16::from_le_bytes([lo, hi]);
                    let val = cpu.mem_read(real_addr);

                    format!("(${base:02X},X) @ {ptr:02X} = {real_addr:04X} = {val:02X}")
                }
                AddressingMode::Indirect_Y => {
                    let base = second_arg;

                    let lo = cpu.mem_read(base as u16);
                    let hi = cpu.mem_read(base.wrapping_add(1) as u16);
                    let ptr = u16::from_le_bytes([lo, hi]);
                    let real_addr = ptr.wrapping_add(cpu.register_y as u16);
                    let contents = cpu.mem_read(real_addr);

                    format!("(${base:02X}),Y = {ptr:04X} @ {real_addr:04X} = {contents:02X}")
                }
                _ => unreachable!("{:?}", opcode.mode),
            };

            format!("{:02X} {:02X}", opcode.code, second_arg)
        }
        3 => {
            let second_arg = cpu.mem_read(cpu.program_counter + 1);
            let third_arg = cpu.mem_read(cpu.program_counter + 2);

            real_addr = match opcode.mode {
                AddressingMode::Absolute => {
                    if opcode.mnemonic != "JMP" && opcode.mnemonic != "JSR" {
                        let val = cpu.mem_read(u16::from_le_bytes([second_arg, third_arg]));
                        format!("${third_arg:02X}{second_arg:02X} = {val:02X}")
                    } else {
                        format!("${third_arg:02X}{second_arg:02X}")
                    }
                },
                AddressingMode::Absolute_X => {
                    let base = u16::from_le_bytes([second_arg, third_arg]);
                    let real_addr = base.wrapping_add(cpu.register_x as u16);
                    let contents = cpu.mem_read(real_addr);

                    format!("${base:04X},X @ {real_addr:04X} = {contents:02X}")
                }
                AddressingMode::Absolute_Y => {
                    let base = u16::from_le_bytes([second_arg, third_arg]);
                    let real_addr = base.wrapping_add(cpu.register_y as u16);
                    let contents = cpu.mem_read(real_addr);

                    format!("${base:04X},Y @ {real_addr:04X} = {contents:02X}")
                }
                AddressingMode::Indirect => {
                    let base = u16::from_le_bytes([second_arg, third_arg]);
                    let lo = cpu.mem_read(base);
                    let hi = if base & 0xFF == 0xFF {
                        cpu.mem_read(base & 0xFF00)
                    } else {
                        cpu.mem_read(base + 1)
                    };
                    let real_addr = u16::from_le_bytes([lo, hi]);

                    format!("(${base:04X}) = {real_addr:04X}")
                }
                _ => unreachable!("{:?}", opcode.mode),
            };

            format!("{:02X} {second_arg:02X} {third_arg:02X}", opcode.code)
        }
        _ => unreachable!(),
    };

    // Костыль потому что нет единой конвенции по наименованию
    // неофициальных опкодов
    let mnemonic = match opcode.mnemonic {
        "*DOP" | "*TOP" => "*NOP",
        _ => opcode.mnemonic,
    };

    format!(
        "{pc:6}{bytes:9}{mnemonic:>4} {real_addr:28}A:{a:02X} X:{x:02X} Y:{y:02X} P:{p:02X} SP:{sp:02X}",
        a = cpu.register_a,
        x = cpu.register_x,
        y = cpu.register_y,
        p = cpu.flags,
        sp = cpu.stack_pointer,
    )
}
