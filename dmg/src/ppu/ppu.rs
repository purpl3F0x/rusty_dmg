use super::*;
use crate::memory::io_registers::*;
use crate::memory::RegisterTrait;

use log::*;

pub const VRAM_START: u16 = 0x8000;
pub const VRAM_END: u16 = 0x9FFF;
pub const OAM_START: u16 = 0xFE00;
pub const OAM_END: u16 = 0xFE9F;

impl<T> OAMEntry<T> {
    pub fn get_y(&self) -> i32
    where
        T: AsRef<[u8]>,
    {
        self.y() as i32 - 16
    }

    pub fn get_x(&self) -> i32
    where
        T: AsRef<[u8]>,
    {
        self.x() as i32 - 8
    }
}

impl PPU {
    pub fn new(ic: Rc<RefCell<InterruptController>>) -> Self {
        PPU {
            mode: PPUMode::HBlank,
            oam: [0; 0xA0],
            vram: [0; 0x2000],
            lcd_control: LCDControlRegister(0),
            lcd_status: LCDStatus(0),
            scroll_y: 0,
            scroll_x: 0,
            ly: 0x00,
            ly_compare: 0,
            window_y: 0,
            window_x: 0,
            bg_palette: BGPalette(0),
            obj_palette_0: BGPalette(0),
            obj_palette_1: BGPalette(0),
            scan_line: 0,
            frame_counter: 0,
            line_counter: 0,
            frame_buffer: [Color32::RGB(0x9a, 0x9e, 0x3f); 160 * 144],
            obj_scanline: [OAMEntry([0, 0, 0, 0]); 10],
            frame_ready: false,
            t_cycles: 0,

            ic: ic,
        }
    }

    fn get_tile_base_address(&self) -> u16 {
        match self.lcd_control.bg_window_tile_data_select() {
            false => 0x9000,
            true => 0x8000,
        }
    }

    fn get_map_base_address(&self) -> u16 {
        match self.lcd_control.bg_tile_map_display_select() {
            false => 0x9800,
            true => 0x9C00,
        }
    }

    #[inline(always)]
    fn bgp_lut(&self, pallete: &BGPalette, id: u8) -> Color32 {
        let value = match id & 0b11 {
            0 => pallete.id0(),
            1 => pallete.id1(),
            2 => pallete.id2(),
            3 => pallete.id3(),
            _ => unreachable!(),
        };

        match value {
            0 => Color32::RGB(155, 188, 15),
            1 => Color32::RGB(139, 172, 15),
            2 => Color32::RGB(48, 98, 48),
            3 => Color32::RGB(15, 56, 15),
            _ => unreachable!(),
        }
    }

    pub fn set_lcd_control(&mut self, value: u8) {
        let is_lcd_enabled = self.lcd_control.lcd_enable();

        self.lcd_control = LCDControlRegister(value);

        if is_lcd_enabled && !self.lcd_control.lcd_enable() {
            // Disabling LCD
            debug!("LCD disabled");

            // Reset the scan line and mode

            self.frame_counter = 0;
            self.line_counter = 0;
            self.scan_line = 0;
            self.ly = 0;
            self.mode = PPUMode::VBlank;
            self.lcd_status.set_ppu_mode(0);
            // self.lcd_status.set_ppu_mode(0);

            if self.mode != PPUMode::VBlank {
                warn!("The screen shouldn't turn off while not in VBLANK");
            }

            self.frame_buffer.fill(Color32::RGB(202, 220, 159));
            self.scan_line = 0;
        } else if !is_lcd_enabled && self.lcd_control.lcd_enable() {
            // Enabling LCD
            debug!("LCD enabled");

            self.update_ppu_mode(PPUMode::OAMSearch);
        }
    }

    #[inline]
    pub fn update_ppu_mode(&mut self, mode: PPUMode) {
        if self.mode != mode {
            self.mode = mode;
            self.lcd_status.set_ppu_mode(self.mode as u8);

            let mut ic_mut = self.ic.borrow_mut();

            match self.mode {
                PPUMode::HBlank => {
                    if self.lcd_status.mode0_int_select() {
                        ic_mut.interrupt_flag.set_lcd(true);
                    }
                }
                PPUMode::VBlank => {
                    ic_mut.interrupt_flag.set_vblank(true);

                    if self.lcd_status.mode1_int_select() {
                        ic_mut.interrupt_flag.set_lcd(true);
                    }
                }
                PPUMode::OAMSearch => {
                    if self.lcd_status.mode2_int_select() {
                        ic_mut.interrupt_flag.set_lcd(true);
                    }
                }
                PPUMode::PixelTransfer => {}
            }
        }
    }

