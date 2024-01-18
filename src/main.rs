#![allow(dead_code, unused_variables)]

use std::fs::File;
use std::io::Read;
use std::path::PathBuf;

mod bus;
mod cli;
mod cpu;
mod dram;
mod consts;
mod opcodes;
mod instruction;

use crate::cpu::Cpu;

fn main() {

    // Read command line arguments
    let bin_file: PathBuf = cli::get_cli_options().unwrap();
    
    println!("====================================>> RISCV Runner <<====================================\n");
    println!("[i] Reading Binary File:\t{}", bin_file.display());

    // Read file contents as Vec<u8>
    let mut code: Vec<u8> = Vec::new();
    let mut _file = File::open(bin_file).unwrap();
    _file.read_to_end(&mut code).unwrap();
    println!("[i] Size of paylad read:\t{}", code.len());

    // Load data into CPU
    let mut cpu = Cpu::new(code);

    
    println!("\n================================>> Running Instructions <<================================\n");
    loop {
        // fetch instruction and update pc 
        let inst: u32 = match cpu.fetch() {
            Ok(v) =>v,
            Err(_) => break,
        };

        print!("[>] Instruction => 0x{:08x} => ", inst);

        // execute instruction
        cpu.execute(inst);
    }

    println!("\n================================>> Final Register State <<================================\n");
    println!("{}", cpu);
}
