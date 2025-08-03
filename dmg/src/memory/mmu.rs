use super::*;

use crate::joypad::Joypad;
use crate::ppu::PPU;
use crate::serial::Serial;
use crate::timer::Timer;

use log::warn;
use std::{cell::RefCell, rc::Rc};

use mbc::MBC;

#[derive(Debug)]
pub struct MMU {
    pub cartridge: MBC,
    pub wram: [u8; 0x2000],
    pub hram: [u8; 0x007F],
    pub ic: Rc<RefCell<InterruptController>>,

    pub boot_rom: BootRom,
    pub dma: DMA,
    pub ppu: PPU,
    pub timer: Timer,
    pub joypad: Joypad,
    pub serial: Serial,
}

impl MMU {
    pub fn new(rom: Option<MBC>, boot_rom: BootRom) -> Rc<RefCell<MMU>> {
        let ic = Rc::new(RefCell::new(InterruptController::new()));

        let ppu = PPU::new(ic.clone());
        let joypad = Joypad::new(ic.clone());

        let mmu = Rc::new(RefCell::new(MMU {
            cartridge: rom.unwrap_or(MBC::empty()),
            wram: [0; 0x2000],
            hram: [0; 0x007F],
            ic: ic,
            boot_rom: boot_rom,
            dma: DMA::new(),
            ppu: ppu,
            timer: Timer::new(),
            joypad: joypad,
            serial: Serial::new(),
        }));

        mmu.clone()
    }

    pub fn tick(&mut self) {
        self.timer.tick(&mut self.ic.borrow_mut());
        self.serial.tick(&mut self.ic.borrow_mut());

        let res = self.dma.tick();
        if let Some(src) = res {
            let dst_idx = src as u8;
            let src_data = self.read_dma(src);
            self.ppu.oam[dst_idx as usize] = src_data;
        }
    }

    #[inline(always)]
    pub fn read(&self, addr: u16) -> u8 {
        if self.dma.is_enabled() && addr <= 0xFF00 {
            log::error!(
                "Attempt to read from OAM during DMA transfer at address {:#04X}",
                addr
            );
            return 0xFF; // Return 0xFF to avoid reading from OAM during DMA transfer
        }

        match addr {
            // ROM Bank 0
            0x0000..=0x3FFF => {
                if self.boot_rom.enabled && addr < 0x0100 {
                    self.boot_rom.read(addr)
                } else {
                    self.cartridge.read_rom(addr)
                }
            }
            // ROM Bank 1-N
            0x4000..=0x7FFF => self.cartridge.read_rom(addr),
            // VRAM
            0x8000..=0x9FFF => self.ppu.read(addr),
            // External RAM
            0xA000..=0xBFFF => self.cartridge.read_ram(addr),
            // Work RAM
            0xC000..=0xDFFF => self.wram[(addr & 0x1FFF) as usize],
            // Echo RAM - Copy of Work RAM
            0xE000..=0xFDFF => self.wram[(addr & 0x1FFF) as usize],
            // OAM
            0xFE00..=0xFE9F => self.ppu.read(addr),
            // Unusable memory
            0xFEA0..=0xFEFF => return 0,
            // IO Registers
            0xFF00..=0xFF7F => match addr {
                P1_JOYP => self.joypad.read(),
                SB => self.serial.read_data(),
                SC => self.serial.read_control(),
                IF => self.ic.borrow().interrupt_flag.0,
                DIV..=TAC => self.timer.read(addr),
                LCDC..=LYC => self.ppu.read(addr),
                DMA => self.dma.read(DMA),
                BGP..=WX => self.ppu.read(addr),
                R_BANK => self.boot_rom.read(R_BANK),
                _ => {
                    warn!("Read from IO Register {:#04X} is not implemented", addr);
                    0xFF
                }
            },
            // HRAM
            0xFF80..=0xFFFE => self.hram[(addr - 0xFF80) as usize],
            // Interrupt Enable Register
            IE => self.ic.borrow().interrupt_enable.0,
        }
    }

    #[inline(always)]
    pub fn read_dma(&self, addr: u16) -> u8 {
        match addr {
            // ROM Bank 0
            0x0000..=0x3FFF => {
                if self.boot_rom.enabled && addr < 0x0100 {
                    self.boot_rom.read(addr)
                } else {
                    self.cartridge.read_rom(addr)
                }
            }
            // ROM Bank 1-N
            0x4000..=0x7FFF => self.cartridge.read_rom(addr),
            // VRAM
            0x8000..=0x9FFF => self.ppu.read(addr),
            // External RAM
            0xA000..=0xBFFF => self.cartridge.read_ram(addr),
            // Work RAM
            0xC000..=0xDFFF => self.wram[(addr & 0x1FFF) as usize],
            // Echo RAM - Copy of Work RAM
            0xE000..=0xFDFF => self.wram[(addr & 0x1FFF) as usize],
            // OAM
            0xFE00..=0xFFFF => self.wram[(addr & 0x1FFF) as usize],
        }
    }

    #[inline(always)]
    pub fn write(&mut self, addr: u16, value: u8) {
        match addr {
            // ROM Bank 0
            0x0000..=0x3FFF => {
                if self.boot_rom.enabled && addr < 0x0100 {
                    warn!("Attempt to write to Boot ROM at address {:#04X}", addr);
                } else {
                    self.cartridge.write_rom(addr, value);
                }
            }
            // ROM Bank 1-N
            0x4000..=0x7FFF => {
                self.cartridge.write_rom(addr, value);
            }
            // VRAM
            0x8000..=0x9FFF => self.ppu.write(addr, value),
            // External RAM
            0xA000..=0xBFFF => self.cartridge.write_ram(addr, value),
            // Work RAM
            0xC000..=0xDFFF => self.wram[(addr & 0x1FFF) as usize] = value,
            // Echo RAM - Copy of Work RAM
            0xE000..=0xFDFF => self.wram[(addr & 0x1FFF) as usize] = value,
            // OAM
            0xFE00..=0xFE9F => self.ppu.write(addr, value),
            // Unusable memory
            0xFEA0..=0xFEFF => (),
            // IO Registers
            0xFF00..=0xFF7F => match addr {
                P1_JOYP => self.joypad.write(value),
                SB => self.serial.write_data(value),
                SC => self.serial.write_control(value),
                IF => self.ic.borrow_mut().interrupt_flag.0 = 0b1110_0000 | value,
                DIV..=TAC => self.timer.write(addr, value),
                DMA => {
                    self.dma.write(DMA, value);
                }
                R_BANK => self.boot_rom.write(R_BANK, value),
                LCDC..=LYC => self.ppu.write(addr, value),
                BGP..=WX => self.ppu.write(addr, value),

                _ => warn!(
                    "Write to IO Register {:#04X} with value {:#04X} is not implemented",
                    addr, value,
                ),
            },
            // HRAM
            0xFF80..=0xFFFE => self.hram[(addr - 0xFF80) as usize] = value,
            // Interrupt Enable Register
            IE => self.ic.borrow_mut().interrupt_enable.0 = value,
        }
    }
}
