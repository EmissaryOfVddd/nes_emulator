pub mod cpu;
mod opcodes;
pub use cpu::{constants, CPU};

#[cfg(test)]
mod test {
    use crate::cpu::constants::{CARRY_FLAG, NEGATIVE_FLAG, ZERO_FLAG};

    use super::cpu::*;

    #[test]
    fn test_0xa9_immediate_load_data() {
        let mut cpu = CPU::new();
        cpu.load_and_run(vec![0xA9, 0x05, 0x00]);
        assert_eq!(cpu.register_a, 0x05);
        assert!(cpu.flags & constants::ZERO_FLAG == 0b00);
        assert!(cpu.flags & constants::NEGATIVE_FLAG == 0b00);
    }

    #[test]
    fn test_0xa9_lda_zero_flag() {
        let mut cpu = CPU::new();
        cpu.load_and_run(vec![0xA9, 0x00, 0x00]);
        assert!(cpu.flags & constants::ZERO_FLAG == 0b10)
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

    #[test]
    fn test_adc_immediate() {
        let mut cpu = CPU::new();

        cpu.load_and_run(vec![0x69, 0x10, 0x00]);

        assert_eq!(cpu.register_a, 0x10);
    }

    #[test]
    fn test_adc_from_memory() {
        let mut cpu = CPU::new();
        cpu.mem_write(0x10, 0x55);

        cpu.load_and_run(vec![0x65, 0x10, 0x00]);

        assert_eq!(cpu.register_a, 0x55);
    }

    #[test]
    fn test_adc_known_correctnes() {
        let table = [
            (80, 16, 96, false, false),
            (80, 80, 160, true, false),
            (80, 144, 224, false, false),
            (80, 208, 32, false, true),
            (208, 16, 224, false, false),
            (208, 80, 32, false, true),
            (208, 144, 96, true, true),
            (208, 208, 160, false, true),
        ];

        for entry in table {
            let (a, b, ans, overflow, carry) = entry;

            let mut cpu = CPU::new();
            cpu.load(vec![0x69, b, 0x00]);
            cpu.reset();
            cpu.register_a = a;
            cpu.run();

            assert_eq!(cpu.register_a, ans);
            assert_eq!(cpu.check_flag(constants::OVERFLOW_FLAG), overflow);
            assert_eq!(cpu.check_flag(constants::CARRY_FLAG), carry);
        }
    }

    #[test]
    fn test_and_immediate() {
        let mut cpu = CPU::new();

        cpu.load_and_run(vec![0x29, 0b1111_1111, 0x00]);

        assert_eq!(cpu.register_a, 0x0);

        cpu.load(vec![0x29, 0b0000_0100, 0x00]);
        cpu.reset();
        cpu.register_a = 0b1010_0101;
        cpu.run();
        assert_eq!(cpu.register_a, 0b0000_0100);
    }

    #[test]
    fn test_and_from_memory() {
        let mut cpu = CPU::new();

        cpu.mem_write(0x10, 0b1111_1111);
        cpu.load(vec![0x25, 0x10, 0x00]);
        cpu.reset();
        cpu.register_a = 0b1010_0101;
        cpu.run();
        assert_eq!(cpu.register_a, 0b1010_0101);
    }

    #[test]
    fn test_asl_immediate() {
        let mut cpu = CPU::new();
        cpu.load(vec![0x0A, 0x00]);
        cpu.reset();
        cpu.run();
        assert_eq!(cpu.register_a, 0);

        cpu.load(vec![0x0A, 0x00]);
        cpu.reset();
        cpu.register_a = 0b0010_1000;
        cpu.run();
        assert_eq!(cpu.register_a, 0b0101_0000);
        assert_eq!(cpu.check_flag(constants::CARRY_FLAG), false);

        cpu.load(vec![0x0A, 0x00]);
        cpu.reset();
        cpu.register_a = 0b1000_0001;
        cpu.run();
        assert_eq!(cpu.register_a, 0b0000_0010);
        assert_eq!(cpu.check_flag(constants::CARRY_FLAG), true);
    }

    #[test]
    fn test_asl_from_memory() {
        let mut cpu = CPU::new();
        cpu.mem_write(0x10, 0b0010_1000);
        cpu.load_and_run(vec![0x06, 0x10, 0x00]);
        assert_eq!(cpu.mem_read(0x10), 0b0101_0000);
    }

    #[test]
    fn test_bcc() {
        let mut cpu = CPU::new();
        cpu.load_and_run(vec![0xE8, 0x90, 0x02, 0x85, 0x22, 0x00]);
        assert_ne!(cpu.register_a, 0x22);
    }

    #[test]
    fn test_bit() {
        let mut cpu = CPU::new();
        cpu.mem_write(0x10, 0b0000_1010);
        cpu.mem_write(0x11, 0b1000_1111);
        cpu.mem_write(0x12, 0b0100_0101);
        cpu.load_and_run(vec![0xA5, 0x10, 0x24, 0x11, 0x00]);
        assert!(!cpu.check_flag(constants::ZERO_FLAG));
        assert!(!cpu.check_flag(constants::OVERFLOW_FLAG));
        assert!(cpu.check_flag(constants::NEGATIVE_FLAG));

        cpu.load_and_run(vec![0x24, 0x12, 0x00]);
        assert!(cpu.check_flag(constants::ZERO_FLAG));
        assert!(cpu.check_flag(constants::OVERFLOW_FLAG));
        assert!(!cpu.check_flag(constants::NEGATIVE_FLAG));
    }

    #[test]
    fn test_cmp() {
        let mut cpu = CPU::new();
        cpu.load_and_run(vec![0xA9, 0x22, 0xC9, 0x22, 0x00]);
        assert!(cpu.check_flag(ZERO_FLAG));

        cpu.mem_write(0x10, 0x23);
        cpu.load_and_run(vec![0xA9, 0x22, 0xC5, 0x10, 0x00]);
        assert!(cpu.check_flag(NEGATIVE_FLAG));

        cpu.mem_write(0x10, 0x21);
        cpu.load_and_run(vec![0xA9, 0x22, 0xC5, 0x10, 0x00]);
        assert!(cpu.check_flag(CARRY_FLAG));
    }

    #[test]
    fn test_dec() {
        let mut cpu = CPU::new();
        cpu.mem_write(0x10, 0x11);
        cpu.load_and_run(vec![0xC6, 0x10, 0xC6, 0x10, 0x00]);
        assert_eq!(cpu.mem_read(0x10), 0x0F);

        cpu.mem_write(0x10, 0x00);
        cpu.load_and_run(vec![0xC6, 0x10, 0x00]);
        assert_eq!(cpu.mem_read(0x10), 0xFF);
    }

    #[test]
    fn test_eor() {
        let mut cpu = CPU::new();
        cpu.load_and_run(vec![0xA9, 0xFF, 0x49, 0xAA, 0x00]);
        assert_eq!(cpu.register_a, 0xFF ^ 0xAA);
    }

    #[test]
    fn test_inc() {
        let mut cpu = CPU::new();
        cpu.mem_write(0x10, 0xFF);
        cpu.load_and_run(vec![0xE6, 0x10, 0xE6, 0x10, 0x00]);
        assert_eq!(cpu.mem_read(0x10), 0x01);
    }

    #[test]
    fn test_jmp_abs() {
        /*
            LDA #$8
            STA $11
        loop:
            BIT $11
            BNE end
            INC $10
            LDA $10
            JMP loop
        end:
            BRK
        */

        let mut cpu = CPU::new();
        cpu.load_and_run(vec![
            0xA9, 0x08, 0x85, 0x11, 0x24, 0x11, 0xD0, 0x07, 0xE6, 0x10, 0xA5, 0x10, 0x4C, 0x04,
            0x06, 0x00,
        ]);

        assert_eq!(cpu.register_a, 0x08);
    }

    #[test]
    fn test_jmp_indirect() {
        let mut cpu = CPU::new();

        cpu.mem_write_u16(0xCC01, 0x1111);

        cpu.load_and_run(vec![
            0xA9, 0x01, 0x85, 0xF0, 0xA9, 0xCC, 0x85, 0xF1, 0x6C, 0xF0, 0x00,
        ]);
        assert_eq!(cpu.program_counter, 0x1112);
    }

    #[test]
    fn test_jsr() {
        let mut cpu = CPU::new();
        cpu.load_and_run(vec![0x20, 0x03, 0x80, 0x00]);
        dbg!(cpu.stack_pointer);
        assert_eq!(cpu.get_stack_top_u16(), 0x8003);
    }

    #[test]
    fn test_lsr() {
        let mut cpu = CPU::new();
        cpu.load_and_run(vec![0xA9, 0xFF, 0x4A, 0x00]);
        assert_eq!(cpu.register_a, 0x7F);
        assert!(cpu.check_flag(CARRY_FLAG));
    }

    #[test]
    fn test_ora() {
        let mut cpu = CPU::new();
        cpu.load_and_run(vec![0xA9, 0x0F, 0x09, 0x0A, 0x00]);
        assert_eq!(cpu.register_a, 0x0F);
    }

    #[test]
    fn test_pha() {
        let mut cpu = CPU::new();
        cpu.load_and_run(vec![0xA9, 0x55, 0x48, 0x00]);
        assert_eq!(cpu.get_stack_top(), 0x55);
    }

    #[test]
    fn test_pla() {
        let mut cpu = CPU::new();
        cpu.load_and_run(vec![0xA9, 0x55, 0x48, 0xA9, 0x00, 0x68, 0x00]);
        assert_eq!(cpu.register_a, 0x55);
    }

    #[test]
    fn test_plp() {
        let mut cpu = CPU::new();
        cpu.mem_write(0x10, 0xFF);
        cpu.load_and_run(vec![0xE6, 0x10, 0x08, 0xE6, 0x10, 0x28, 0x00]);
        assert!(cpu.check_flag(ZERO_FLAG));
    }

    #[test]
    fn test_rol() {
        let mut cpu = CPU::new();
        cpu.load_and_run(vec![0xA9, 0xFE, 0x2A, 0x00]);
        assert_eq!(cpu.register_a, 0xFC);
        assert!(cpu.check_flag(CARRY_FLAG));
    }

    #[test]
    fn test_ror() {
        let mut cpu = CPU::new();
        cpu.load_and_run(vec![0xA9, 0x7F, 0x6A, 0x00]);
        assert_eq!(cpu.register_a, 0x3F);
        assert!(cpu.check_flag(CARRY_FLAG));
    }

    #[test]
    fn test_rti() {
        let mut cpu = CPU::new();
        cpu.load_and_run(vec![0xA9, 0x33, 0x48, 0x48, 0xA9, 0xFF, 0x08, 0x40, 0x00]);
        assert_eq!(cpu.program_counter, 0x3334);
        assert!(cpu.check_flag(NEGATIVE_FLAG));
    }

    #[test]
    fn test_jsr_rts() {
        let mut cpu = CPU::new();
        cpu.load_and_run(vec![
            0x20, 0x09, 0x80, 0x20, 0x0c, 0x80,
            0x20, 0x12, 0x80, 0xa2, 0x00, 0x80, 0xe8,
            0xe0, 0x05, 0xd0, 0xfb, 0x60, 0x00
        ]);
    }

    #[test]
    fn test_sbc() {
        let mut cpu = CPU::new();
        cpu.load_and_run(vec![0xA9, 0x22, 0xE9, 0x12, 0x00]);
        assert_eq!(cpu.register_a, 0x0F);
    }
}
