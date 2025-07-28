use super::*;

use log::*;

impl CPU {
    pub fn new(mmu: Rc<RefCell<MMU>>) -> CPU {
        CPU {
            af: Reg::new(),
            bc: Reg::new(),
            de: Reg::new(),
            hl: Reg::new(),
            pc: 0u16,
            sp: 0u16,
            mmu: mmu,
            mode: CPUMode::Normal,
            t_cycles: 0,
            ime: false,
        }
    }

    pub fn init(&mut self) {
        *self.af.value_mut() = 0x01B0; // A = 0x01, F = 0xB0 (Z = 1, N = 0, H = 1, C = 0)
        *self.bc.value_mut() = 0x0013; // B = 0x00, C = 0x13
        *self.de.value_mut() = 0x00D8; // D = 0x00, E = 0xD8
        *self.hl.value_mut() = 0x014D; // H = 0x01, L = 0x4D
        self.pc = 0x0100; // Start at address 0x0100
        self.sp = 0xFFFE; // Stack pointer initialized to top of RAM
    }

    /* Mutable 8-bit register methods  */

    pub fn a_mut(&mut self) -> &mut u8 {
        self.af.high_mut()
    }

    pub fn f_mut(&mut self) -> &mut u8 {
        self.af.low_mut()
    }

    pub fn b_mut(&mut self) -> &mut u8 {
        self.bc.high_mut()
    }

    pub fn c_mut(&mut self) -> &mut u8 {
        self.bc.low_mut()
    }

    pub fn d_mut(&mut self) -> &mut u8 {
        self.de.high_mut()
    }

    pub fn e_mut(&mut self) -> &mut u8 {
        self.de.low_mut()
    }

    pub fn h_mut(&mut self) -> &mut u8 {
        self.hl.high_mut()
    }

    pub fn l_mut(&mut self) -> &mut u8 {
        self.hl.low_mut()
    }

    /* Immutable 8-bit register methods  */

    pub fn a(&self) -> u8 {
        self.af.high()
    }

    pub fn f(&self) -> u8 {
        self.af.low()
    }

    pub fn b(&self) -> u8 {
        self.bc.high()
    }

    pub fn c(&self) -> u8 {
        self.bc.low()
    }

    pub fn d(&self) -> u8 {
        self.de.high()
    }

    pub fn e(&self) -> u8 {
        self.de.low()
    }

    pub fn h(&self) -> u8 {
        self.hl.high()
    }

    pub fn l(&self) -> u8 {
        self.hl.low()
    }

    /* Flag operations */

    pub fn set_flag(&mut self, flag: u8) {
        *self.f_mut() |= flag;
    }

    pub fn clear_flag(&mut self, flag: u8) {
        *self.f_mut() &= !flag;
    }

    pub fn update_flag(&mut self, flag: u8, value: bool) {
        if value {
            self.set_flag(flag);
        } else {
            self.clear_flag(flag);
        }
    }

    pub fn get_flag(&self, flag: u8) -> bool {
        self.f() & flag != 0
    }

    /* Clock Ticks */

    pub fn tick(&mut self) {
        self.mmu.borrow_mut().ppu.tick();
    }

    pub fn tick4(&mut self) {
        self.t_cycles += 4;

        self.mmu.borrow_mut().tick();

        let dma = self.mmu.borrow().dma.clone();

        dma.borrow_mut().tick();

        for _ in 0..4 {
            self.tick();
        }
    }

    /// returns the new flag value that should be set
    #[inline(always)]
    pub fn handle_interupt(&mut self, addr: u8) -> bool {
        self.mode = CPUMode::Normal;

        if self.ime == false {
            self.mode = CPUMode::Normal;
            return true;
        } else {
            self.ime = false;
            self.tick4();
            self.tick4();
            // Push current PC onto the stack
            self.sp = self.sp.wrapping_sub(1);
            self.write_byte(self.sp, (self.pc >> 8) as u8);
            self.sp = self.sp.wrapping_sub(1);
            self.write_byte(self.sp, self.pc as u8);

            // Set PC to the new address
            self.pc = addr as u16;
            self.tick4();

            return false;
        }
    }

