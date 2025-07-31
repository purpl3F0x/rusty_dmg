use crate::mbc::MBCTrait;

pub struct NoMBC {
    rom: Vec<u8>,
}

impl NoMBC {
    pub fn new(rom: Vec<u8>) -> NoMBC {
        NoMBC { rom }
    }
}

impl MBCTrait for NoMBC {
    fn name(&self) -> String {
        "NoMBC".to_string()
    }

    fn read_rom_raw(&self, address: u16) -> u8 {
        self.rom[address as usize]
    }

    fn read_rom(&self, address: u16) -> u8 {
        self.rom[address as usize]
    }

    fn write_rom(&mut self, _address: u16, _value: u8) {
        ()
    }

    fn read_ram(&self, _address: u16) -> u8 {
        0
    }

    fn write_ram(&mut self, _address: u16, _value: u8) {
        ()
    }

    fn has_battery(&self) -> bool {
        false
    }

    fn dump_ram(&self) -> Vec<u8> {
        vec![]
    }

    fn rom_size(&self) -> u32 {
        self.rom.len() as u32
    }

    fn ram_size(&self) -> u32 {
        0
    }
}
