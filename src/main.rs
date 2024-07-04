use nes_emulator::cpu::{cpu::trace, CPU};
use std::{
    fs::File,
    io::{self, BufRead, BufReader, Read},
};

fn main() -> io::Result<()> {
    let mut buf = vec![];

    let mut nestest = File::open("roms/nestest.nes")?;
    let mut log = BufReader::new(File::open("roms/nestest.log")?);
    nestest.read_to_end(&mut buf)?;

    let mut cpu = CPU::load_rom(buf).unwrap();

    cpu.reset();
    cpu.program_counter = 0xC000;
    let mut buf = String::new();
    let mut idx = 0;
    cpu.run_with_callback(move |cpu| {
        idx += 1;
        println!("{idx}");
        let trace = trace(cpu);
        log.read_line(&mut buf).unwrap();
        let (test, _) = buf.split_at(73);
        if trace != test {
            println!("{idx} MY TEST:  {trace}");
            println!("{idx} STANDART: {test}");
            std::process::exit(0);
        }
        buf.clear();
    });

    println!("Tests succesful");

    Ok(())
}
