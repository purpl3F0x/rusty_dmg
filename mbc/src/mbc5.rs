use crate::mbc::MBCTrait;
use crate::mbc::{ram_banks, rom_banks};

pub struct MBC5 {
    rom: Vec<u8>,
    ram: Vec<u8>,
    has_battery: bool,
    rom_banks: u16,
    ram_banks: u8,
    active_rom_back: u16,
    rom_upper_bank_offset: i32,
    active_ram_back: u8,
    ram_enabled: bool,
    ram_offset: i32,
}

impl MBC5 {
    pub fn new(rom: Vec<u8>, ram_size: u32, has_battery: bool) -> MBC5 {
        let rom_banks = rom_banks(rom.len() as u32);
        let ram_banks = ram_banks(ram_size);

        MBC5 {
            rom,
            ram: vec![0; ram_size as usize],
            has_battery,
            rom_banks,
            ram_banks,
            active_rom_back: 1,
            rom_upper_bank_offset: 0,
            active_ram_back: 0,
            ram_enabled: false,
            ram_offset: -0xA000,
        }
    }
}

impl MBCTrait for MBC5 {
    fn name(&self) -> String {
        let mut name = "MBC5".to_string();
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

    fn read_rom(&self, address: u16) -> u8 {
        if address < 0x4000 {
            self.rom[address as usize]
        } else {
            self.rom[(address as i32 + self.rom_upper_bank_offset) as usize]
        }
    }

    fn write_rom(&mut self, address: u16, value: u8) {
        match address {
            // Ram enable/disable
            0x0000..=0x1FFF => {
                self.ram_enabled = value & 0x0F == 0x0A;
            }
            // ROM bank select ( lower 8 bits )
            0x2000..=0x2FFF => {
                self.active_rom_back = (self.active_rom_back & 0x100) | (value as u16 & 0xFF);
                self.active_rom_back &= self.rom_banks - 1;
                self.rom_upper_bank_offset = (self.active_rom_back as i32 - 1) * 0x4000;
            }
            // ROM bank select ( 9th bit )
            0x3000..=0x3FFF => {
                self.active_rom_back = (self.active_rom_back & 0xFF) | (value as u16 & 0b1) << 1;
                self.active_rom_back &= self.rom_banks - 1;
                self.rom_upper_bank_offset = (self.active_rom_back as i32 - 1) * 0x4000;
            }
            // RAM bank select
            0x4000..=0x5FFF => {
                self.active_ram_back = (value & 0x0F) % self.ram_banks;
                self.ram_offset = self.active_ram_back as i32 * 0x2000 - 0xA000;
            }

            _ => {}
        }
    }

    fn read_ram(&self, address: u16) -> u8 {
        if self.ram_enabled == false {
            return 0xFF;
        }

        let idx = (address as i32 + self.ram_offset) as usize;
        self.ram[idx]
    }

    fn write_ram(&mut self, address: u16, value: u8) {
        if self.ram_enabled == false {
            return;
        }

        let idx = self.active_ram_back as usize * 0x2000 | (address as usize & 0x1FFF);
        self.ram[idx] = value;
    }

    fn has_battery(&self) -> bool {
        self.has_battery
    }

    fn dump_ram(&self) -> Vec<u8> {
        self.ram.clone()
    }

    fn rom_banks(&self) -> u16 {
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
