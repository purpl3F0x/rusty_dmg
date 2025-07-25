use std::rc::Weak;

use crate::memory::{boot_rom, dma, BootRom, DMA, MMU};
use crate::ppu::PPU;

use super::*;

mod test_0x;
mod test_1x;
mod test_2x;
mod test_3x;
mod test_4x;
mod test_5x;
mod test_6x;
mod test_7x;
mod test_8x;
mod test_9x;
mod test_ax;
mod test_bx;

mod test_cb;

struct TestHardware {
    pub cpu: CPU,
    pub instruction_counter: usize,
}

// #[macro_export]
// macro_rules! assert_eq_with_info {
//     ($struct:expr, $left:expr, $right:expr) => {{
//         if $left != $right {
//             eprintln!("[assert_eq_with_info] struct debug: {:?}", $struct);
//             panic!(
//                 "assertion failed: `(left == right)`\n  left: `{:?}`\n right: `{:?}`",
//                 $left, $right
//             );
//         }
//     }};
// }

fn run_test<I: Fn(&mut TestHardware) -> ()>(instructions: &[u8], init: I) -> TestHardware {
    let mut rom: [u8; 0x8000] = [0xFD; 0x8000];

    let mut i = 0;
    while i < instructions.len() && i < rom.len() - 1 {
        rom[i] = instructions[i];
        i += 1;
    }

    let mut boot_rom = BootRom::new();
    boot_rom.enabled = false;

    let dma = Rc::new(RefCell::new(DMA::new(Weak::new())));

    let mmu: Rc<RefCell<MMU>> = MMU::new(rom, boot_rom, dma.clone());
    let mut test_hardware = TestHardware {
        cpu: CPU::new(mmu),
        instruction_counter: 0,
    };

    init(&mut test_hardware);

    while test_hardware.cpu.peek_opcode() != 0xFD {
        println!(
            "Executing instruction: {:02X}",
            test_hardware.cpu.peek_opcode()
        );

        test_hardware.cpu.run_instr();
        test_hardware.instruction_counter += 1;
    }
    test_hardware
}
