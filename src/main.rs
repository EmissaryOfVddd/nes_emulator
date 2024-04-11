use crate::cpu::{constants, CPU};

mod cpu;

fn main() {
    let table = [
        (80u8, 16u8, 96u8, false),
        (80, 80, 160, true),
        (80, 144, 224, false),
        (80, 208, 32, false),
        (208, 16, 224, false),
        (208, 80, 32, false),
        (208, 144, 96, true),
    ];

    for entry in table {
        let (a, b, ans, val) = entry;
        let (ans_adc, val_adc) = adc(a, b, 0);
        if ans_adc == ans && val == val_adc {
            println!("PASS");
        } else {
            println!("FAIL: {ans_adc}, {val_adc}");
            dbg!(ans_adc, val_adc);
        }
    }
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
