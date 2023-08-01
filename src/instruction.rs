#[derive(Debug, Default)]
pub enum Instruction {
    // special
    #[default]
    Noop, Halt,

    // alu
    Sum, Subtraction, Multiplication,
    BitwiseAND, BitwiseOR, BitwiseXOR,
    BitshiftRight, BitshiftLeft,
    CompareSigned, CompareUnsigned,

    // memory management
    Copy, Move,
    CopyIfZero, CopyIfNotZero,
    MoveIfZero, MoveIfNotZero,
    Put, Get,

    // flag instructions
    Test,
    Set, Unset,
    SetIfZero, SetIfNotZero,
    UnsetIfZero, UnsetIfNotZero,

    // bus communication
    Send, Receive,
}


impl From<u8> for Instruction {
    fn from(value: u8) -> Self {
        match value {
            0b00000000 => Self::Noop,
            0b00000001 => Self::Halt,

            0b00010000 => Self::Sum,
            0b00010001 => Self::Subtraction,
            0b00010010 => Self::Multiplication,
            0b00010100 => Self::BitwiseAND,
            0b00010101 => Self::BitwiseOR,
            0b00010110 => Self::BitwiseXOR,
            0b00011000 => Self::BitshiftRight,
            0b00011001 => Self::BitshiftLeft,
            0b00011100 => Self::CompareSigned,
            0b00011101 => Self::CompareUnsigned,

            0b00110000 => Self::Copy,
            0b00110001 => Self::Move,
            0b00110100 => Self::CopyIfZero,
            0b00110101 => Self::CopyIfNotZero,
            0b00110110 => Self::MoveIfZero,
            0b00110111 => Self::MoveIfNotZero,
            0b00111000 => Self::Put,
            0b00111001 => Self::Get,

            0b01000000 => Self::Test,
            0b01000010 => Self::Set,
            0b01000011 => Self::Unset,
            0b01000100 => Self::SetIfZero,
            0b01000101 => Self::SetIfNotZero,
            0b01000110 => Self::UnsetIfZero,
            0b01000111 => Self::UnsetIfNotZero,

            0b01010000 => Self::Send,
            0b01010001 => Self::Receive,

            _ => Self::Noop,
        }
    }
}
