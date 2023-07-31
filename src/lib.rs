mod alu;
mod instruction;


pub struct CPU {
    registry: [u16; 13],
    memory: [u8; u16::MAX as usize + 1],
}

// TODO tests
