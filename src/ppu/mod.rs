use registers::{AddrRegister, ControlRegister, MaskRegister, StatusRegister};

use crate::cartridge::Mirroring;

pub mod registers;

pub struct PPU {
    pub chr_rom: Vec<u8>,
    pub palette_table: [u8; 32],
    pub vram: [u8; 2048],
    pub oam_data: [u8; 256],
    pub mirroring: Mirroring,
    pub ctrl: ControlRegister,
    pub mask: MaskRegister,
    pub status: StatusRegister,

    internal_data_buf: u8,
    addr: AddrRegister,
}

impl PPU {
    pub fn new(chr_rom: Vec<u8>, mirroring: Mirroring) -> Self {
        PPU {
            chr_rom,
            palette_table: [0; 32], //TODO
            vram: [0; 2048],
            oam_data: [0; 256],
            mirroring,
            internal_data_buf: 0,
            addr: AddrRegister::new(),
            mask: MaskRegister::new(),
            status: StatusRegister::new(),
            ctrl: ControlRegister::new(),
        }
    }

    pub fn write_to_ppu_addr(&mut self, data: u8) {
        self.addr.update(data);
    }

    fn increment_vram_addr(&mut self) {
        self.addr.increment(self.ctrl.vram_add_increment());
    }

    pub fn write_to_ctrl(&mut self, data: u8) {
        self.ctrl.update(data);
    }

    pub fn write_to_mask(&mut self, data: u8) {
        self.mask.update(data);
    }

    pub fn read_from_status(&mut self) -> u8 {
        self.status.get()
    }

    pub fn read_to_data(&mut self) -> u8 {
        let addr = self.addr.get();
        self.increment_vram_addr();

        match addr {
            0..=0x1fff => {
                let result = self.internal_data_buf;
                self.internal_data_buf = self.chr_rom[addr as usize];
                result
            }
            0x2000..=0x2fff => {
                let result = self.internal_data_buf;
                self.internal_data_buf = self.vram[self.mirror_vram_addr(addr) as usize];
                result
            }
            0x3000..=0x3eff => panic!("addr space 0x3000..0x3eff is not expected to be used, requested = {} ", addr),
            0x3f00..=0x3fff => self.palette_table[(addr - 0x3f00) as usize],
            _ => panic!("unexpected access to mirrored space {}", addr),
        }
    }

    pub fn write_to_data(&mut self, data: u8) {
        let addr = self.addr.get();

        match addr {
            0..=0x1fff => panic!("attempt writing to chr rom space"),
            0x2000..=0x2fff => {
                self.vram[self.mirror_vram_addr(addr) as usize] = data;
            }
            0x3000..=0x3eff => panic!("addr space 0x3000..0x3eff is not expected to be used, requested = {} ", addr),
            //Addresses $3F10/$3F14/$3F18/$3F1C are mirrors of $3F00/$3F04/$3F08/$3F0C
            0x3f10 | 0x3f14 | 0x3f18 | 0x3f1c => {
                let add_mirror = addr - 0x10;
                self.palette_table[(add_mirror - 0x3f00) as usize] = data;
            }
            0x3f00..=0x3fff => self.palette_table[(addr - 0x3f00) as usize] = data,
            _ => panic!("unexpected access to mirrored space {}", addr),
        }

        self.increment_vram_addr();
    }

    pub fn mirror_vram_addr(&self, addr: u16) -> u16 {
        let mirrored_vram = addr & 0b10111111111111; // mirror down 0x3000-0x3eff to 0x2000 - 0x2eff
        let vram_idx = mirrored_vram - 0x2000;
        let name_table = vram_idx / 0x400;

        match (&self.mirroring, name_table) {
            (Mirroring::Vertical, 2) | (Mirroring::Vertical, 3) => vram_idx - 0x800,
            (Mirroring::Horizontal, 2) => vram_idx - 0x400,
            (Mirroring::Horizontal, 1) => vram_idx - 0x400,
            (Mirroring::Horizontal, 3) => vram_idx - 0x800,
            _ => vram_idx, 
        }
    }
}