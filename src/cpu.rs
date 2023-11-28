use std::fmt;
use crate::consts::MEMORY_SIZE;

#[derive(Debug)]
pub struct Cpu {
    pub regs: [u64; 32],                // 32 general-purpose registers
    pub pc:   u64,                      // Program counter
    pub dram: Vec<u8>,                  // Memory regions for dynamic RAM
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
    pub fn new(code: Vec<u8>) -> Cpu {
       let mut regs: [u64; 32] = [0; 32];

        // Set x0
        regs[0] = 0;

        // Set x2 to size of Memory
        regs[2] = MEMORY_SIZE;

        // The base RISC-V ISA has fixed-length 32-bit instructions,
        // so that must be naturally aligned on 4-byte boundary.
        let mut p_code = code.clone();
        let len = code.len();
        let remainder = len % 4;
        if remainder != 0 {
            let padding = 4 - remainder;
            p_code.resize(len + padding, 0);
        }

        Cpu {
            regs: regs,
            pc: 0,
            dram: p_code
        }
    }

    pub fn fetch(&self) -> u32 {

        let index = self.pc as usize;
        return (self.dram[index] as u32)
            | ((self.dram[index + 1] as u32) << 8)
            | ((self.dram[index + 2] as u32) << 16)
            | ((self.dram[index + 3] as u32) << 24);
    }

    pub fn execute(&mut self, inst: u32) {
        // From - Volume I: RISC-V User-Level ISA V2.2
        // 2.2 Base Instruction Formats
        let opcode = inst & 0x7f;                   // 0-6
        let rd = ((inst >> 7) & 0x1f) as usize;     // 7-11
        let rs1 = ((inst >> 15) & 0x1f) as usize;   // 15-19
        let rs2 = ((inst >> 20) & 0x1f) as usize;   // 20-24

        if cfg!(debug_assertions) {
            print!("<opcode: {:08x} | rd: {:08x} | ", opcode, rd);
            println!("rs1: {:08x} | rs2: {:08x}>", rs1, rs2);
        }

        // https://www.cs.sfu.ca/~ashriram/Courses/CS295/assets/notebooks/RISCV/RISCV_CARD.pdf
        match opcode {
            0x13 => {
                // addi - R type
                let imm = ((inst & 0xfff00000) as i32 as i64 >> 20) as u64;
                self.regs[rd] = self.regs[rs1].wrapping_add(imm);
            }
            0x33 => {
                // add - I-Type
                self.regs[rd] = self.regs[rs1].wrapping_add(self.regs[rs2]);
            }
            _ => {
                // Not implemented
                if cfg!(debug_assertions) {
                    println!("Instruction not implemented");
                }
            }
        }
    }
}