    pub fn tick(&mut self) {
        // fake that the ppu does something

        if self.lcd_control.lcd_enable() == false {
            return;
        }

        self.t_cycles = self.t_cycles.wrapping_add(1);
        self.frame_counter += 1;
        self.line_counter += 1;

        if self.line_counter == 80 && self.ly < 144 {
            self.update_ppu_mode(PPUMode::PixelTransfer);
        } else if self.line_counter == (80 + 172) && self.ly < 144 {
            self.update_ppu_mode(PPUMode::HBlank);
            self.render_scanline();
            self.render_sprites();
        } else if self.line_counter == 456 {
            self.ly += 1;
            self.line_counter = 0;
            // Update LYC and trinnger LYC interrupt if needed
            self.lcd_status.set_lyc_flag(self.ly == self.ly_compare);
            if self.lcd_status.lyc_flag() && self.lcd_status.lyc_int_select() {
                self.ic.borrow_mut().interrupt_flag.set_lcd(true);
            }

            if self.ly < 144 {
                self.update_ppu_mode(PPUMode::OAMSearch);
            } else {
                self.update_ppu_mode(PPUMode::VBlank);
            }
        }

        if self.frame_counter == 70224 {
            self.ly = 0;
            self.frame_counter = 0;
            self.frame_ready = true;
            self.update_ppu_mode(PPUMode::OAMSearch);
        }
    }

    pub fn render_scanline(&mut self) {
        if self.lcd_control.lcd_enable() == false {
            return;
        }

        let y = self.ly.wrapping_add(self.scroll_y) as i32; // TODO: This needs wrapping_add
        let current_tile = y / 8;
        let tile_y = y % 8;

        let bg_map_base = self.get_map_base_address() as i32;
        let bg_tile_base = self.get_tile_base_address() as i32;

        for x in 0..160 {
            let x_pixel = (x as u8).wrapping_add(self.scroll_x) as i32;
            let tile_index: u8 =
                self.vram_read((bg_map_base + (current_tile * 32 + x_pixel / 8)) as u16);

            let tile_index = if bg_tile_base == 0x9000 {
                tile_index as i8 as i32
            } else {
                tile_index as u8 as i32
            };

            let v0 = self.vram_read((bg_tile_base + (tile_index * 16) + tile_y * 2) as u16);
            let v1 = self.vram_read((bg_tile_base + (tile_index * 16) + tile_y * 2 + 1) as u16);

            let tile_x = x_pixel % 8;
            // for tile_x in 0..8 {
            let p1 = (v0 >> (7 - tile_x)) & 1;
            let p2 = (v1 >> (7 - tile_x)) & 1;
            let color_index = (p1 << 1) | p2;

            let color = self.bgp_lut(&self.bg_palette, color_index);

            let idx = (self.ly as i32) * 160 + x as i32;

            self.frame_buffer[idx as usize] = color;
            // }
        }
    }

