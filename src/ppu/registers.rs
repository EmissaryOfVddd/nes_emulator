use bitflags::bitflags;

bitflags! {
    // 7  bit  0
    // ---- ----
    // VPHB SINN
    // |||| ||||
    // |||| ||++- Base nametable address
    // |||| ||    (0 = $2000; 1 = $2400; 2 = $2800; 3 = $2C00)
    // |||| |+--- VRAM address increment per CPU read/write of PPUDATA
    // |||| |     (0: add 1, going across; 1: add 32, going down)
    // |||| +---- Sprite pattern table address for 8x8 sprites
    // ||||       (0: $0000; 1: $1000; ignored in 8x16 mode)
    // |||+------ Background pattern table address (0: $0000; 1: $1000)
    // ||+------- Sprite size (0: 8x8 pixels; 1: 8x16 pixels)
    // |+-------- PPU master/slave select
    // |          (0: read backdrop from EXT pins; 1: output color on EXT pins)
    // +--------- Generate an NMI at the start of the
    //            vertical blanking interval (0: off; 1: on)
    pub struct ControlRegister: u8 {
        const NAMETABLE1              = 0b00000001;
        const NAMETABLE2              = 0b00000010;
        const VRAM_ADD_INCREMENT      = 0b00000100;
        const SPRITE_PATTERN_ADDR     = 0b00001000;
        const BACKGROUND_PATTERN_ADDR  = 0b00010000;
        const SPRITE_SIZE             = 0b00100000;
        const MASTER_SLAVE_SELECT     = 0b01000000;
        const GENERATE_NMI            = 0b10000000;
    }
 }

 impl ControlRegister {
    pub fn new() -> Self {
        ControlRegister::from_bits_truncate(0x00)
    }

    pub fn get_nametable_addr(&self) -> u16 {
        match self.bits() & 0x03 {
            0 => 0x2000,
            1 => 0x2400,
            2 => 0x2800,
            3 => 0x2C00,
            _ => unreachable!(),
        }
    }

    pub fn sprite_pattern_addr(&self) -> u16 {
        if self.contains(ControlRegister::SPRITE_PATTERN_ADDR) {
            0x1000
        } else {
            0x0
        }
    }

    pub fn get_sprite_size(&self) -> u8 {
        if self.contains(ControlRegister::SPRITE_SIZE) { 16 } else { 8 }
    }

    pub fn get_bgrnd_patt_addr(&self) -> u16 {
        if self.contains(ControlRegister::BACKGROUND_PATTERN_ADDR) {
            0x1000
        } else {
            0x0000
        }
    }

    pub fn generate_vblank_nmi(&mut self) -> bool {
        self.contains(ControlRegister::GENERATE_NMI)
    }

    pub fn vram_add_increment(&self) -> u8 {
        if !self.contains(ControlRegister::VRAM_ADD_INCREMENT) {
            1
        } else {
            32
        }
    }

    pub fn update(&mut self, data: u8) {
        self.set(ControlRegister::from_bits_truncate(data), true);
    }
 }

