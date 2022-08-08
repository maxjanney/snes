use crate::{cpu::bus::AccessWidth, emu::Snes};

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

fn read_word(emu: &mut Snes, addr: u32) -> u32 {
    read_byte(emu, addr) as u32 | (read_byte(emu, addr.wrapping_add(1)) as u32) << 8
}

fn read_word_bank_zero(emu: &mut Snes, addr: u16) -> u16 {
    (read_byte(emu, addr as u32) as u16) | (read_byte(emu, addr.wrapping_add(1) as u32) as u16) << 8
}

fn read_long_bank_zero(emu: &mut Snes, addr: u16) -> u32 {
    (read_byte(emu, addr.wrapping_add(2) as u32) as u32) << 16
        | read_word_bank_zero(emu, addr) as u32
}

fn read_indirect_addr(emu: &mut Snes, addr: u16) -> u32 {
    emu.cpu.regs.data_bank() | read_word_bank_zero(emu, addr) as u32
}

fn read_imm<T: AccessWidth>(emu: &mut Snes) -> T {
    if T::IS_16 {
        todo!()
    } else {
        let val = read_byte(emu, emu.cpu.regs.program_bank() | emu.cpu.regs.pc as u32);
        emu.cpu.regs.pc = emu.cpu.regs.pc.wrapping_add(1);
        T::from_u8(val)
    }
}

fn effective_address<T: AccessWidth, const ADDR_MODE: AddressingMode>(emu: &mut Snes) -> u32 {
    // TODO: timings
    match ADDR_MODE {
        Direct => direct_page_address(emu) as u32,
        Absolute => absolute_address(emu),
        DirectIndirect => {
            let addr = direct_page_address(emu);
            read_indirect_addr(emu, addr)
        }
        DirectXIndirect => {
            let addr = direct_page_address(emu).wrapping_add(emu.cpu.regs.x);
            read_indirect_addr(emu, addr)
        }
        DirectIndirectY => {
            let indirect = direct_page_address(emu);
            let unindexed = read_indirect_addr(emu, indirect);
            (unindexed + emu.cpu.regs.y as u32) & 0xffffff
        }
        DirectIndirectLong => {
            let indirect = direct_page_address(emu);
            read_long_bank_zero(emu, indirect)
        }
        DirectIndirectLongY => {
            let indirect = direct_page_address(emu);
            let unindexed = read_long_bank_zero(emu, indirect);
            (unindexed + emu.cpu.regs.y as u32) & 0xffffff
        }
        DirectX => {
            let direct = direct_page_address(emu);
            direct.wrapping_add(emu.cpu.regs.x) as u32
        }
        DirectY => {
            let direct = direct_page_address(emu);
            direct.wrapping_add(emu.cpu.regs.y) as u32
        }
        AbsoluteX => {
            let unindexed = absolute_address(emu);
            (unindexed + emu.cpu.regs.x as u32) & 0xffffff
        }
        AbsoluteY => {
            let unindexed = absolute_address(emu);
            (unindexed + emu.cpu.regs.y as u32) & 0xffffff
        }
        StackRelative => stack_relative(emu) as u32,
        StackRelativeIndirectY => {
            let indirect = stack_relative(emu);
            let unindexed = read_indirect_addr(emu, indirect);
            (unindexed + emu.cpu.regs.y as u32) & 0xffffff
        }
        _ => todo!(),
    }
}

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

fn stack_relative(emu: &mut Snes) -> u16 {
    (read_imm::<u8>(emu) as u16).wrapping_add(emu.cpu.regs.sp)
}
