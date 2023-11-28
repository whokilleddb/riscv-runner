use std::fs::File;
use std::path::PathBuf;
use std::io::Read;

mod cpu;
mod cli;
mod consts;

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

    let mut cpu: cpu::Cpu = cpu::Cpu::new(code);
    
    if cfg!(debug_assertions) {
        println!("==================================== Initial CPU State ====================================");
        println!("{}", cpu);
        println!("===========================================================================================");
    }

    while cpu.pc < cpu.dram.len() as u64 {
        // Fetch 32 bit instruction
        let inst = cpu.fetch();
        if cfg!(debug_assertions) {
            print!("[i] Instruction: 0x{:08x}", inst);
        }

        // Move forward 4 bytes to next instruction
        cpu.pc = cpu.pc + 4;

        // Execute instruction
        cpu.execute(inst);
    }

    println!("===================================== Final CPU State =====================================");
        println!("{}", cpu);
        println!("===========================================================================================");
}
