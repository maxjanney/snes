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

pub trait RegisterSize: Copy {
    const IS_16: bool;

    fn from_u8(val: u8) -> Self;
    fn as_u8(self) -> u8;
    fn as_u16(self) -> u16;
}

impl RegisterSize for u8 {
    const IS_16: bool = false;

    fn from_u8(val: u8) -> Self {
        val
    }

    fn as_u8(self) -> u8 {
        self
    }

    fn as_u16(self) -> u16 {
        self as u16
    }
}

impl RegisterSize for u16 {
    const IS_16: bool = true;

    fn from_u8(val: u8) -> Self {
        val as u16
    }

    fn as_u8(self) -> u8 {
        self as u8
    }

    fn as_u16(self) -> u16 {
        self
    }
}

fn read_byte(emu: &mut Snes, addr: u32) -> u8 {
    todo!()
}

// read an 8 or 16 bit immediate value
fn read_imm<T: RegisterSize>(emu: &mut Snes) -> T {
    if T::IS_16 {
        todo!()
    } else {
        let val = read_byte(emu, emu.cpu.regs.program_bank() | emu.cpu.regs.pc as u32);
        emu.cpu.regs.pc = emu.cpu.regs.pc.wrapping_add(1);
        T::from_u8(val)
    }
}

// calculate the effective address for the given addressing mode
fn effective_address<T: RegisterSize, const ADDR_MODE: AddressingMode>(emu: &mut Snes) -> u32 {
    match ADDR_MODE {
        Direct => direct_page_address(emu) as u32,
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
