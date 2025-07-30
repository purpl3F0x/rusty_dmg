mod app;
pub use app::App;

use dmg::ppu::IntoRawBytes;
use dmg::{
    cpu::CPU,
    memory::{BootRom, MMU},
    ppu::color32::Color32,
};

use eframe::{UserEvent, egui};
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::mpsc::channel;
use std::sync::{Arc, Condvar, Mutex};
use std::{cell::RefCell, rc::Rc, rc::Weak};
use winit::event_loop::{ControlFlow, EventLoop};

use flexi_logger::{DeferredNow, Logger, WriteMode};
use log::Record;

static DMG_ROM: &[u8] = include_bytes!("..\\..\\roms\\dmg_boot.bin");
static TEST_ROM: &[u8] = include_bytes!("..\\..\\roms\\Tetris (JUE) (V1.1) [!].gb");

pub fn no_info_format(
    w: &mut dyn std::io::Write,
    _now: &mut DeferredNow,
    record: &Record,
) -> Result<(), std::io::Error> {
    write!(w, "{}", &record.args())
}

fn main() {
    let _logger = Logger::try_with_str("warn, dmg=info")
        .unwrap()
        .write_mode(WriteMode::Async)
        // .format(no_info_format)
        // .log_to_file(FileSpec::default())
        .start()
        .unwrap();

    let run_emulator = Arc::new(AtomicBool::new(true));
    let r = run_emulator.clone();

    let frame_ready_condvar = Arc::new((Mutex::new(true), Condvar::new()));
    let frame_ready_condvar_clone = Arc::clone(&frame_ready_condvar);

    let screen_buffer = Arc::new(Mutex::new(egui::ColorImage::filled(
        [160, 144],
        egui::Color32::PURPLE,
    )));
    let background_buffer = Arc::new(Mutex::new(egui::ColorImage::filled(
        [256, 256],
        egui::Color32::PURPLE,
    )));
    let sprites_buffer = Arc::new(Mutex::new(egui::ColorImage::filled(
        [10 * 5, 10 * 8],
        egui::Color32::PURPLE,
    )));

    let screen_buffer_clone = screen_buffer.clone();
    let background_buffer_clone = background_buffer.clone();
    let sprites_buffer_clone = sprites_buffer.clone();

    let (keypad_tx, keypad_rx) = channel::<(u8, u8)>();

    // Run emulator in a seperate thread
    let emu_thread = std::thread::spawn(move || {
        let mut bootrom = BootRom::new();
        _ = bootrom.load(DMG_ROM);

        let dma = Rc::new(RefCell::new(dmg::memory::dma::DMA::new(Weak::new())));

        // Create a new MBC
        let rom = mbc::MBC::new(TEST_ROM.to_vec());

        log::info!("Starting emulator with ROM: {:?}", rom);

        let mmu: Rc<RefCell<MMU>> = MMU::new(Some(rom), bootrom.clone(), dma.clone());

        let mut cpu = CPU::new(mmu.clone());

        while r.load(Ordering::Relaxed) {
            cpu.do_step();

            // if cpu.pc == 0x0100 {
            //     _logger.parse_new_spec("dmg::cpu::decode = trace").unwrap();
            //     println!("Entering main loop, PC: {:04X}", cpu.pc);
            // }

            if mmu.borrow().ppu.frame_ready {
                // Copy the frame_buffer to the screen buffer
                {
                    let ppu = &mut mmu.borrow_mut().ppu;

                    // Copy the screen
                    let mut screen_buffer = screen_buffer_clone.lock().unwrap();
                    let screen_buffer_ptr = screen_buffer.as_raw_mut();
                    let src = &ppu.frame_buffer.as_raw_bytes();
                    screen_buffer_ptr.clone_from_slice(src);

                    // Render the background debug view
                    let background_buffer = background_buffer_clone.lock().unwrap();
                    let mut bg_buffer_as_color32_slice = unsafe {
                        std::slice::from_raw_parts_mut(
                            background_buffer.pixels.as_ptr() as *mut Color32,
                            background_buffer.pixels.len(),
                        )
                    };
                    ppu.render_bg_debug(&mut bg_buffer_as_color32_slice);

                    // Render the sprites debug view
                    let sprites_buffer = sprites_buffer_clone.lock().unwrap();
                    let mut sprites_buffer_as_color32_slice = unsafe {
                        std::slice::from_raw_parts_mut(
                            sprites_buffer.pixels.as_ptr() as *mut Color32,
                            sprites_buffer.pixels.len(),
                        )
                    };
                    ppu.render_sprites_debug(&mut sprites_buffer_as_color32_slice);
                }

                let (lock, cvar) = &*frame_ready_condvar_clone;
                let mut frame_ready_sync = lock.lock().unwrap();
                *frame_ready_sync = true;
                cvar.notify_one();
                drop(frame_ready_sync);

                // Blocking
                let input = keypad_rx
                    .recv_timeout(std::time::Duration::from_millis(100))
                    .ok();

                if let Some((buttons, dpad)) = input {
                    mmu.borrow_mut().joypad.set_buttons(buttons, dpad);
                }

                mmu.borrow_mut().ppu.frame_ready = false;
            }
        }

        log::info!("Exiting emulator loop");
    });

    let native_options = eframe::NativeOptions {
        renderer: eframe::Renderer::Wgpu,
        viewport: egui::ViewportBuilder::default().with_inner_size([400.0, 300.0]),
        ..Default::default()
    };

    let eventloop = EventLoop::<UserEvent>::with_user_event().build().unwrap();
    eventloop.set_control_flow(ControlFlow::Poll);

    let mut winit_app = eframe::create_native(
        "Rusty-DMG",
        native_options,
        Box::new(|cc| {
            Ok(Box::new(App::new(
                cc,
                frame_ready_condvar,
                screen_buffer,
                background_buffer,
                sprites_buffer,
                keypad_tx,
            )))
        }),
        &eventloop,
    );
    eventloop.run_app(&mut winit_app).unwrap();

    log::warn!("Shutting down emulator...");

    run_emulator.store(false, Ordering::Relaxed);
    emu_thread.join().unwrap();
}
