use crate::cpu::bus::AccessWidth;
use crate::emu::Snes;

use self::AddressingMode::*;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum AddressingMode {
    Direct,
    Absolute,
    // indirect modes
    DirectIndirect,
    DirectXIndirect,
    DirectIndirectY,
    AbsoluteIndirect,
    AbsoluteXIndirect,
    DirectIndirectLong,
    DirectIndirectLongY,
    // indexed modes
    DirectX,
    DirectY,
    AbsoluteX,
    AbsoluteY,
    AbsoluteLongX,
    // stack relative
    StackRelative,
    StackRelativeIndirectY,
}

fn read_byte(emu: &mut Snes, addr: u32) -> u8 {
    todo!()
}

// read an 8 or 16 bit immediate value
fn read_imm<T: AccessWidth>(emu: &mut Snes) -> T {
    if T::IS_16 {
        todo!()
    } else {
        let val = read_byte(emu, emu.cpu.regs.program_bank() | emu.cpu.regs.pc as u32);
        emu.cpu.regs.pc = emu.cpu.regs.pc.wrapping_add(1);
        T::from_u8(val)
    }
}

// calculate the effective address for the given addressing mode
fn effective_address<T: AccessWidth, const ADDR_MODE: AddressingMode>(emu: &mut Snes) -> u32 {
    match ADDR_MODE {
        Direct => direct_page_address(emu) as u32,
        Absolute => absolute_address(emu),
        _ => todo!(),
    }
}

// calculate the effective address for direct page mode
fn direct_page_address(emu: &mut Snes) -> u16 {
    let dp = emu.cpu.regs.dp;
    let ea = dp.wrapping_add(read_imm::<u8>(emu) as u16);
    if dp != 0 {
        // extra cycle if the low byte of the direct Page register is not zero
    }
    ea
}

fn absolute_address(emu: &mut Snes) -> u32 {
    emu.cpu.regs.data_bank() | read_imm::<u16>(emu) as u32
}
