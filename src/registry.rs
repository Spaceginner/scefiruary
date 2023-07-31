#[derive(Default, Debug)]
pub struct Registry {
    pub generic_a: u16,
    pub generic_b: u16,
    pub generic_c: u16,
    pub generic_d: u16,
    pub generic_e: u16,
    pub generic_f: u16,
    pub instruction: u16,
    pub flag_bank: u16,
    pub operand_a: u16,
    pub operand_b: u16,
    pub operand_c: u16,
    pub display_a: u16,
    pub display_b: u16,
}


pub struct OutOfBounds;


impl Registry {
    pub fn by_address(&self, address: u8) -> Result<&u16, OutOfBounds> {
        match address {
            0 => Ok(&self.generic_a),
            1 => Ok(&self.generic_b),
            2 => Ok(&self.generic_c),
            3 => Ok(&self.generic_d),
            4 => Ok(&self.generic_e),
            5 => Ok(&self.generic_f),

            6 => Ok(&self.instruction),
            7 => Ok(&self.flag_bank),

            8 => Ok(&self.operand_a),
            9 => Ok(&self.operand_b),
            10 => Ok(&self.operand_c),

            11 => Ok(&self.display_a),
            12 => Ok(&self.display_b),

            _ => Err(OutOfBounds),
        }
    }

    pub fn by_address_mut(&mut self, address: u8) -> Result<&mut u16, OutOfBounds> {
        match address {
            0 => Ok(&mut self.generic_a),
            1 => Ok(&mut self.generic_b),
            2 => Ok(&mut self.generic_c),
            3 => Ok(&mut self.generic_d),
            4 => Ok(&mut self.generic_e),
            5 => Ok(&mut self.generic_f),

            6 => Ok(&mut self.instruction),
            7 => Ok(&mut self.flag_bank),

            8 => Ok(&mut self.operand_a),
            9 => Ok(&mut self.operand_b),
            10 => Ok(&mut self.operand_c),

            11 => Ok(&mut self.display_a),
            12 => Ok(&mut self.display_b),

            _ => Err(OutOfBounds),
        }
    }
}
