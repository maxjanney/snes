use bitflags::bitflags;

bitflags! {
    #[derive(Default)]
    pub struct Psr: u8 {
        // Carry (0=No Carry, 1=Carry)
        const C = 0b0000_0001;
        // Zero (0=Nonzero, 1=Zero)
        const Z = 0b0000_0010;
        // IRQ Disable (0=IRQ Enable, 1=IRQ Disable)
        const I = 0b0000_0100;
        // Decimal Mode (0=Normal, 1=BCD Mode for ADC/SBC opcodes)
        const D = 0b0000_1000;
        // Break Flag (0=IRQ/NMI, 1=BRK/PHP opcode)  (0=16bit, 1=8bit)
        const B = 0b0001_0000;
        // Unused (Always 1)
        const U = 0b0010_0000;
        // Overflow (0=No Overflow, 1=Overflow)
        const V = 0b0100_0000;
        // Negative/Sign (0=Positive, 1=Negative)
        const N = 0b1000_0000;
    }
}

#[derive(Default)]
pub struct Registers {
    // accumulator (a=lo, b=hi, c=both)
    a: u16,
    // x and y are paired, both must be in 8 bit or 16 bit at the same time
    // When x and y leave 16 bit mode, their high bytes get cleared to 0
    //
    // index register, can be treated as a 8 bits or 16 bits
    x: u16,
    // index register, can be treated as a 8 bits or 16 bits
    y: u16,
    // This register keeps track of the high and low bytes of the address of the currently executed instruction.
    // e.g. if there is an instruction executed at $018009, this register will hold the value $8009.
    pub pc: u16,
    // stack pointer, holds the pointer to the stack in RAM, relative to address $000000
    sp: u16,
    // processor status register, holds the current processor flags
    psr: Psr,
    // direct page used for the direct page addressing mode
    // when accessing a memory address by its direct page notation,
    // the value in the direct page is added to that address
    pub dp: u16,
    // data bank, holds the current data bank address. When accessing an address
    // using the absolute addressing notation, the system uses this register to
    // determine the bank of the given address.
    db: u32,
    // program bank, keeps track of the current bank of the currently executed instruction
    // e.g. if there is code executed at address $018009, this register will hold the value $01.
    pb: u32,
    // flag for 6502 emulation mode
    emu_mode: bool,
}

impl Registers {
    pub(crate) fn new() -> Self {
        Self {
            sp: 0x1FC,
            ..Default::default()
        }
    }

    pub(crate) const fn program_bank(&self) -> u32 {
        self.pb
    }

    pub(crate) fn set_program_bank(&mut self, val: u8) {
        self.pb = (val as u32) << 16
    }

    pub(crate) const fn data_bank(&self) -> u32 {
        self.db
    }

    pub(crate) fn set_data_bank(&mut self, val: u8) {
        self.db = (val as u32) << 16;
    }
}
