use dmg::cpu::CPU;
use dmg::memory::MMU;
use dmg::memory::*;

use std::cell::RefCell;
use std::io::Read;
use std::rc::Rc;
use std::rc::Weak;

use rstest::*;

#[rstest]
fn for_each_file(
    #[files("./tests/mooneye-test-suite/**/**/*.gb")]
    #[files("./tests/mooneye-test-suite/emulator-only/**/*.gb")]
    #[mode = bytes]
    #[exclude("sgb")]
    #[exclude("mbc2")]
    #[exclude("dmg0")]
    #[exclude("mgb")]
    #[exclude("vgb")]
    #[exclude("agb")]
    #[exclude("ags")]
    #[exclude("-S")]
    rom: &[u8],
) {
    test_mooneye_rom(rom);
}

fn test_mooneye_rom(rom: &[u8]) {
    let mut bootrom = BootRom::new();
    let _ = bootrom.load(include_bytes!("../../roms/dmg_boot.bin"));

    let dma = Rc::new(RefCell::new(dmg::memory::dma::DMA::new(Weak::new())));
    let rom = mbc::MBC::new(rom.to_vec());

    let mmu: Rc<RefCell<MMU>> = MMU::new(Some(rom), bootrom.clone(), dma.clone());

    let mut cpu = CPU::new(mmu.clone());

    loop {
        cpu.do_step();

        let pc = cpu.pc;
        let mmu = mmu.borrow();

        if mmu.read(pc) == 0x18 {
            if mmu.read(pc + 1) == (-2 as i8 as u8) {
                break;
            }
        }
    }

    assert_eq!(cpu.b(), 3);
    assert_eq!(cpu.c(), 5);
    assert_eq!(cpu.d(), 8);
    assert_eq!(cpu.e(), 13);
    assert_eq!(cpu.h(), 21);
    assert_eq!(cpu.l(), 34);
}
