mod app;
pub use app::TemplateApp;

use dmg::{
    cpu::CPU,
    memory::{BootRom, MMU},
};

use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{Arc, Mutex};
use std::{cell::RefCell, rc::Rc, rc::Weak};
use std::{time, vec};

use eframe::{UserEvent, egui};
use winit::event_loop::{ControlFlow, EventLoop};

use flexi_logger::{DeferredNow, FileSpec, Logger, WriteMode};
use log::Record;

static DMG_ROM: &[u8] = include_bytes!("..\\..\\dmg\\src\\bin\\dmg_boot.bin");
static DMA_TEST_ROM: &[u8] = include_bytes!("..\\..\\dmg\\src\\bin\\Tetris (JUE) (V1.1) [!].gb");

pub fn no_info_format(
    w: &mut dyn std::io::Write,
    _now: &mut DeferredNow,
    record: &Record,
) -> Result<(), std::io::Error> {
    write!(w, "{}", &record.args())
}

fn main() {
    let _logger = Logger::try_with_str("warn")
        .unwrap()
        .write_mode(WriteMode::Async)
        .format(no_info_format)
        .log_to_file(FileSpec::default())
        .start()
        .unwrap();

    let run_emulator = Arc::new(AtomicBool::new(true));
    let r = run_emulator.clone();

    let frame_ready = Arc::new(AtomicBool::new(false));
    let frame_ready_clone = frame_ready.clone();

    let screen_buffer = Arc::new(Mutex::new(vec![0u8; 160 * 144 * 4]));
    let background_buffer = Arc::new(Mutex::new(vec![0u8; 256 * 256 * 4]));
    let sprites_buffer = Arc::new(Mutex::new(vec![0u8; 10 * 5 * 10 * 8 * 4]));

    let screen_buffer_clone = screen_buffer.clone();
    let background_buffer_clone = background_buffer.clone();
    let sprites_buffer_clone = sprites_buffer.clone();

    // Run emulator in a seperate thread
    std::thread::spawn(move || {
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
        let mmu: Rc<RefCell<MMU>> = MMU::new(rom, bootrom.clone(), dma.clone());

        let mut cpu = CPU::new(mmu.clone());

        let mut bg_buffer = vec![0u32; 256 * 256];
        let mut oam_buffer = vec![0u32; 10 * 5 * 10 * 8];

        while r.load(Ordering::Relaxed) {
            cpu.do_step();

            if cpu.pc == 0x0100 {
                _logger.parse_new_spec("dmg::cpu::decode = trace").unwrap();
                println!("Entering main loop, PC: {:04X}", cpu.pc);
            }

            let frame_ready = mmu.borrow().ppu.frame_ready;
            let ppu = &mut mmu.borrow_mut().ppu;
            if frame_ready {
                // Copy the frame_buffer to the screen buffer
                {
                    let mut screen_buffer = screen_buffer_clone.lock().unwrap();
                    let src = &ppu.frame_buffer;

                    for i in 0..src.len() {
                        screen_buffer[i * 4] = (src[i] >> 16) as u8;
                        screen_buffer[i * 4 + 1] = (src[i] >> 8) as u8;
                        screen_buffer[i * 4 + 2] = (src[i] >> 0) as u8;
                        screen_buffer[i * 4 + 3] = 0xFF;
                    }

                    ppu.render_bg(&mut bg_buffer);
                    let mut background_buffer = background_buffer_clone.lock().unwrap();
                    for i in 0..256 * 256 {
                        background_buffer[i * 4] = (bg_buffer[i] >> 16) as u8;
                        background_buffer[i * 4 + 1] = (bg_buffer[i] >> 8) as u8;
                        background_buffer[i * 4 + 2] = (bg_buffer[i] >> 0) as u8;
                        background_buffer[i * 4 + 3] = 0xFF;
                    }

                    ppu.render_sprites_debug(&mut oam_buffer);
                    let mut sprites_buffer = sprites_buffer_clone.lock().unwrap();
                    for i in 0..oam_buffer.len() {
                        sprites_buffer[i * 4] = (oam_buffer[i] >> 16) as u8;
                        sprites_buffer[i * 4 + 1] = (oam_buffer[i] >> 8) as u8;
                        sprites_buffer[i * 4 + 2] = (oam_buffer[i] >> 0) as u8;
                        sprites_buffer[i * 4 + 3] = 0xFF;
                    }
                }

                frame_ready_clone.store(true, Ordering::Relaxed);

                while frame_ready_clone.load(Ordering::Relaxed) == true {
                    std::thread::sleep(time::Duration::from_millis(1));
                }
                ppu.frame_ready = false;
            }
        }

        println!("Exiting emulator loop");
    });

    let native_options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default().with_inner_size([400.0, 300.0]),
        ..Default::default()
    };

    let eventloop = EventLoop::<UserEvent>::with_user_event().build().unwrap();
    eventloop.set_control_flow(ControlFlow::Poll);

    let mut winit_app = eframe::create_native(
        "Rusty-DMG",
        native_options,
        Box::new(|cc| {
            Ok(Box::new(TemplateApp::new(
                cc,
                frame_ready,
                screen_buffer,
                background_buffer,
                sprites_buffer,
            )))
        }),
        &eventloop,
    );
    eventloop.run_app(&mut winit_app).unwrap();

    run_emulator.store(false, Ordering::Relaxed);
}
