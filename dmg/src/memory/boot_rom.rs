use super::*;

use core::panic;
use std::cell::RefCell;
use std::rc::Weak;
use std::result::Result;

use log::{error, warn, debug};

#[derive(Debug, Clone)]
pub struct BootRom {
    pub rom: [u8; 0x100],
    pub enabled: bool,
}

impl BootRom {
    pub fn new() -> Self {
        BootRom {
            rom: [0; 0x100],
            enabled: true,
        }
    }

    pub fn load(&mut self, data: &[u8]) -> Result<(), String> {
        if data.len() > 0x100 {
            return Err("Boot ROM data exceeds 256 bytes".into());
        }

        self.rom[..data.len()].copy_from_slice(data);
        Ok(())
    }
}

impl RegisterTrait for BootRom {
    fn read(&self, address: u16) -> u8 {
        if address < 0x0100 {
            if self.enabled {
                return self.rom[address as usize];
            } else {
                return 0x00;
            }
        } else if address == R_BANK {
            return 0xFF;
        }

        panic!(
            "Attempted to read from Boot ROM at unsupported address: {:#04X}",
            address
        );
    }

    fn write(&mut self, address: u16, value: u8) {
        if address == R_BANK {
            debug!("Writing to Boot ROM at address: {:#04X}, value: {:#04X}", address, value);
            self.enabled = value == 0;

            debug!("Boot ROM enabled: {}", self.enabled);
        }
    }
}