    pub fn render_sprites(&mut self) {
        if self.lcd_control.lcd_enable() == false || self.lcd_control.obj_display_enable() == false
        {
            return;
        }

        let mut sprites_found = 0; // We can only render 10 sprites per scanline

        let sprit_height = if self.lcd_control.obj_size() { 16 } else { 8 };
        let mask = if self.lcd_control.obj_size() {
            0xFE
        } else {
            0xFF
        };

        for i in 0..40 {
            let oam_entry: OAMEntry<[u8; 4]> =
                OAMEntry(self.oam[i * 4..i * 4 + 4].try_into().unwrap());
            let y = oam_entry.get_y();

            if y <= self.ly.into() && y + sprit_height > self.ly.into() {
                self.obj_scanline[sprites_found] = oam_entry;
                sprites_found += 1;
                if sprites_found == 10 {
                    break;
                }
            }
        }

        // Sort sprites by x coordinate, to emulate sprite priority
        self.obj_scanline[0..sprites_found].sort_by_key(|entry| entry.x());

        for idx in (0..sprites_found).rev() {
            let s = &(self.obj_scanline[idx]);

            let tile_row = if s.y_flip() {
                sprit_height - 1 - (self.ly as i32 - s.get_y())
            } else {
                self.ly as i32 - s.get_y()
            };

            for x in 0..8 {
                let pixel_x = s.get_x() + x;

                if pixel_x < 0 || pixel_x >= 160 {
                    continue;
                }
                let x_idx = if s.x_flip() { 7 - x } else { x };

                let tile_addr = ((s.tile_index() & mask) as i32) * 16 + (tile_row * 2);

                let lo = self.vram[tile_addr as usize] << x_idx;
                let hi = self.vram[tile_addr as usize + 1] << x_idx;

                let color_index = ((lo >> 7) & 0b1) | ((hi >> 6) & 0b10);

                let color = if s.dmg_palette() == false {
                    self.bgp_lut(&self.obj_palette_0, color_index)
                } else {
                    self.bgp_lut(&self.obj_palette_1, color_index)
                };

                let idx = (self.ly as i32) * 160 + pixel_x;
                self.frame_buffer[idx as usize] = color;
            }
        }
    }

    pub fn render_bg_debug(&self, frame_buffer: &mut [Color32]) {
        let bg_map_base = self.get_map_base_address() as i32;
        let bg_tile_base = self.get_tile_base_address() as i32;

        for y in 0..32 {
            for x in 0..32 {
                let tile_index: u8 = self.vram_read((bg_map_base + (y * 32 + x)) as u16);

                let tile_index = if bg_tile_base == 0x9000 {
                    tile_index as i8 as i32
                } else {
                    tile_index as u8 as i32
                };

                for tile_y in 0..8 {
                    let v0 = self.vram_read((bg_tile_base + (tile_index * 16) + tile_y * 2) as u16);
                    let v1 =
                        self.vram_read((bg_tile_base + (tile_index * 16) + tile_y * 2 + 1) as u16);

                    for tile_x in 0..8 {
                        let p1 = (v0 >> (7 - tile_x)) & 1;
                        let p2 = (v1 >> (7 - tile_x)) & 1;
                        let color_index = (p1 << 1) | p2;

                        let color = self.bgp_lut(&self.bg_palette, color_index);

                        let idx_x = x * 8 + tile_x;
                        let idx_y = y * 8 + tile_y;
                        let idx = idx_y * 256 + idx_x;
                        frame_buffer[idx as usize] = color;
                    }
                }
            }
        }

        // overlay viewport rectangle
        let viewport_x = self.scroll_x as usize;
        let viewport_y = self.scroll_y as usize;
        let viewport_width = 160; // Assuming 160 pixels wide
        let viewport_height = 144; // Assuming 144 pixels tall
        for y in 0..viewport_height {
            if y == 0 || y == viewport_height - 1 {
                // Draw top and bottom lines
                for x in 0..viewport_width {
                    let idx = (viewport_y + y) * 256 + (viewport_x + x);
                    if idx < frame_buffer.len() {
                        frame_buffer[idx] = Color32::RGB(0xFF, 0x00, 0x00); // Red color for viewport
                    }
                }
            } else {
                // Draw left and right lines
                let left_idx = (viewport_y + y) * 256 + viewport_x;
                let right_idx = (viewport_y + y) * 256 + (viewport_x + viewport_width - 1);
                if left_idx < frame_buffer.len() {
                    frame_buffer[left_idx] = Color32::RGB(0xFF, 0x00, 0x00); // Red color for viewport
                }
                if right_idx < frame_buffer.len() {
                    frame_buffer[right_idx] = Color32::RGB(0xFF, 0x00, 0x00); // Red color for viewport
                }
            }
        }

        // Draw the grid
        for y in 0..256 {
            for x in 0..256 {
                if x % 8 == 0 || y % 8 == 0 {
                    let idx = y * 256 + x;
                    if idx < frame_buffer.len() {
                        let mut r = frame_buffer[idx].r;
                        let mut g = frame_buffer[idx].g;
                        let mut b = frame_buffer[idx].b;

                        r = r / 2;
                        g = g / 2;
                        b = b / 2;
                        frame_buffer[idx] = Color32::RGB(r, g, b);
                    }
                }
            }
        }
    }

