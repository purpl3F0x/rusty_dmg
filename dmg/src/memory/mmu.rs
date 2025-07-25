use super::*;

use crate::joypad::{self, Joypad};
use crate::ppu::PPU;
use crate::timer::Timer;

use log::{info, warn};
use std::{cell::RefCell, rc::Rc};

#[derive(Debug)]
pub struct MMU {
    pub rom: [u8; 0x8000],
    pub wram: [u8; 0x2000],
    pub hram: [u8; 0x007F],
    pub ic: Rc<RefCell<InterruptController>>,

    pub boot_rom: BootRom,
    pub dma: Rc<RefCell<DMA>>,
    pub ppu: PPU,
    pub timer: Timer,
    pub joypad: Joypad,
}

impl MMU {
    pub fn new(rom: [u8; 0x8000], boot_rom: BootRom, dma: Rc<RefCell<DMA>>) -> Rc<RefCell<MMU>> {
        let ic = Rc::new(RefCell::new(InterruptController::new()));

        let ppu = PPU::new(ic.clone());
        let joypad = Joypad::new(ic.clone());

        let mmu = Rc::new(RefCell::new(MMU {
            rom: rom,
            wram: [0; 0x2000],
            hram: [0; 0x007F],
            ic: ic,
            boot_rom: boot_rom,
            dma: dma.clone(),
            ppu: ppu,
            timer: Timer::new(),
            joypad: joypad,
        }));

        // Set the DMA's mmu weak reference
        dma.borrow_mut().mmu = Rc::downgrade(&mmu);

        mmu.clone()
    }

    pub fn tick(&mut self) {
        self.timer.tick(&mut self.ic.borrow_mut());

        // self.dma.borrow_mut().tick();
    }

    #[inline(always)]
    pub fn read(&self, addr: u16) -> u8 {
        match addr {
            // ROM Bank 0
            0x0000..=0x3FFF => {
                if self.boot_rom.enabled && addr < 0x0100 {
                    self.boot_rom.read(addr)
                } else {
                    self.rom[(addr - 0x0000) as usize]
                }
            }
            // ROM Bank 1-N
            0x4000..=0x7FFF => self.rom[(addr - 0x0000) as usize],
            // VRAM
            0x8000..=0x9FFF => self.ppu.read(addr),
            // External RAM
            0xA000..=0xBFFF => unimplemented!("Read from external RAM"),
            // Work RAM
            0xC000..=0xDFFF => self.wram[(addr - 0xC000) as usize],
            // Echo RAM - Copy of Work RAM
            0xE000..=0xFDFF => self.wram[(addr - 0xE000) as usize],
            // OAM
            0xFE00..=0xFE9F => self.ppu.read(addr),
            // Unusable memory
            0xFEA0..=0xFEFF => return 0,
            // IO Registers
            0xFF00..=0xFF7F => match addr {
                P1_JOYP => self.joypad.read(),
                IF => self.ic.borrow().interrupt_flag.0,
                DIV..=TAC => self.timer.read(addr),
                LCDC..=LYC => self.ppu.read(addr),
                BGP..=WX => self.ppu.read(addr),
                R_BANK => self.boot_rom.read(addr),
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
    pub fn write(&mut self, addr: u16, value: u8) {
        match addr {
            // ROM Bank 0
            0x0000..=0x3FFF => {
                if self.boot_rom.enabled && addr < 0x0100 {
                    warn!("Attempt to write to Boot ROM at address {:#04X}", addr);
                } else {
                    self.rom[(addr - 0x0000) as usize] = value;
                }
            }
            // ROM Bank 1-N
            0x4000..=0x7FFF => unimplemented!("Write to ROM Bank "),
            // VRAM
            0x8000..=0x9FFF => self.ppu.write(addr, value),
            // External RAM
            0xA000..=0xBFFF => unimplemented!("Write to external RAM"),
            // Work RAM
            0xC000..=0xDFFF => self.wram[(addr - 0xC000) as usize] = value,
            // Echo RAM - Copy of Work RAM
            0xE000..=0xFDFF => self.wram[(addr - 0xE000) as usize] = value,
            // OAM
            0xFE00..=0xFE9F => self.ppu.write(addr, value),
            // Unusable memory
            0xFEA0..=0xFEFF => (),
            // IO Registers
            0xFF00..=0xFF7F => match addr {
                P1_JOYP => self.joypad.write(value),
                IF => self.ic.borrow_mut().interrupt_flag.0 = value,
                DIV..=TAC => self.timer.write(addr, value),
                DMA => {
                    let mut dma = self.dma.borrow_mut();
                    dma.write(addr, value);
                }
                R_BANK => self.boot_rom.write(addr, value),
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
