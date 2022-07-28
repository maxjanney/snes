#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum AddressingMode {
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