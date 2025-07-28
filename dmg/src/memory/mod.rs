pub mod boot_rom;
pub mod dma;
pub mod interrupt_controller;
pub mod io_registers;
pub mod mmu;

pub use boot_rom::*;
pub use dma::*;
pub use interrupt_controller::*;
pub use io_registers::*;
pub use mmu::*;

pub trait RegisterTrait {
    fn read(&self, address: u16) -> u8;
    fn write(&mut self, address: u16, value: u8);
}
