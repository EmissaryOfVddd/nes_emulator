pub struct CPU {
    pub register_a: u8,
    pub register_x: u8,
    pub register_y: u8,
    pub status: u8,
    pub program_counter: u16,
}

impl CPU {
    pub fn new() -> Self {
        CPU {
            register_a: 0,
            register_x: 0,
            register_y: 0,
            status: 0,
            program_counter: 0,
        }
    }

    pub fn interpret(&mut self, program: Vec<u8>) {
        self.program_counter = 0;

        loop {
            let opscode = program[self.program_counter as usize];
            self.inc_prg();

            match opscode {
                // BRK
                0x00 => return,
                // INX
                0xE8 => self.inx(),
                // LDA
                0xA9 => {
                    let param = program[self.program_counter as usize];
                    self.inc_prg();
                    self.lda(param);
                }
                // TAX
                0xAA => self.tax(),
                _ => todo!(),
            }
        }
    }

    fn lda(&mut self, value: u8) {
        self.register_a = value;
        self.update_neg_and_zero_status(self.register_a);
    }

    fn tax(&mut self) {
        self.register_x = self.register_a;
        self.update_neg_and_zero_status(self.register_x);
    }

    fn inx(&mut self) {
        (self.register_x, _) = self.register_x.overflowing_add(1);
        self.update_neg_and_zero_status(self.register_x);
    }

    fn inc_prg(&mut self) {
        self.program_counter += 1;
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

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_0xa9_immediate_load_data() {
        let mut cpu = CPU::new();
        cpu.interpret(vec![0xA9, 0x05, 0x00]);
        assert_eq!(cpu.register_a, 0x05);
        assert!(cpu.status & constants::ZERO_FLAG == 0b00);
        assert!(cpu.status & constants::NEGATIVE_FLAG == 0b00);
    }

    #[test]
    fn test_0xa9_lda_zero_flag() {
        let mut cpu = CPU::new();
        cpu.interpret(vec![0xA9, 0x00, 0x00]);
        assert!(cpu.status & constants::ZERO_FLAG == 0b10)
    }

    #[test]
    fn test_0xaa_tax_move_a_to_x() {
        let mut cpu = CPU::new();
        cpu.register_a = 10;
        cpu.interpret(vec![0xAA, 0x0]);

        assert_eq!(cpu.register_x, 10);
    }

    #[test]
   fn test_5_ops_working_together() {
       let mut cpu = CPU::new();
       cpu.interpret(vec![0xa9, 0xc0, 0xaa, 0xe8, 0x00]);
 
       assert_eq!(cpu.register_x, 0xc1)
   }

    #[test]
    fn test_inx_overflow() {
        let mut cpu = CPU::new();
        cpu.register_x = 0xff;
        cpu.interpret(vec![0xe8, 0xe8, 0x00]);

        assert_eq!(cpu.register_x, 1)
    }
}
