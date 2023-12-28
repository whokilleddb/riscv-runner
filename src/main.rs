use std::fs::File;
use std::io::Read;
use std::path::PathBuf;

mod bus;
mod cli;
mod cpu;
mod dram;
mod consts;
mod instruction;

use crate::cpu::Cpu;
use crate::instruction::Instruction;

fn main() {

    // Read command line arguments
    let bin_file: PathBuf = cli::get_cli_options().unwrap();
    
    println!("================================== Rusty RISC-V Emulator ==================================\n");
    println!("[i] Reading Binary File:\t{}", bin_file.display());

    // Read file contents as Vec<u8>
    let mut code: Vec<u8> = Vec::new();
    let mut _file = File::open(bin_file).unwrap();
    _file.read_to_end(&mut code).unwrap();
    println!("[i] Size of paylad read:\t{}\n", code.len());

    // Load data into CPU
    let mut cpu = Cpu::new(code);

    //println!("{}", cpu);
    
    loop {
        // fetch instruction and update pc 
        let inst: Instruction = match cpu.fetch() {
            Ok(v) =>Instruction::new(v),
            Err(_) => break,
        };

        println!("{:?}", inst);
    }

    
}
