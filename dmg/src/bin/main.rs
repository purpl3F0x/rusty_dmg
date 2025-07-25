use dmg::{
    cpu::CPU,
    memory::RegisterTrait,
    memory::{BootRom, MMU},
    ppu::PPU,
};

use std::io::{Cursor, Write};
use std::time;
use std::{cell::RefCell, rc::Rc, rc::Weak};

use minifb::{Key, ScaleMode, Window, WindowOptions};

static DMG_ROM: &[u8] = include_bytes!("./dmg_boot.bin");
static DMA_TEST_ROM: &[u8] = include_bytes!(".\\Dropzone (Europe).gb");

fn main() {
    let env = env_logger::Env::default()
        .filter_or("MY_LOG_LEVEL", "warn")
        .write_style_or("MY_LOG_STYLE", "auto");

    env_logger::init_from_env(env);

    let mut rom = [0u8; 0x8000];
    let mut bootrom = BootRom::new();
    _ = bootrom.load(DMG_ROM);

    let dma = Rc::new(RefCell::new(dmg::memory::dma::DMA::new(Weak::new())));

    // Copy DMA_TEST_ROM into the ROM
    let mut i = 0;
    while i < DMA_TEST_ROM.len() && i < rom.len() {
        rom[i] = DMA_TEST_ROM[i];
        i += 1;
    }

    let mmu: Rc<RefCell<MMU>> = MMU::new(rom, bootrom, dma.clone());

    let mut cpu = CPU::new(mmu.clone());

    let mut window = Window::new(
        "Test - ESC to exit",
        256,
        256,
        WindowOptions {
            resize: true,
            scale: minifb::Scale::X2,
            scale_mode: ScaleMode::AspectRatioStretch,
            ..WindowOptions::default()
        },
    )
    .unwrap_or_else(|e| {
        panic!("{}", e);
    });

    let mut display_window = Window::new(
        "Test - ESC to exit",
        160,
        144,
        WindowOptions {
            resize: true,
            scale: minifb::Scale::X4,
            scale_mode: ScaleMode::AspectRatioStretch,
            ..WindowOptions::default()
        },
    )
    .unwrap_or_else(|e| {
        panic!("{}", e);
    });

    let mut sprite_window = Window::new(
        "OAM entries",
        10 * 5,
        10 * 8,
        WindowOptions {
            resize: true,
            scale: minifb::Scale::X4,
            scale_mode: ScaleMode::AspectRatioStretch,
            ..WindowOptions::default()
        },
    )
    .unwrap_or_else(|e| {
        panic!("{}", e);
    });

    // window.set_target_fps(300);
    let mut buffer = vec![0; 32 * 8 * 32 * 8];
    let mut oam_buffer = vec![0; 10 * 10 * 5 * 8 * 8];

    // Calculate FPS
    let mut last_time = time::Instant::now();
    let mut currnt_time = time::Instant::now();

    while window.is_open() && !window.is_key_down(Key::Escape) {
        // If the next opcode is 0x00, skip it
        cpu.do_step();

        if mmu.clone().borrow().ppu.frame_ready {
            mmu.clone().borrow_mut().ppu.render_bg(&mut buffer);

            mmu.clone()
                .borrow_mut()
                .ppu
                .render_sprites_debug(&mut oam_buffer);

            window.update_with_buffer(&buffer, 256, 256).unwrap();

            sprite_window
                .update_with_buffer(&oam_buffer, 10 * 5, 10 * 8)
                .unwrap();

            display_window
                .update_with_buffer(&mmu.borrow().ppu.frame_buffer, 160, 144)
                .unwrap();

            last_time = currnt_time;
            currnt_time = time::Instant::now();

            let elapsed = currnt_time.duration_since(last_time);
            let fps = 1.0 / elapsed.as_secs_f64();

            print!("\rFPS: {}", fps);
        }
    }

    let mut vram_file = std::fs::File::create("vram_dump.bin").unwrap();
    for i in 0x8000..=0x9FFF {
        let value = mmu.borrow().ppu.read(i);
        vram_file.write(&[value]).unwrap();
    }

    let mut oam_file = std::fs::File::create("oam_dump.bin").unwrap();
    for i in 0xFE00..=0xFE9F {
        let value = mmu.borrow().ppu.read(i);
        oam_file.write(&[value]).unwrap();
    }

    let mut wram_file = std::fs::File::create("wram_dump.bin").unwrap();
    for i in 0xC000..=0xC090 {
        let value = mmu.borrow().read(i);
        wram_file.write(&[value]).unwrap();
    }
}
