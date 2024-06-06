const NES_TAG: &[u8] = &[0x4E, 0x45, 0x53, 0x1A];
const PRG_ROM_PAGE_SIZE: usize = 0x4000;
const CHR_ROM_PAGE_SIZE: usize = 0x2000;

#[derive(Debug, PartialEq)]
pub enum Mirroring {
    Vertical,
    Horizontal,
    FourScreen,
}

pub struct Rom {
    pub prg_rom: Vec<u8>,
    pub chr_rom: Vec<u8>,
    pub mapper: u8,
    pub screen_mirroring: Mirroring,
}

impl Rom {
    pub fn new(raw: &Vec<u8>) -> Result<Rom, String> {
        if &raw[0..4] != NES_TAG {
            return Err("File is not in iNES format".into())
        }

        let prg_rom_size = raw[4] as usize * PRG_ROM_PAGE_SIZE;
        let chr_rom_size = raw[5] as usize * CHR_ROM_PAGE_SIZE;

        let control_byte_1 = raw[6];
        let control_byte_2 = raw[7];

        let _size_prg_8kb = raw[8];

        // cb1 contains 4 lower bits of mapper type
        // cb2 contains 4 upper bits of mapper type
        let mapper = (control_byte_2 & 0xF0) | (control_byte_1 >> 4);

        let version = (control_byte_2 >> 2) & 0x03;
        if version != 0 {
            return Err("Doesn't currently support iNES 2.0 format".into())
        }

        let four_screen = control_byte_1 & 0x08 != 0;
        let vertical_mirroring = control_byte_1 & 0x01 != 0;
        let screen_mirroring = match (four_screen, vertical_mirroring) {
            (true, _) => Mirroring::FourScreen,
            (false, true) => Mirroring::Vertical,
            (false, false) => Mirroring::Horizontal,
        };

        let skip_trainer = control_byte_1 & 0x04 != 0;

        let prg_rom_start = 16 + if skip_trainer { 512 } else { 0 };
        let chr_rom_start = prg_rom_start + prg_rom_size;

        Ok(Rom {
            prg_rom: raw[prg_rom_start..(prg_rom_start + prg_rom_size)].into(),
            chr_rom: raw[chr_rom_start..(chr_rom_start + chr_rom_size)].into(),
            mapper,
            screen_mirroring,
        })
    }
}
