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

pub struct Registers {
    a: u16,
    x: u16,
    y: u16,
    pc: u16,
    sp: u16,
    psr: Psr,
    d: u16,
    db: u8,
    pb: u8,
    emu_mode: bool,
}

impl Registers {
    pub(crate) fn new() -> Self {
        Self {
            a: 0,
            x: 0,
            y: 0,
            pc: 0,
            sp: 0x1FC,
            psr: Psr::default(),
            d: 0,
            db: 0,
            pb: 0,
            emu_mode: false,
        }
    }
}
