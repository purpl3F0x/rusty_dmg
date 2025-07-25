use crate::mbc::MBCTrait;
use crate::mbc::{ram_banks, rom_banks};

pub struct MBC1 {
    rom: Vec<u8>,
    ram: Vec<u8>,
    has_battery: bool,
    rom_banks: u8,
    ram_banks: u8,
    active_rom_bank: u8,
    active_ram_bank: u8,
    ram_enabled: bool,
    banking_mode: u8,
}

impl MBC1 {
    pub fn new(rom: Vec<u8>, ram_size: u32, has_battery: bool) -> MBC1 {
        let rom_banks = rom_banks(rom.len() as u32);
        let ram_banks = ram_banks(ram_size);

        MBC1 {
            rom,
            ram: vec![0; 0 as usize],
            has_battery,
            rom_banks,
            ram_banks,
            active_rom_bank: 0,
            active_ram_bank: 0,
            ram_enabled: false,
            banking_mode: 0,
        }
    }
}

impl MBCTrait for MBC1 {
    fn name(&self) -> String {
        let mut name = "MBC1".to_string();
        if self.ram.len() > 0 {
            name.push_str("+RAM");
        }
        if self.has_battery {
            name.push_str("+Battery");
        }

        name
    }

    fn read_rom_raw(&self, address: u16) -> u8 {
        self.rom[address as usize]
    }

    fn read_rom(&self, a: u16) -> u8 {
        let bank = if a < 0x4000 {
            if self.banking_mode == 1 {
                0
            } else {
                self.active_rom_bank & 0xE0
            }
        } else {
            self.active_rom_bank
        } as usize;

        let idx = bank * 0x4000 | ((a as usize) & 0x3FFF);
        *self.rom.get(idx).unwrap_or(&0xFF)
    }

    fn write_rom(&mut self, a: u16, v: u8) {
        match a {
            // RAM Enable
            0x0000..=0x1FFF => {
                self.ram_enabled = v & 0x0F == 0x0A;
            }
            // ROM Bank Number
            0x2000..=0x3FFF => {
                let lower_5_bits = v & 0x1F;
                let lower_5_bits = if lower_5_bits == 0 { 1 } else { lower_5_bits };
                self.active_rom_bank = (self.active_rom_bank & 0xE0) | lower_5_bits;
            }
            // RAM Bank Number or Upper Bits of ROM Bank Number
            0x4000..=0x5FFF => {
                if self.rom_banks > 32 {
                    self.active_rom_bank = (self.active_rom_bank & 0x1F) | ((v & 0x03) << 5);
                }
                if self.ram_banks > 1 {
                    self.active_ram_bank = v & 0x03;
                }
            }
            // ROM/RAM Mode Select
            0x6000..=0x7FFF => {
                self.banking_mode = v & 0x01;
                self.active_rom_bank = (self.active_rom_bank & 0x1F) | ((v & 0x03) << 5);
            }
            _ => {}
        }
    }

    fn read_ram(&self, a: u16) -> u8 {
        if self.ram_enabled == false {
            return 0xFF;
        }

        let bank = if self.banking_mode == 1 {
            self.active_ram_bank
        } else {
            0
        } as usize;

        let address = (bank * 0x2000) | ((a & 0x1FFF) as usize);

        self.ram[address]
    }

    fn write_ram(&mut self, a: u16, v: u8) {
        if self.ram_enabled == false {
            return;
        }

        let bank = if self.banking_mode == 1 {
            self.active_ram_bank
        } else {
            0
        } as usize;

        let address = (bank * 0x2000) | ((a & 0x1FFF) as usize);
        self.ram[address] = v;
    }

    fn has_battery(&self) -> bool {
        self.has_battery
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
