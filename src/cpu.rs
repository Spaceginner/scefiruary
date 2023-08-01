use std::ops::Shl;
use std::mem::size_of;
use num_traits::Zero;
use crate::alu;
use crate::instruction::Instruction;
use crate::registry::Registry;


#[derive(Default)]
struct CPUState {
    halted: bool,
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


pub struct CPUHalted;


impl CPU {
    // FIXME come up with a better name for this function
    fn advance_memory<T>(&mut self) -> T
        where T: Sized + Zero + Shl<Output = T> + From<u8>
    {
        let mut value = Zero::zero();

        for _ in 0..size_of::<T>() {
            // get next byte
            let next_byte = self.memory[self.registry.instruction as usize];
            self.registry.instruction += 1;

            // store da thing
            value = value << T::from(8) + T::from(next_byte);
        };

        value
    }

    /// copy data into memory with an offset
    /// all data which goes out of bounds, will be ignored
    pub fn load_memory(&mut self, offset: u16, data: &[u8]) {
        for (index, byte) in data.iter().enumerate().take(u16::MAX as usize + 1 - offset as usize) {
            self.memory[index + offset as usize] = *byte;
        };
    }

    pub fn tick(&mut self) -> Result<(), CPUHalted> {
        if self.state.halted { return Err(CPUHalted); };

        // TODO perhaps somehow separate those?
        match Instruction::from(self.advance_memory::<u8>()) {
            // special
            Instruction::Noop => (),
            Instruction::Halt => self.state.halted = true,

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

                let _ = self.registry.copy_by_address(registry_a, registry_b);
            },
            Instruction::Move => {
                let registry_address = self.advance_memory();
                let value = self.advance_memory();

                let _ = self.registry.move_by_address(registry_address, value);
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
                    let _ = self.registry.copy_by_address(registry_a, registry_b);
                };
            },
            Instruction::MoveIfZero => {
                let registry_address = self.advance_memory();
                let value = self.advance_memory();

                if self.registry.operand_a == 0 {
                    let _ = self.registry.move_by_address(registry_address, value);
                };
            },
            Instruction::MoveIfNotZero => {
                let registry_address = self.advance_memory();
                let value = self.advance_memory();

                if self.registry.operand_a != 0 {
                    let _ = self.registry.move_by_address(registry_address, value);
                };
            },
            Instruction::Put => {
                let address = self.advance_memory::<u16>() as usize;
                let value = self.registry.operand_a as u8;

                self.memory[address] = value;
            },
            Instruction::Get => {
                let address = self.advance_memory::<u16>() as usize;

                self.registry.operand_a = self.memory[address] as u16;
            }

            instruction => todo!("{:?}", instruction),
        };

        Ok(())
    }
}
