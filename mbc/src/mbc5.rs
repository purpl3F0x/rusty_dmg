use crate::mbc::MBCTrait;
use crate::mbc::{ram_banks, rom_banks};

pub struct MBC5 {
    rom: Vec<u8>,
    ram: Vec<u8>,
    has_battery: bool,
    rom_banks: u8,
    ram_banks: u8,
    active_rom_back: u16,
    active_ram_back: u8,
    ram_enabled: bool,
}

impl MBC5 {
    pub fn new(rom: Vec<u8>, ram_size: u32, has_battery: bool) -> MBC5 {
        let rom_banks = rom_banks(rom.len() as u32);
        let ram_banks = ram_banks(ram_size);

        MBC5 {
            rom,
            ram: vec![0; 0 as usize],
            has_battery,
            rom_banks,
            ram_banks,
            active_rom_back: 0,
            active_ram_back: 0,
            ram_enabled: false,
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

    fn read_rom(&self, a: u16) -> u8 {
        self.rom[a as usize]
    }

    fn write_rom(&mut self, a: u16, v: u8) {
        match a {
            // Ram enable/disable
            0x0000..=0x1FFF => {
                self.ram_enabled = v & 0x0F == 0x0A;
            }
            // ROM bank select ( lower 8 bits )
            0x2000..=0x2FFF => {
                self.active_rom_back = v as u16;
            }
            // ROM bank select ( 9th bit )
            0x3000..=0x3FFF => {
                self.active_rom_back |= (v as u16 & 0x01) << 8;
            }
            // RAM bank select
            0x4000..=0x5FFF => {
                self.active_ram_back = (v & 0x0F) % self.ram_banks;
            }

            _ => {}
        }
    }

    fn read_ram(&self, a: u16) -> u8 {
        if self.ram_enabled == false {
            return 0xFF;
        }

        let idx = self.active_ram_back as usize * 0x2000 | (a as usize & 0x1FFF);
        self.ram[idx]
    }

    fn write_ram(&mut self, a: u16, v: u8) {
        if self.ram_enabled == false {
            return;
        }

        let idx = self.active_ram_back as usize * 0x2000 | (a as usize & 0x1FFF);
        self.ram[idx] = v;
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
