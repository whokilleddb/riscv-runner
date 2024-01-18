// Emulate a CPU
use std::fmt;
use std::error::Error;
use crate::bus::Bus;
use crate::instruction::*;
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
        let mut j = 0;
        let mut msg = String::new();

        // Print Register stuff
        for i in 0..32 {
            msg = format!("{} x{}:\t0x{:08X}",  msg, i, self.regs[i]);
            j = j + 1;
            if j%4 == 0 || self.regs[i] == u64::MAX { 
                msg = format!("{}\n", msg);
                if self.regs[i] == u64::MAX {
                    j = 0;
                }
            }
            else {
                msg = format!("{}\t", msg);
            }
        }

        msg = format!("{}\n pc:\t0x{:08X}", msg, self.pc);
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

    pub fn execute(&mut self, inst: u32) {
        let opcode = inst  & 0x7f;
        match opcode {
            0x13 => {
                let i_type: IType = IType::new(inst);
                println!("{}", i_type);

                // "The shift amount is encoded in the lower 6 bits of the I-immediate field for RV64I."
                let shamt = (i_type.imm & 0x3f) as u32;

                match i_type.funct3 {
                    // addi
                    0x0 => {
                        self.regs[i_type.rd] = self.regs[i_type.rs1].wrapping_add(i_type.imm);
                    },
                    _ => {
                        eprintln!("[!] Invalid funct3(0x{:02x}) for opcode(0x13)", i_type.funct3);
                    }
                }
               
            },
            // add, mul, div, sub, rem, xor, or, and, sll. srl, sra, slt, sltu
            0x33 => {
                // "SLL, SRL, and SRA perform logical left, logical right, and arithmetic right
                // shifts on the value in register rs1 by the shift amount held in register rs2.
                // In RV64I, only the low 6 bits of rs2 are considered for the shift amount."
                let r_type: RType = RType::new(inst);
                println!("{}", r_type);

                let shamt = ((self.regs[r_type.rs2] & 0x3f) as u64) as u32;

                match (r_type.funct3, r_type.funct7) {
                    // add
                    (0x0, 0x00) => {
                        self.regs[r_type.rd] = self.regs[r_type.rs1].wrapping_add(self.regs[r_type.rs2]);
                    },
                    // mul
                    (0x0, 0x01) => {
                        self.regs[r_type.rd] = self.regs[r_type.rs1].wrapping_mul(self.regs[r_type.rs2]);
                    },
                    // div
                    (0x4, 0x01) => {
                        // See: Chapter 6
                        // “M” Standard Extension for Integer Multiplication and Division, Version 2.0
                        // The RISC-V Instruction Set Manual Volume I: User-Level ISA
                        match self.regs[r_type.rs2] {
                            0 => {
                                self.regs[r_type.rd] = self.regs[r_type.rs1]
                            }
                            _ => {
                                self.regs[r_type.rd] = self.regs[r_type.rs1].wrapping_div(self.regs[r_type.rs2])
                            }
                        };
                    },
                    // rem 
                    (0x6, 0x1) => {
                        // See: Chapter 6
                        // “M” Standard Extension for Integer Multiplication and Division, Version 2.0
                        // The RISC-V Instruction Set Manual Volume I: User-Level ISA
                        match self.regs[r_type.rs2] {
                            0 => {
                                self.regs[r_type.rd] = self.regs[r_type.rs1]
                            }
                            _ => {
                                self.regs[r_type.rd] = self.regs[r_type.rs1].wrapping_rem(self.regs[r_type.rs2])
                            }
                        };
                    }

                    // sub
                    (0x0, 0x20) => {
                        self.regs[r_type.rd] = self.regs[r_type.rs1].wrapping_sub(self.regs[r_type.rs2]);
                    },
                    // sll
                    (0x1, 0x00) => {
                        self.regs[r_type.rd] = self.regs[r_type.rs1].wrapping_shl(shamt);
                    }
                    
                    _ => {
                        eprintln!("[!] Invalid funct3(0x{:02x}) - funct7(0x{:02x}) for opcode(0x33)", r_type.funct3, r_type.funct7);
                    }
                }
                
            }
            _ => {
                eprintln!("Invalid Intruction")
            }
        }

    }
}
