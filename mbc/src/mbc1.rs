use crate::mbc::MBCTrait;
use crate::mbc::{ram_banks, rom_banks};

pub struct MBC1 {
    rom: Vec<u8>,
    ram: Vec<u8>,
    has_battery: bool,
    rom_banks: u8,
    ram_banks: u8,
    active_rom_bank: u8,
    rom_offsets: (i32, i32),
    active_ram_bank: u8,
    ram_offset: i32,
    ram_enabled: bool,
    banking_mode: bool,
}

impl MBC1 {
    pub fn new(rom: Vec<u8>, ram_size: u32, has_battery: bool) -> MBC1 {
        let rom_banks = rom_banks(rom.len() as u32);
        let ram_banks = ram_banks(ram_size);

        MBC1 {
            rom,
            ram: vec![0; ram_banks as usize * 0x2000],
            has_battery,
            rom_banks,
            ram_banks,
            active_rom_bank: 1,
            rom_offsets: (0x0000, 0x0000),
            active_ram_bank: 0,
            ram_offset: -0xA000,
            ram_enabled: false,
            banking_mode: false,
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

    fn read_rom(&self, address: u16) -> u8 {
        if address < 0x4000 {
            self.rom[address as usize + self.rom_offsets.0 as usize]
        } else {
            self.rom[(address as i32 + self.rom_offsets.1) as usize]
        }
    }

    fn write_rom(&mut self, address: u16, value: u8) {
        match address {
            // RAM Enable
            0x0000..=0x1FFF => {
                self.ram_enabled = value & 0x0F == 0x0A;
            }
            // ROM Bank Number
            0x2000..=0x3FFF => {
                let mut value = value & 0x1F;
                value = if value == 0 { 1 } else { value };

                value = value & (self.rom_banks - 1);

                value = if self.banking_mode {
                    (self.active_rom_bank & 0b110_0000) | value
                } else {
                    value
                };
                self.active_rom_bank = value;

                self.rom_offsets.1 = (value as i32 - 1) * 0x4000;
            }
            // RAM Bank Number or Upper Bits of ROM Bank Number
            0x4000..=0x5FFF => {
                if self.ram_banks > 1 && self.banking_mode {
                    let value = value & 0b11;
                    self.active_ram_bank = value;
                    self.ram_offset = (value as u16 * 0x2000) as i32 - 0xA000;

                    log::debug!(
                        "Setting RAM bank to {} - offset -0x{:04x}",
                        value,
                        -self.ram_offset
                    );
                } else if self.rom_banks > 32 {
                    let value = value & 0b11;
                    let value = (self.active_rom_bank & 0x1F) | (value << 5);
                    let value = value & (self.rom_banks - 1);
                    self.active_rom_bank = value;
                    self.rom_offsets.1 = (value as i32 - 1) * 0x4000;

                    log::debug!(
                        "Setting ROM bank to {} - offset 0x{:04x}",
                        self.active_rom_bank,
                        self.rom_offsets.1
                    );
                }
            }
            // ROM/RAM Mode Select
            0x6000..=0x7FFF => {
                self.banking_mode = (value & 1) != 0;
                self.ram_offset = -0xA000;

                if self.banking_mode {
                    let active_rom_bank = self.active_rom_bank & 0b1_1111;
                    self.rom_offsets.1 = (active_rom_bank as i32 - 1) * 0x4000;
                }

                log::debug!(
                    "Setting banking mode to {} - active ROM bank {} - offset 0x{:04x}",
                    self.banking_mode,
                    self.active_rom_bank,
                    self.rom_offsets.1
                );
            }
            _ => {}
        }
    }

    fn read_ram(&self, address: u16) -> u8 {
        if self.ram_enabled == false || self.ram_banks == 0 {
            return 0xFF;
        }

        let idx: usize = (address as i32 + self.ram_offset) as usize;
        self.ram[idx]
    }

    fn write_ram(&mut self, address: u16, value: u8) {
        if self.ram_enabled == false || self.ram_banks == 0 {
            return;
        }

        let idx: usize = (address as i32 + self.ram_offset) as usize;
        self.ram[idx] = value;
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