    pub fn do_step(&mut self) {
        if self.mode == CPUMode::Normal {
            self.run_instr();
        } else {
            self.tick4();
        }

        if self.ime || (self.mode != CPUMode::Normal) {
            let mmu = self.mmu.borrow();
            let ic_ref = mmu.ic.clone();
            let ic = ic_ref.borrow();

            let flags = ic.interrupt_enable & ic.interrupt_flag;
            drop(ic);
            drop(mmu);

            if flags.vblank() {
                debug!("VBlank interrupt");
                debug!("PPU Tick: {}", self.mmu.borrow().ppu.t_cycles);
                let f = self.handle_interupt(0x40);
                ic_ref.borrow_mut().interrupt_flag.set_vblank(f);
            } else if flags.lcd() {
                debug!("LCD interrupt");
                let f = self.handle_interupt(0x48);
                ic_ref.borrow_mut().interrupt_flag.set_lcd(f);
            } else if flags.timer() {
                debug!("Timer interrupt");
                let f = self.handle_interupt(0x50);
                ic_ref.borrow_mut().interrupt_flag.set_timer(f);
            } else if flags.serial() {
                debug!("Serial interrupt");
                let f = self.handle_interupt(0x58);
                ic_ref.borrow_mut().interrupt_flag.set_serial(f);
            } else if flags.joypad() {
                debug!("Joypad interrupt");
                let f = self.handle_interupt(0x60);
                ic_ref.borrow_mut().interrupt_flag.set_joypad(f);
            }
        }
    }

    /* Helper methods */
    pub fn peek_opcode(&self) -> u8 {
        self.mmu.borrow().read(self.pc)
    }
}

use std::fmt;

impl fmt::Display for CPU {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "PC: {:04X} SP: {:04X}\
            A: {:02X} F: {:02X} \
            B: {:02X} C: {:02X} \
            D: {:02X} E: {:02X} \
            H: {:02X} L: {:02X}",
            self.pc,
            self.sp,
            self.a(),
            self.f(),
            self.b(),
            self.c(),
            self.d(),
            self.e(),
            self.h(),
            self.l(),
        )
    }
}

#[macro_export]
macro_rules! read_register {
    ($self:ident, $reg:expr) => {
        match $reg {
            Register8::A => $self.a(),
            Register8::B => $self.b(),
            Register8::C => $self.c(),
            Register8::D => $self.d(),
            Register8::E => $self.e(),
            Register8::H => $self.h(),
            Register8::L => $self.l(),
            Register8::IndirectHL => $self.read_byte($self.hl.value()),
        }
    };
    () => {};
}

#[macro_export]
macro_rules! write_register {
    ($self:ident, $reg:expr, $value:expr) => {
        match $reg {
            Register8::A => *$self.a_mut() = $value,
            Register8::B => *$self.b_mut() = $value,
            Register8::C => *$self.c_mut() = $value,
            Register8::D => *$self.d_mut() = $value,
            Register8::E => *$self.e_mut() = $value,
            Register8::H => *$self.h_mut() = $value,
            Register8::L => *$self.l_mut() = $value,
            Register8::IndirectHL => $self.write_byte($self.hl.value(), $value),
        }
    };
    () => {};
}

#[macro_export]
macro_rules! read_register16 {
    ($self:ident, $reg:expr) => {
        match $reg {
            Register16::AF => $self.af.value(),
            Register16::BC => $self.bc.value(),
            Register16::DE => $self.de.value(),
            Register16::HL => $self.hl.value(),
            Register16::SP => $self.sp,
            Register16::PC => $self.pc,
        }
    };
}

#[macro_export]
macro_rules! write_register16 {
    ($self:ident, $reg:expr, $value:expr) => {
        match $reg {
            Register16::AF => *$self.af.value_mut() = $value,
            Register16::BC => *$self.bc.value_mut() = $value,
            Register16::DE => *$self.de.value_mut() = $value,
            Register16::HL => *$self.hl.value_mut() = $value,
            Register16::SP => $self.sp = $value,
            Register16::PC => $self.pc = $value,
        }
    };
}
