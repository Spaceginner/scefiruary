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
    fn advance_memory(&mut self) -> u8 {
        let value = self.memory[self.registry.instruction as usize];
        self.registry.instruction += 1;
        value
    }

    pub fn tick(&mut self) -> Result<(), CPUHalted> {
        if self.state.halted { return Err(CPUHalted); };

        // TODO perhaps somehow separate those?
        match Instruction::from(self.advance_memory()) {
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

            instruction => todo!("{:?}", instruction),
        };

        Ok(())
    }
}
