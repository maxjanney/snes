pub trait AccessWidth: Copy {
    const IS_16: bool;

    fn from_u8(val: u8) -> Self;
    fn from_u16(val: u16) -> Self;
    fn as_u8(self) -> u8;
    fn as_u16(self) -> u16;
}

impl AccessWidth for u8 {
    const IS_16: bool = false;

    fn from_u8(val: u8) -> Self {
        val
    }

    fn from_u16(val: u16) -> Self {
        val as u8
    }

    fn as_u8(self) -> u8 {
        self
    }

    fn as_u16(self) -> u16 {
        self as u16
    }
}

impl AccessWidth for u16 {
    const IS_16: bool = true;

    fn from_u8(val: u8) -> Self {
        val as u16
    }

    fn from_u16(val: u16) -> Self {
        val
    }

    fn as_u8(self) -> u8 {
        self as u8
    }

    fn as_u16(self) -> u16 {
        self
    }
}
