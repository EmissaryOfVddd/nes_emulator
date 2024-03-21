
use crate::cpu::{constants, CPU};

mod cpu;

fn main() {
    let mut cpu = CPU::new();
    cpu.register_x = 0xff;
    dbg!(cpu.register_x);
    cpu.load_and_run(vec![0xe8, 0xe8, 0x00]);
    assert_eq!(dbg!(cpu.register_x), 1)
}
