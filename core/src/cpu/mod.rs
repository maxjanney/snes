pub mod registers;
pub mod interpreter;


use registers::Registers;

pub struct Cpu {
    pub regs: Registers,
}