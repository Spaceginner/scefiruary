pub mod arithmetic {
    use std::num::Wrapping;

    pub fn sum(operand_a: u16, operand_b: u16) -> u16 {
        (Wrapping(operand_a) + Wrapping(operand_b)).0
    }

    pub fn subtraction(operand_a: u16, operand_b: u16) -> u16 {
        (Wrapping(operand_a) - Wrapping(operand_b)).0
    }

    pub fn multiplication(operand_a: u16, operand_b: u16) -> u16 {
        (Wrapping(operand_a) * Wrapping(operand_b)).0
    }
}

pub mod bitwise {
    pub fn and(operand_a: u16, operand_b: u16) -> u16 {
        operand_a & operand_b
    }

    pub fn or(operand_a: u16, operand_b: u16) -> u16 {
        operand_a | operand_b
    }

    pub fn xor(operand_a: u16, operand_b: u16) -> u16 {
        operand_a ^ operand_b
    }
}

pub mod bitshift {
    pub fn right(operand_a: u16, operand_b: u16) -> u16 {
        operand_a >> operand_b
    }

    pub fn left(operand_a: u16, operand_b: u16) -> u16 {
        operand_a << operand_b
    }

    // mod rot {
    //     use std::mem::size_of;
    //
    //     fn right(operand_a: i16, operand_b: u16) -> i16 {
    //         let operand_b = operand_b % size_of::<u16>();
    //         operand_a >> operand_b + operand_a << (size_of::<u16>() - operand_b)
    //     }
    //
    //     fn left(operand_a: i16, operand_b: u16) -> i16 {
    //         let operand_b = operand_b % size_of::<u16>();
    //         operand_a << operand_b + operand_a >> (size_of::<u16>() - operand_b)
    //     }
    // }
}

// TODO perhaps make it drier somehow, or smth, idk
pub mod compare {
    pub fn signed(operand_a: i16, operand_b: i16) -> u16 {
        let mut compared = 0;

        if operand_a == operand_b {
            compared |= 0b1;
        };

        if operand_a > operand_b {
            compared |= 0b10;
        }

        compared
    }

    pub fn unsigned(operand_a: u16, operand_b: u16) -> u16 {
        let mut compared = 0;

        if operand_a == operand_b {
            compared |= 0b1;
        };

        if operand_a > operand_b {
            compared |= 0b10;
        }

        compared
    }
}
