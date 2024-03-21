pub mod cpu;
mod opcodes;
pub use cpu::{CPU, constants};

#[cfg(test)]
mod test {
    use super::cpu::*;

    #[test]
    fn test_0xa9_immediate_load_data() {
        let mut cpu = CPU::new();
        cpu.load_and_run(vec![0xA9, 0x05, 0x00]);
        assert_eq!(cpu.register_a, 0x05);
        assert!(cpu.status & constants::ZERO_FLAG == 0b00);
        assert!(cpu.status & constants::NEGATIVE_FLAG == 0b00);
    }

    #[test]
    fn test_0xa9_lda_zero_flag() {
        let mut cpu = CPU::new();
        cpu.load_and_run(vec![0xA9, 0x00, 0x00]);
        assert!(cpu.status & constants::ZERO_FLAG == 0b10)
    }

    #[test]
    fn test_0xaa_tax_move_a_to_x() {
        let mut cpu = CPU::new();
        cpu.load(vec![0xAA, 0x0]);
        cpu.reset();
        cpu.register_a = 10;
        cpu.run();

        assert_eq!(cpu.register_x, 10);
    }

    #[test]
   fn test_5_ops_working_together() {
       let mut cpu = CPU::new();
       cpu.load_and_run(vec![0xa9, 0xc0, 0xaa, 0xe8, 0x00]);
 
       assert_eq!(cpu.register_x, 0xc1)
   }

    #[test]
    fn test_inx_overflow() {
        let mut cpu = CPU::new();
        cpu.load(vec![0xE8, 0xE8, 0x00]);
        cpu.reset();
        cpu.register_x = 0xff;
        cpu.run();

        assert_eq!(cpu.register_x, 1)
    }

    #[test]
    fn test_lda_from_memory() {
        let mut cpu = CPU::new();
        cpu.mem_write(0x10, 0x55);
 
        cpu.load_and_run(vec![0xa5, 0x10, 0x00]);
 
        assert_eq!(cpu.register_a, 0x55);
    }
}