#![feature(try_blocks)]

use std::{fs, thread};
use std::time::Duration;
use std::env::args;

use scefiruary::cpu::{CPU, CPUException};

const HELP_MESSAGE: &str = r#"sfemu
Scenite Firuary CPU emulator

USAGE: sfemu PATH [FREQUENCY]

ARGUMENTS:
    PATH         Path to the executable to run
    FREQUENCY    Frequency (in Hz) to run CPU ata
"#;

fn remind_user_how_to_use_this_shit() -> ! {
    println!("{HELP_MESSAGE}");
    std::process::exit(1);
}


const DEFAULT_FREQUENCY: f32 = 15.0;

fn main() {
    let mut args = args().skip(1);

    // load
    let program = fs::read(args.next().unwrap_or_else(|| remind_user_how_to_use_this_shit())).expect("couldnt read input file");

    let frequency = if let Some(freq) = args.next() {
        freq.parse().expect("invalid frequency")
    } else {
        DEFAULT_FREQUENCY
    };

    // execute
    let mut machine = CPU::default();
    machine.load_memory(0, &program);

    let cpu_exception: Result<(), CPUException> = try {
        while true {  // so that rustrover wouldnt complain about "unreachable" code meh meh meh
            let (display_a, display_b, instruction_ptr, stack_ptr, cycles, instructions) = machine.tick()?;

            println!("RDA: {display_a:>5}  RDB: {display_b:>5}  |  RIP: {instruction_ptr:>5}  RSP: {stack_ptr:>5}  |  CYCLES: {cycles}  INSTRUCTIONS: {instructions}");

            thread::sleep(Duration::from_secs_f64(1.0 / frequency as f64));
        };
    };

    println!("\nCPU EXCEPTED - {}", cpu_exception.unwrap_err());
}
