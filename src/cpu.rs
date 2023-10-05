use std::fmt;
use std::fmt::Formatter;
use std::ops::{BitOr, Shl};
use std::mem::size_of;
use num_traits::Num;
use crate::alu;
use crate::instruction::Instruction;
use crate::registry::{Registry, RegistryError};


#[derive(Default)]
struct CPUState {
    exception: Option<CPUException>,
    cycles: u32,
    instructions: u32,
}


pub struct CPU {
    registry: Registry,
    memory: [u8; u16::MAX as usize + 1],
    state: CPUState,
}


impl Default for CPU {
    fn default() -> Self {
        Self {
            registry: Default::default(),
            memory: [0; u16::MAX as usize + 1],
            state: Default::default(),
        }
    }
}


#[derive(Debug, Copy, Clone)]
pub enum CPUException {
    Halted,
    InvalidInstruction(u8, u16), InvalidRegistry(u8),
}


impl fmt::Display for CPUException {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            CPUException::Halted => write!(f, "CPU was halted"),
            CPUException::InvalidInstruction(instr, addr) => write!(f, "instruction 0x{:X} at 0x{:X} was not recognised", instr, addr),
            CPUException::InvalidRegistry(reg) => write!(f, "registry address 0x{:X} was not recognised", reg),
        }
    }
}


impl CPU {
    // FIXME come up with a better name for this function
    fn advance_memory<T>(&mut self) -> T
        where T: Sized + Num + Shl<Output = T> + From<u8> + BitOr<Output = T>
    {
        let mut value = T::zero();

        for i in 0..size_of::<T>() {
            // get next byte
            let next_byte = self.memory[self.registry.instruction as usize];
            self.registry.instruction += 1;

            // store da thing
            if i > 0 {
                value = value << T::from(8) | T::from(next_byte);
            } else {
                value = T::from(next_byte);
            };

            // yes, it takes 1 cycle to get 1 byte
            // and we are doing here wrapping add just. in. case.
            self.state.cycles = self.state.cycles.wrapping_add(1);
        };

        value
    }

    fn flag_mask(flag_address: u16) -> u16 {
        1 << ((flag_address as u8) << 4 >> 4)
    }

    /// copy data into memory with an offset
    /// all data which goes out of bounds, will be ignored
    pub fn load_memory(&mut self, offset: u16, data: &[u8]) {
        for (index, byte) in data.iter().enumerate().take(u16::MAX as usize + 1 - offset as usize) {
            self.memory[index + offset as usize] = *byte;
        };
    }

    pub fn tick(&mut self) -> Result<(u16, u16, u16, u16, u32, u32), CPUException> {
        if let Some(exception) = self.state.exception {
            return Err(exception);
        };

        // TODO perhaps somehow separate those?
        match Instruction::from(self.advance_memory::<u8>()) {
            // special
            Instruction::Noop => (),
            Instruction::Halt => self.state.exception = Some(CPUException::Halted),

            // alu
            Instruction::Sum => self.registry.operand_c = alu::arithmetic::sum(self.registry.operand_a, self.registry.operand_b),
            Instruction::Subtraction => self.registry.operand_c = alu::arithmetic::subtraction(self.registry.operand_a, self.registry.operand_b),
            Instruction::Multiplication => self.registry.operand_c = alu::arithmetic::multiplication(self.registry.operand_a, self.registry.operand_b),
            Instruction::BitwiseAND => self.registry.operand_c = alu::bitwise::and(self.registry.operand_a, self.registry.operand_b),
            Instruction::BitwiseOR => self.registry.operand_c = alu::bitwise::or(self.registry.operand_a, self.registry.operand_b),
            Instruction::BitwiseXOR => self.registry.operand_c = alu::bitwise::xor(self.registry.operand_a, self.registry.operand_b),
            Instruction::BitshiftRight => self.registry.operand_c = alu::bitshift::right(self.registry.operand_a, self.registry.operand_b),
            Instruction::BitshiftLeft => self.registry.operand_c = alu::bitshift::left(self.registry.operand_a, self.registry.operand_b),
            Instruction::CompareUnsigned => self.registry.operand_c = alu::compare::unsigned(self.registry.operand_a, self.registry.operand_b),
            Instruction::CompareSigned => self.registry.operand_c = alu::compare::signed(self.registry.operand_a as i16, self.registry.operand_b  as i16),

            // memory management
            Instruction::Copy => {
                let registry_a = self.advance_memory();
                let registry_b = self.advance_memory();

                match self.registry.copy_by_address(registry_a, registry_b) {
                    Ok(()) => (),
                    Err(RegistryError::OutOfBounds(addr)) => return Err(CPUException::InvalidRegistry(addr)),
                };
            },
            Instruction::Move => {
                self.registry.operand_a = self.advance_memory();
            },
            Instruction::CopyIfZero => {
                let registry_a = self.advance_memory();
                let registry_b = self.advance_memory();

                if self.registry.operand_a == 0 {
                    let _ = self.registry.copy_by_address(registry_a, registry_b);
                };
            },
            Instruction::CopyIfNotZero => {
                let registry_a = self.advance_memory();
                let registry_b = self.advance_memory();

                if self.registry.operand_a != 0 {
                    match self.registry.copy_by_address(registry_a, registry_b) {
                        Ok(()) => (),
                        Err(RegistryError::OutOfBounds(addr)) => return Err(CPUException::InvalidRegistry(addr)),
                    };
                };
            },
            Instruction::MoveIfZero => {
                if self.registry.operand_b == 0 {
                    self.registry.operand_a = self.advance_memory();
                };
            },
            Instruction::MoveIfNotZero => {
                if self.registry.operand_b != 0 {
                    self.registry.operand_a = self.advance_memory();
                };
            },
            Instruction::Put => {
                let address = self.registry.operand_a as usize;
                let value = self.registry.operand_b as u8;

                self.memory[address] = value;
            },
            Instruction::Get => {
                let address = self.registry.operand_a as usize;

                self.registry.operand_b = self.memory[address] as u16;
            },

            // flag instructions
            Instruction::Test => {
                if (self.registry.flag_bank & Self::flag_mask(self.registry.operand_a)) != 0 {
                    self.registry.operand_c = self.registry.operand_b;
                };
            },
            Instruction::Set => {
                self.registry.flag_bank |= Self::flag_mask(self.registry.operand_a);
            },
            Instruction::Unset => {
                self.registry.flag_bank &= !Self::flag_mask(self.registry.operand_a);
            },
            Instruction::SetIfZero => {
                if self.registry.operand_a == 0 {
                    self.registry.flag_bank |= Self::flag_mask(self.registry.operand_a);
                };
            },
            Instruction::SetIfNotZero => {
                if self.registry.operand_a != 0 {
                    self.registry.flag_bank |= Self::flag_mask(self.registry.operand_a);
                };
            },
            Instruction::UnsetIfZero => {
                if self.registry.operand_a == 0 {
                    self.registry.flag_bank &= !Self::flag_mask(self.registry.operand_a);
                };
            },
            Instruction::UnsetIfNotZero => {
                if self.registry.operand_a != 0 {
                    self.registry.flag_bank &= !Self::flag_mask(self.registry.operand_a);
                };
            },

            instruction => todo!("{:?}", instruction),
        };

        self.state.instructions = self.state.instructions.wrapping_add(1);

        Ok((
            self.registry.display_a, self.registry.display_b,
            self.registry.instruction, self.registry.generic_f,
            self.state.cycles, self.state.instructions
        ))
    }

    // pub fn get_display(&self) -> (u16, u16) {
    //     (self.registry.display_a, self.registry.display_b)
    // }
}
