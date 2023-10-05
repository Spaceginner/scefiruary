#![feature(try_blocks)]

use std::thread;
use std::time::Duration;
use scefiruary::cpu::{CPU, CPUException};


/// example program - fibonacci sequence
const FIBONACCI_PROGRAM: &[u8] = &[
    // python pseudocode                 // num (byte): asm
    // a = 0
    0b00110001, 0b00000000, 0b00000000,  //  1 ( 0): mv 0
    0b00110000, 0b00001000, 0b00000000,  //  2 ( 3): cp roa, rga
    // b = 1
    0b00110001, 0b00000000, 0b00000001,  //  3 ( 6): mv 1
    0b00110000, 0b00001000, 0b00000001,  //  4 ( 9): cp roa, rgb
    // counter = 0
    0b00110001, 0b00000000, 0b00000000,  //  5 (12): mv 0
    0b00110000, 0b00001000, 0b00000010,  //  6 (15): cp roa, rgc
    // print(a, counter)
    // LABEL: fib_loop
    0b00110000, 0b00000000, 0b00001011,  //  7 (18): cp rga, rda
    0b00110000, 0b00000010, 0b00001100,  //  8 (21): cp rgc, rdb
    // counter += 1
    0b00110001, 0b00000000, 0b00000001,  //  9 (24): mv 1
    0b00110000, 0b00000010, 0b00001001,  // 10 (27): cp rgc, rob
    0b00010000,                          // 11 (30): sum
    0b00110000, 0b00001010, 0b00000010,  // 12 (31): cp roc, rgc
    // check = compare(0x7FFF, a)
    0b00110000, 0b00000000, 0b00001001,  // 13 (34): cp rga, rob
    0b00110001, 0b01111111, 0b11111111,  // 14 (37): mv 0x7FFF
    0b00011100,                          // 15 (40): cmpr
    0b00110000, 0b00001010, 0b00000011,  // 16 (41): cp roc, rgd
    // c = b + a
    0b00110000, 0b00000001, 0b00001000,  // 17 (44): cp rgb, roa
    0b00010000,                          // 18 (47): sum
    // a = b
    0b00110000, 0b00000001, 0b00000000,  // 19 (48): cp rgb, rga
    // b = c
    0b00110000, 0b00001010, 0b00000001,  // 20 (51): cp roc, rgb
    // if check != 0: continue
    0b00110001, 0b00000000, 0b00010010,  // 21 (54): mv fib_loop
    0b00110000, 0b00001000, 0b00001001,  // 22 (57): cp roa, rob
    0b00110000, 0b00000011, 0b00001000,  // 22 (63): cp rgd, roa
    0b00110101, 0b00001001, 0b00000110,  // 23 (66): cpnz rob, rip

    0b00000001,                          // 24 (67): hlt
];
// ...i should prob write a compiler, even if it is for ASM o_o
// i just was debugging an emulator, thinking *it* was the issue
// but apparently not, i just messed up the instruction when was "compiling" the program
// even emulated CPUs are always correct


const FREQUENCY: f64 = 15.0;


fn main() {
    let mut cpu = CPU::default();
    cpu.load_memory(0, FIBONACCI_PROGRAM);

    // while let Ok((display_a, display_b, instruction_ptr, stack_ptr, cycles, instructions)) = cpu.tick() {
    //     println!("RDA: {display_a:>5}  RDB: {display_b:>5} RIP: {instruction_ptr:>5} RSP: {stack_ptr:>5} CYCLES: {cycles}  INSTRUCTIONS: {instructions}");
    //
    //     thread::sleep(Duration::from_secs_f64(1.0 / FREQUENCY));
    // };

    let cpu_exception: Result<(), CPUException> = try {
        while true {  // so that rustrover wouldnt complain about "unreachable" code meh meh meh
            let (display_a, display_b, instruction_ptr, stack_ptr, cycles, instructions) = cpu.tick()?;

            println!("RDA: {display_a:>5}  RDB: {display_b:>5}  |  RIP: {instruction_ptr:>5}  RSP: {stack_ptr:>5}  |  CYCLES: {cycles}  INSTRUCTIONS: {instructions}");

            thread::sleep(Duration::from_secs_f64(1.0 / FREQUENCY));
        };
    };

    println!("\nCPU EXCEPTED - {}", cpu_exception.unwrap_err());
}
