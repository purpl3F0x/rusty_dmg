use crate::mbc::MBCTrait;
use crate::mbc::{ram_banks, rom_banks};

use std::time;

struct RTC {
    rtc_ram: [u8; 5],
}

pub struct MBC3 {
    rom: Vec<u8>,
    ram: Vec<u8>,
    has_battery: bool,
    rom_banks: u8,
    ram_banks: u8,
    active_rom_bank: u8,
    active_ram_bank: u8,
    ram_enabled: bool,
    rtc: Option<RTC>,
}

impl MBC3 {
    pub fn new(rom: Vec<u8>, ram_size: u32, has_battery: bool, has_rtc: bool) -> MBC3 {
        let rom_banks = rom_banks(rom.len() as u32);
        let ram_banks = ram_banks(ram_size);

        let rtc = match has_rtc {
            true => Some(RTC { rtc_ram: [0; 5] }),
            false => None,
        };

        MBC3 {
            rom,
            ram: vec![0; ram_size as usize],
            has_battery,
            rom_banks,
            ram_banks,
            active_rom_bank: 1,
            active_ram_bank: 0,
            ram_enabled: false,
            rtc: rtc,
        }
    }
}

impl MBCTrait for MBC3 {
    fn name(&self) -> String {
        let mut name = "MBC3".to_string();
        if self.ram.len() > 0 {
            name.push_str("+RAM");
        }
        if self.has_battery {
            name.push_str("+Battery");
        }
        if self.rtc.is_some() {
            name.push_str("+RTC");
        }
        name
    }

    fn read_rom_raw(&self, address: u16) -> u8 {
        self.rom[address as usize]
    }

    fn read_rom(&self, a: u16) -> u8 {
        *self.rom.get(a as usize).unwrap_or(&0xFF)
    }

    fn write_rom(&mut self, a: u16, v: u8) {
        match a {
            // Ram enable/disable
            0x0000..=0x1FFF => {
                self.ram_enabled = v & 0x0F == 0x0A;
            }
            // ROM bank select
            0x2000..=0x3FFF => {
                self.active_rom_bank = v & 0x7F;
                if self.active_rom_bank == 0 {
                    self.active_rom_bank = 1;
                }
            }
            // RAM bank select
            0x4000..=0x5FFF => {
                self.active_ram_bank = v & 0x0F;
            }
            // Latch clock data
            0x6000..=0x7FFF => {
                if let Some(ref mut rtc) = self.rtc {
                    if v & 0x01 == 0x01 {
                        // rtc.latched_rtc = rtc.rtc_ram;
                    }
                }
            }
            _ => {}
        }
    }

    fn read_ram(&self, a: u16) -> u8 {
        if self.ram_enabled == false {
            return 0xFF;
        }

        let idx = self.active_ram_bank as usize * 0x2000 | (a as usize & 0x1FFF);
        *self.ram.get(idx).unwrap_or(&0xFF)
    }

    fn write_ram(&mut self, a: u16, v: u8) {
        if self.ram_enabled == false {
            return;
        }

        let idx = self.active_ram_bank as usize * 0x2000 | (a as usize & 0x1FFF);
        self.ram[idx] = v;
    }

    fn has_battery(&self) -> bool {
        self.has_battery
    }

    fn has_rtc(&self) -> bool {
        self.rtc.is_some()
    }

    fn dump_ram(&self) -> Vec<u8> {
        self.ram.clone()
    }

    fn rom_banks(&self) -> u8 {
        self.rom_banks
    }

    fn ram_banks(&self) -> u8 {
        self.ram_banks
    }

    fn rom_size(&self) -> u32 {
        self.rom.len() as u32
    }

    fn ram_size(&self) -> u32 {
        self.ram.len() as u32
    }
}
