use crate::cpu::{constants, CPU};

mod cpu;

fn main() {
    let mut cpu = CPU::new();
    cpu.interpret(vec![0xA9, 0x05, 0x00]);
    assert_eq!(cpu.register_a, 0x05);
    assert!(cpu.status & constants::ZERO_FLAG == 0b00);
    assert!(cpu.status & constants::NEGATIVE_FLAG == 0b00);
}
