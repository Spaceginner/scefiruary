mod arithmetic {
    fn sum(operand_a: i16, operand_b: i16) -> i16 {
        operand_a + operand_b
    }

    fn subtraction(operand_a: i16, operand_b: i16) -> i16 {
        operand_a - operand_b
    }

    fn multiplication(operand_a: i16, operand_b: i16) -> i16 {
        operand_a * operand_b
    }
}

mod bitwise {
    fn and(operand_a: i16, operand_b: i16) -> i16 {
        operand_a & operand_b
    }

    fn or(operand_a: i16, operand_b: i16) -> i16 {
        operand_a | operand_b
    }

    fn xor(operand_a: i16, operand_b: i16) -> i16 {
        operand_a ^ operand_b
    }
}

mod bitshift {
    fn right(operand_a: i16, operand_b: i16) -> i16 {
        operand_a >> operand_b
    }

    fn left(operand_a: i16, operand_b: i16) -> i16 {
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
mod compare {
    fn signed(operand_a: i16, operand_b: i16) -> u16 {
        let mut compared = 0;

        if operand_a == operand_b {
            compared |= 0b1;
        };

        if operand_a > operand_b {
            compared |= 0b10;
        }

        compared
    }

    fn unsigned(operand_a: u16, operand_b: u16) -> u16 {
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
