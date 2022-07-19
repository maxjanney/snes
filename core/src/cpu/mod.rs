pub mod registers;

use registers::Registers;

pub struct Cpu {
    pub regs: Registers,
}