use crate::cpu::{constants, CPU};

mod cpu;

fn main() {
    let mut cpu = CPU::new();
    cpu.load_and_run(vec![]);
}

fn adc(a: u8, b: u8, c: u8) -> (u8, bool) {
    let a = a as u16;
    let b = b as u16;
    let c = c as u16;

    let res = a + b + c;
    let res_u8 = res as u8;
    let sign_a = a >> 7;
    let sign_b = b >> 7;
    let sign_r = res_u8 >> 7;
    if sign_a == sign_b && sign_r as u16 != sign_a {
        return (res_u8, true);
    }

    return (res_u8, false);
}