bitflags! {
    // 7  bit  0
    // ---- ----
    // BGRs bMmG
    // |||| ||||
    // |||| |||+- Greyscale (0: normal color, 1: produce a greyscale display)
    // |||| ||+-- 1: Show background in leftmost 8 pixels of screen, 0: Hide
    // |||| |+--- 1: Show sprites in leftmost 8 pixels of screen, 0: Hide
    // |||| +---- 1: Show background
    // |||+------ 1: Show sprites
    // ||+------- Emphasize red (green on PAL/Dendy)
    // |+-------- Emphasize green (red on PAL/Dendy)
    // +--------- Emphasize blue
    pub struct MaskRegister: u8 {
        const GREYSCALE         = 0b00000001;
        const BACKGROUND_CTRL   = 0b00000010;
        const SPRITES_CTRL      = 0b00000100;
        const BACKGROUND_SHOW   = 0b00001000;
        const SPRITES_SHOW      = 0b00010000;
        const RED               = 0b00100000;
        const GREEN             = 0b01000000;
        const BLUE              = 0b10000000;
    }
 }

 impl MaskRegister {
    pub fn new() -> Self {
        MaskRegister::from_bits_truncate(0x00)
    }

    pub fn update(&mut self, data: u8) {
        self.set(MaskRegister::from_bits_truncate(data), true);
    }
 }

 bitflags! {
    // 7  bit  0
    // ---- ----
    // VSO. ....
    // |||| ||||
    // |||+-++++- PPU open bus. Returns stale PPU bus contents.
    // ||+------- Sprite overflow. The intent was for this flag to be set
    // ||         whenever more than eight sprites appear on a scanline, but a
    // ||         hardware bug causes the actual behavior to be more complicated
    // ||         and generate false positives as well as false negatives; see
    // ||         PPU sprite evaluation. This flag is set during sprite
    // ||         evaluation and cleared at dot 1 (the second dot) of the
    // ||         pre-render line.
    // |+-------- Sprite 0 Hit.  Set when a nonzero pixel of sprite 0 overlaps
    // |          a nonzero background pixel; cleared at dot 1 of the pre-render
    // |          line.  Used for raster timing.
    // +--------- Vertical blank has started (0: not in vblank; 1: in vblank).
    //            Set at dot 1 of line 241 (the line *after* the post-render
    //            line); cleared after reading $2002 and at dot 1 of the
    //            pre-render line.
    pub struct StatusRegister: u8 {
        const OPEN_BUS_0        = 0b00000001;
        const OPEN_BUS_1        = 0b00000010;
        const OPEN_BUS_2        = 0b00000100;
        const OPEN_BUS_3        = 0b00001000;
        const OPEN_BUS_4        = 0b00010000;
        const SPRITE_OVERFLOW   = 0b00100000;
        const SPRITE_0_HIT      = 0b01000000;
        const V_BLANK_STARTED   = 0b10000000;
    }
 }

 impl StatusRegister {
    pub fn new() -> Self {
        StatusRegister::from_bits_truncate(0x00)
    }

    pub fn get(&mut self) -> u8 {
        let res = self.bits();
        self.remove(StatusRegister::V_BLANK_STARTED);
        res
    }

    pub fn set_vblank_status(&mut self, status: bool) {
        self.set(StatusRegister::V_BLANK_STARTED, status)
    }

    pub fn is_in_v_blank(&self) -> bool {
        self.contains(StatusRegister::V_BLANK_STARTED)
    }

    pub fn update(&mut self, data: u8) {
        self.set(StatusRegister::from_bits_truncate(data), true);
    }
 }

pub struct AddrRegister {
    value: (u8, u8), // high first, low second
    hi_ptr: bool,
}

impl AddrRegister {
    pub fn new() -> Self {
        Self {
            value: (0, 0),
            hi_ptr: true,
        }
    }

    fn set(&mut self, data: u16) {
        let [hi, lo] = data.to_be_bytes();
        self.value.0 = hi;
        self.value.1 = lo;
    }

    pub fn update(&mut self, data: u8) {
        if self.hi_ptr {
            self.value.0 = data;
        } else {
            self.value.1 = data;
        }
        self.mirror_down();
        
        self.hi_ptr = !self.hi_ptr;
    }

    pub fn increment(&mut self, inc: u8) {
        let lo = self.value.1;
        self.value.1 = self.value.1.wrapping_add(inc);
        if lo > self.value.1 {
            self.value.0 = self.value.0.wrapping_add(1);
        }
        self.mirror_down();
    }

    pub fn reset_latch(&mut self) {
        self.hi_ptr = true;
    }

    pub fn get(&self) -> u16 {
        u16::from_be_bytes([self.value.0, self.value.1])
    }
    
    fn mirror_down(&mut self) {
        if self.get() > 0x3FFF {
            self.set(self.get() & 0b11111111111111); // mirror down
        }
    }
}