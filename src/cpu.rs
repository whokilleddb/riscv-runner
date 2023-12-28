// Emulate a CPU
use std::fmt;
use std::error::Error;
use crate::bus::Bus;
use crate::consts::*;

// This structure represents a CPU unit 
#[derive(Debug)]    
pub struct Cpu{
    pub regs: [u64; 32],        // 32 registers of RV64
    pub pc: u64,                // Program counter
    pub bus: Bus,               // System bus
}

impl fmt::Display for Cpu {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut msg = String::new();

        // Print Register stuff
        for i in 0..32 {
            msg = format!("{} x{}:\t0x{:08X}",  msg, i, self.regs[i]);
            if i%4 == 3 {
                msg = format!("{}\n", msg);
            }
            else {
                msg = format!("{}\t", msg);
            }
        }

        msg = format!("{} pc:\t0x{:08X}", msg, self.pc);
        write!(f, "{}", msg)
    }
}

impl Cpu {
    pub fn new(code: Vec<u8>)  -> Self {
        // set registers to zero
        let mut regs = [0; 32];

        // set up stack pointer
        regs[2] = DRAM_BASE + DRAM_SIZE;

        Self {
            regs,
            pc: DRAM_BASE,
            bus: Bus::new(code)
        }
    }

    pub fn fetch(&mut self)->Result<u32, Box<dyn Error>> {
        let curr_pc = self.pc;
        self.pc = self.pc + 4;
        match self.bus.load(curr_pc, 32) {
            Ok(v) => {
                if v == 0 {
                    return Err("Invalid Instruction (0x0000)".into());
                }
                Ok(v as u32)
            },
            Err(e) => Err(e)
        }    
    }

    pub fn execute(&mut self, instruction: u32) {

    }
}
