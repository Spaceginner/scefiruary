use crate::registry::Registry;


pub struct CPU {
    registry: Registry,
    memory: [u8; u16::MAX as usize + 1],
}