    pub fn render_sprites_debug(&mut self, frame_buffer: &mut [Color32]) {
        for i in 0..40 {
            let block_y = i / 5;
            let block_x = i % 5;

            let oam_entry: OAMEntry<[u8; 4]> =
                OAMEntry(self.oam[i * 4..i * 4 + 4].try_into().unwrap());

            let data_addr = 0x8000 + oam_entry.tile_index() as u16 * 16;

            for row in 0..8 {
                let v0 = self.vram_read(data_addr + row * 2);
                let v1 = self.vram_read(data_addr + row * 2 + 1);

                for col in 0..8 {
                    let p1 = (v0 >> (7 - col)) & 1;
                    let p2 = (v1 >> (7 - col)) & 1;
                    let color_index = (p1 << 1) | p2;

                    let color = if oam_entry.dmg_palette() == false {
                        self.bgp_lut(&self.obj_palette_0, color_index)
                    } else {
                        self.bgp_lut(&self.obj_palette_1, color_index)
                    };

                    let x = 1 + block_x * 10 + col;
                    let y = 1 + block_y * 10 + row as usize;

                    frame_buffer[y * 5 * 10 + x] = color // Dark gray for sprite background
                }
            }
        }
    }

    #[inline]
    fn vram_read(&self, address: u16) -> u8 {
        self.vram[address as usize - VRAM_START as usize]
    }
}

impl RegisterTrait for PPU {
    /*
        |     Mode      | Accessible VRAM |
        |---------------|-----------------|
        | OAM Scan (2)  | VRAM            |
        | Drawing  (3)  | -               |
        | H-Blank  (0)  | VRAM, OAM       |
        | V-Bank   (1)  | VRAM, OAM       |
    */

    fn read(&self, address: u16) -> u8 {
        match address {
            VRAM_START..=VRAM_END => match self.mode {
                PPUMode::PixelTransfer => {
                    warn!("trying to read VRAM while in Pixel Transfer mode");
                    0xFF
                }
                _ => self.vram[(address - VRAM_START) as usize],
            },

            OAM_START..=OAM_END => match self.mode {
                PPUMode::PixelTransfer | PPUMode::OAMSearch => {
                    warn!("trying to read OAM while in Pixel Transfer or OAM Search mode");
                    0xFF
                }
                _ => self.oam[(address - OAM_START) as usize],
            },

            LCDC => self.lcd_control.0,
            STAT => self.lcd_status.0,
            SCY => self.scroll_y,
            SCX => self.scroll_x,
            LY => self.ly,
            LYC => self.ly_compare,
            BGP => self.bg_palette.0,
            OBP0 => self.obj_palette_0.0,
            OBP1 => self.obj_palette_1.0,
            WX => self.window_x,
            WY => self.window_y,
            _ => {
                panic!("PPU read from unknown address: {:#04X}", address);
            }
        }
    }

    fn write(&mut self, address: u16, value: u8) {
        match address {
            VRAM_START..=VRAM_END => match self.mode {
                PPUMode::PixelTransfer => {
                    warn!("trying to write VRAM while in Pixel Transfer mode");
                }
                _ => self.vram[address as usize - VRAM_START as usize] = value,
            },
            OAM_START..=OAM_END => match self.mode {
                PPUMode::PixelTransfer | PPUMode::OAMSearch => {
                    warn!("trying to write OAM while in Pixel Transfer or OAM Search mode");
                }
                _ => self.oam[address as usize - OAM_START as usize] = value,
            },

            LCDC => self.set_lcd_control(value),
            STAT => {
                self.lcd_status.0 = (value & 0b0111_1000u8) | (self.lcd_status.0 & 0b0000_0111u8)
            }
            SCY => self.scroll_y = value,
            SCX => self.scroll_x = value,
            LY => warn!("Write to LY is not allowed !"),
            LYC => self.ly_compare = value,

            BGP => self.bg_palette = BGPalette(value),
            OBP0 => self.obj_palette_0 = BGPalette(value),
            OBP1 => self.obj_palette_1 = BGPalette(value),

            WX => self.window_x = value,
            WY => self.window_y = value,
            _ => {
                panic!("PPU write to unknown address: {:#04X}", address);
            }
        }
    }
}
