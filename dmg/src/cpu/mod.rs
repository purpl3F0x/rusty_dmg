use std::cell::RefCell;
use std::rc::Rc;

use super::memory::mmu::MMU;
use super::ppu::PPU;

mod register;
use register::*;

pub mod core;
pub mod decode;
pub mod fetch;
pub mod instructions;

const ZERO_FLAG: u8 = 0x80;
const SUB_FLAG: u8 = 0x40;
const HALF_FLAG: u8 = 0x20;
const CARRY_FLAG: u8 = 0x10;

#[derive(Debug, PartialEq, Eq)]
pub enum CPUMode {
    Normal,
    Halt,
    HaltBug,
    HaltDI,
    Stop,
    EnableIME,
}

#[derive(Debug, PartialEq, Eq)]
pub enum Register8 {
    A,
    B,
    C,
    D,
    E,
    H,
    L,
    IndirectHL,
}

#[derive(Debug, PartialEq, Eq)]
pub enum Register16 {
    AF,
    BC,
    DE,
    HL,
    SP,
    PC,
}

pub enum U3 {
    // Not so necessary, but y not
    B0 = 0,
    B1 = 1,
    B2 = 2,
    B3 = 3,
    B4 = 4,
    B5 = 5,
    B6 = 6,
    B7 = 7,
}

#[derive(Debug)]
pub struct CPU {
    pub af: Reg,
    pub bc: Reg,
    pub de: Reg,
    pub hl: Reg,
    pub pc: u16,
    pub sp: u16,

    pub mmu: Rc<RefCell<MMU>>,

    pub mode: CPUMode,

    pub t_cycles: usize,

    pub ime: bool,
}

#[cfg(test)]
mod test;
