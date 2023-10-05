#[derive(Default, Debug)]
pub struct Registry {
    pub generic_a: u16,
    pub generic_b: u16,
    pub generic_c: u16,
    pub generic_d: u16,
    pub generic_e: u16,

    pub stack_ptr: u16,
    pub instruction_ptr: u16,
    pub flag_bank: u16,

    pub operand_a: u16,
    pub operand_b: u16,
    pub operand_c: u16,
    pub display_a: u16,
    pub display_b: u16,

    pub parameter_a: u16,
    pub parameter_b: u16,
    pub parameter_c: u16,

    pub return_a: u16,
    pub return_b: u16,

    pub burner_a: u16,
    pub burner_b: u16,
}


#[derive(Debug)]
pub enum RegistryError {
    OutOfBounds(u8),
}


impl Registry {
    pub fn by_address(&self, address: u8) -> Result<&u16, RegistryError> {
        match address {
            0 => Ok(&self.generic_a),
            1 => Ok(&self.generic_b),
            2 => Ok(&self.generic_c),
            3 => Ok(&self.generic_d),
            4 => Ok(&self.generic_e),

            5 => Ok(&self.stack_ptr),
            6 => Ok(&self.instruction_ptr),
            7 => Ok(&self.flag_bank),

            8 => Ok(&self.operand_a),
            9 => Ok(&self.operand_b),
            10 => Ok(&self.operand_c),

            11 => Ok(&self.display_a),
            12 => Ok(&self.display_b),

            13 => Ok(&self.parameter_a),
            14 => Ok(&self.parameter_b),
            15 => Ok(&self.parameter_c),

            16 => Ok(&self.return_a),
            17 => Ok(&self.return_b),

            18 => Ok(&self.burner_a),
            19 => Ok(&self.burner_b),

            addr => Err(RegistryError::OutOfBounds(addr)),
        }
    }

    pub fn by_address_mut(&mut self, address: u8) -> Result<&mut u16, RegistryError> {
        match address {
            0 => Ok(&mut self.generic_a),
            1 => Ok(&mut self.generic_b),
            2 => Ok(&mut self.generic_c),
            3 => Ok(&mut self.generic_d),
            4 => Ok(&mut self.generic_e),

            5 => Ok(&mut self.stack_ptr),
            6 => Ok(&mut self.instruction_ptr),
            7 => Ok(&mut self.flag_bank),

            8 => Ok(&mut self.operand_a),
            9 => Ok(&mut self.operand_b),
            10 => Ok(&mut self.operand_c),

            11 => Ok(&mut self.display_a),
            12 => Ok(&mut self.display_b),

            13 => Ok(&mut self.parameter_a),
            14 => Ok(&mut self.parameter_b),
            15 => Ok(&mut self.parameter_c),

            16 => Ok(&mut self.return_a),
            17 => Ok(&mut self.return_b),

            18 => Ok(&mut self.burner_a),
            19 => Ok(&mut self.burner_b),

            addr => Err(RegistryError::OutOfBounds(addr)),
        }
    }

    /// copy registry A into B
    pub fn copy_by_address(&mut self, address_a: u8, address_b: u8) -> Result<(), RegistryError> {
        let registry_a_value = *self.by_address(address_a)?;
        let registry_b = self.by_address_mut(address_b)?;

        *registry_b = registry_a_value;

        Ok(())
    }
}
