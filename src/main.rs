use nes_emulator::cpu::{cpu::trace, CPU};
use std::{
    fs::File,
    io::{self, Read},
};

fn main() -> io::Result<()> {
    let mut buf = vec![];

    let mut nestest = File::open("roms/nestest.nes")?;
    nestest.read_to_end(&mut buf)?;

    let mut cpu = CPU::load_rom(buf).unwrap();

    cpu.reset();
    cpu.program_counter = 0xC000;
    cpu.run_with_callback(move |cpu| println!("{}", trace(cpu)));

    Ok(())
}
