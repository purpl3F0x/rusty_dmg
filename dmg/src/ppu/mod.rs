use crate::memory::InterruptController;

use std::cell::{RefCell, RefMut, UnsafeCell};
use std::rc::{Rc, Weak};

use bitfield::bitfield;

pub mod ppu;

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum PPUMode {
    HBlank = 0,
    VBlank = 1,
    OAMSearch = 2,
    PixelTransfer = 3,
}

bitfield! {
    #[derive(Clone, Copy)]
    pub struct LCDControlRegister(u8);
    impl Debug;
    pub lcd_enable, set_lcd_enable: 7;
    pub window_tile_map_display_select, set_window_tile_map_display_select: 6;
    pub window_display_enable, set_window_display_enable: 5;
    pub bg_window_tile_data_select, set_bg_window_tile_data_select: 4;
    pub bg_tile_map_display_select, set_bg_tile_map_display_select: 3;
    pub obj_size, set_obj_size: 2;
    pub obj_display_enable, set_obj_display_enable: 1;
    pub bg_display_enable, set_bg_display_enable: 0;
}

bitfield! {
    pub struct LCDStatus(u8);
    impl Debug;
    pub lyc_int_select, set_lyc_int_select: 6;
    pub mode2_int_select, set_mode2_int_select: 5;
    pub mode1_int_select, set_mode1_int_select: 4;
    pub mode0_int_select, set_mode0_int_select: 3;
    pub lyc_flag, set_lyc_flag: 2;
    pub ppu_mode, set_ppu_mode: 1, 0;
}

bitfield! {
    pub struct BGPalette(u8);
    impl Debug;
    pub id3, set_id3: 7, 6;
    pub id2, set_id2: 5, 4;
    pub id1, set_id1: 3, 2;
    pub id0, set_id0: 1, 0;
}

bitfield! {
    #[derive(Clone, Copy)]
    struct OAMEntry(MSB0 [u8]);
    impl Debug;
    u8;
    y, _: 7, 0; // Y coordinate of the sprite (0-143)
    x, _: 15, 8; // X coordinate of the sprite (0-159)
    tile_index, _: 23, 16; // Tile index in VRAM

    cgb_palette, _: 31, 29; // CGB palette number (0-7)
    bank, _: 28; // Bank number (0 = normal, 1 = banked)
    dmg_palette, _: 27; // DMG palette number (0 = normal, 1 = alternate)
    x_flip, _: 26; // X flip (0 = normal, 1 = flipped)
    y_flip, _: 25; // Y flip (0 = normal, 1 = flipped)
    priority, _: 24; // Priority (0 = in front of background, 1 = behind background)
}

#[derive(Debug)]
pub struct PPU {
    vram: [u8; 0x2000],
    oam: [u8; 0x00A0],
    // Registers
    lcd_control: LCDControlRegister,
    lcd_status: LCDStatus,
    scroll_y: u8,
    scroll_x: u8,
    ly: u8,
    ly_compare: u8,
    window_y: u8,
    window_x: u8,
    bg_palette: BGPalette,
    obj_palette_0: BGPalette,
    obj_palette_1: BGPalette,
    // Other state
    mode: PPUMode,
    frame_counter: u32,
    line_counter: u16,

    pub scan_line: u8,
    pub frame_buffer: Vec<u32>,
    obj_scanline: [OAMEntry<[u8; 4]>; 10],

    pub frame_ready: bool,
    pub t_cycles: usize,

    pub ic: Rc<RefCell<InterruptController>>,
}


