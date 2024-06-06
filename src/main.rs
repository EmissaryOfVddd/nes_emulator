use std::{fs::File, io::{self, Read}};
use nes_emulator::cpu::{cpu::trace, CPU};

fn main() -> io::Result<()> {
    let mut buf = vec![];

    let mut nestest = File::open("roms/nestest.nes")?;
    nestest.read_to_end(&mut buf)?;

    let mut cpu = CPU::load_rom(buf).unwrap();

    cpu.reset();
    cpu.program_counter = 0xB00B;
    cpu.run_with_callback(move |cpu|{
        println!("{}", trace(cpu))
    });

    Ok(())
}
