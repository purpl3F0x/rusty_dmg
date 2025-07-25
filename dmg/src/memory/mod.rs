pub mod boot_rom;
pub mod dma;
pub mod io_registers;
pub mod mmu;
pub mod interrupt_controller;

pub use boot_rom::*;
pub use dma::*;
pub use io_registers::*;
pub use mmu::*;
pub use interrupt_controller::*;

pub trait RegisterTrait {
    fn read(&self, address: u16) -> u8;
    fn write(&mut self, address: u16, value: u8);
}
