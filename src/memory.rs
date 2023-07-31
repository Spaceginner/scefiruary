#[derive(Default, Debug)]
pub struct Memory {
    registry: [u16; 13],
    memory: [u8; u16::MAX + 1],
}
