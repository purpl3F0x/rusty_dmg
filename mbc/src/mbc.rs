use crate::mbc1::MBC1;
use crate::mbc3::MBC3;
use crate::mbc5::MBC5;
use crate::no_mbc::NoMBC;

pub fn rom_banks<T: TryFrom<u32>>(rom_size: u32) -> T {
    let banks = rom_size / (16 * 1024);

    T::try_from(banks).unwrap_or_else(|_| panic!("Invalid ROM banks: {}", banks))
}

pub fn ram_banks(ram_size: u32) -> u8 {
    match ram_size {
        0 => 0,
        0x2000 => 1,
        0x8000 => 4,
        0x20000 => 16,
        0x10000 => 8,
        _ => panic!("Unsupported RAM size: {}", ram_size),
    }
}

pub trait MBCTrait: Send {
    fn name(&self) -> String;

    fn read_rom_raw(&self, address: u16) -> u8;

    fn read_rom(&self, address: u16) -> u8;
    fn write_rom(&mut self, address: u16, value: u8);

    fn read_ram(&self, address: u16) -> u8;
    fn write_ram(&mut self, address: u16, value: u8);

    fn has_battery(&self) -> bool;
    fn dump_ram(&self) -> Vec<u8>;

    fn has_rtc(&self) -> bool {
        false
    }

    fn rom_banks(&self) -> u16 {
        2
    }

    fn ram_banks(&self) -> u8 {
        0
    }

    fn rom_size(&self) -> u32;
    fn ram_size(&self) -> u32;

    fn rom_name(&self) -> String {
        const TITLE_START: u16 = 0x134;
        const CGB_FLAG: u16 = 0x143;

        let title_size = match self.read_rom(CGB_FLAG) & 0x80 {
            0x80 | 0xC0 => 11,
            _ => 16,
        };

        let mut title = String::with_capacity(title_size);

        for i in 0..title_size {
            match self.read_rom_raw(TITLE_START + i as u16) {
                0 => break,
                c => title.push(c as char),
            }
        }
        title
    }
}

pub struct MBC {
    mbc: Box<dyn MBCTrait>,
}

fn get_mbc(rom: Vec<u8>) -> Box<dyn MBCTrait> {
    let ty = rom[0x147];
    let ram_size = match rom[0x149] {
        0x00 => 0,
        0x01 => 2 * 1024,
        0x02 => 8 * 1024,
        0x03 => 32 * 1024,
        0x04 => 128 * 1024,
        0x05 => 64 * 1024,
        _ => panic!("Unsupported RAM size: {:02X}", rom[0x149]),
    };

    match ty {
        0x00 => Box::new(NoMBC::new(rom)),
        0x01 => Box::new(MBC1::new(rom, 0, false)),
        0x02 => Box::new(MBC1::new(rom, ram_size, true)),
        0x03 => Box::new(MBC1::new(rom, ram_size, true)),

        0x0F => Box::new(MBC3::new(rom, ram_size, false, true)),
        0x10 => Box::new(MBC3::new(rom, ram_size, true, true)),
        0x11 => Box::new(MBC3::new(rom, ram_size, true, false)),
        0x12 => Box::new(MBC3::new(rom, ram_size, true, false)),
        0x13 => Box::new(MBC3::new(rom, ram_size, true, false)),

        0x19 => Box::new(MBC5::new(rom, ram_size, false)),
        0x1A => Box::new(MBC5::new(rom, ram_size, true)),
        0x1B => Box::new(MBC5::new(rom, ram_size, true)),
        0x1C => Box::new(MBC5::new(rom, ram_size, false)),
        0x1D => Box::new(MBC5::new(rom, ram_size, true)),
        0x1E => Box::new(MBC5::new(rom, ram_size, true)),

        _ => panic!("Unsupported MBC type: 0x{:02X}", ty),
    }
}

impl MBC {
    pub fn new(rom: Vec<u8>) -> MBC {
        MBC { mbc: get_mbc(rom) }
    }

    pub fn empty() -> MBC {
        MBC {
            mbc: Box::new(NoMBC::new(vec![0xFF; 0x8000])),
        }
    }

    pub fn name(&self) -> String {
        self.mbc.name()
    }

    pub fn rom_name(&self) -> String {
        self.mbc.rom_name()
    }

    pub fn read_rom_raw(&self, address: u16) -> u8 {
        self.mbc.read_rom_raw(address)
    }

    pub fn read_rom(&self, a: u16) -> u8 {
        self.mbc.read_rom(a)
    }

    pub fn read_ram(&self, a: u16) -> u8 {
        self.mbc.read_ram(a)
    }

    pub fn write_rom(&mut self, a: u16, v: u8) {
        self.mbc.write_rom(a, v)
    }

    pub fn write_ram(&mut self, a: u16, v: u8) {
        self.mbc.write_ram(a, v)
    }

    pub fn has_battery(&self) -> bool {
        self.mbc.has_battery()
    }

    pub fn has_rtc(&self) -> bool {
        self.mbc.has_rtc()
    }

    pub fn dump_ram(&self) -> Vec<u8> {
        self.mbc.dump_ram()
    }

    pub fn rom_banks(&self) -> u16 {
        self.mbc.rom_banks()
    }

    pub fn ram_banks(&self) -> u8 {
        self.mbc.ram_banks()
    }

    pub fn rom_size(&self) -> u32 {
        self.mbc.rom_size()
    }

    pub fn ram_size(&self) -> u32 {
        self.mbc.ram_size()
    }
}

impl<T> From<T> for MBC
where
    T: MBCTrait + 'static,
{
    fn from(mbc: T) -> MBC {
        MBC { mbc: Box::new(mbc) }
    }
}

impl std::fmt::Debug for MBC {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("MBC")
            .field("rom_name", &self.rom_name())
            .field("mbc", &self.name())
            .field("rom_size", &self.rom_size())
            .field("ram_size", &self.ram_size())
            .field("rom_banks", &self.rom_banks())
            .field("ram_banks", &self.ram_banks())
            .field("has_battery", &self.has_battery())
            .field("has_rtc", &self.has_rtc())
            .finish()
    }
}
