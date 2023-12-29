use std::fmt;

// R Type Instruction
#[derive(Debug)]
pub struct RType {
    pub rd: usize,
    pub funct3: u32,
    pub rs1: usize,
    pub rs2: usize,
    pub funct7: u32
} 

impl RType {
    pub fn new(inst: u32) -> Self {
        let rd = ((inst >> 7) & 0x1f) as usize;        // 7-11
        let funct3 = (inst >> 12) & 0x7;             // 12 - 14
        let rs1 = ((inst >> 15) & 0x1f) as usize;      // 15-19
        let rs2 = ((inst >> 20) & 0x1f) as usize;      // 20-24
        let funct7 = (inst >> 25) & 0x7f;            // 24-31
        
        Self {
            rd,
            funct3,
            rs1,
            rs2,
            funct7
        }
    }
}

impl fmt::Display for RType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, 
            "R | rd: {:02} | funct3: {:02} | rs1: {:02} | rs2: {:02} | funct7: {:02}", 
            self.rd, self.funct3, self.rs1, self.rs2, self.funct7)
    }
}


// I Type Instruction
#[derive(Debug)]
pub struct IType {
    pub rd: usize,
    pub funct3: u32,
    pub rs1: usize,
    pub imm: u64
}

impl IType {
    pub fn new(inst: u32) -> Self {
        let rd = ((inst >> 7) & 0x1f) as usize;        // 7 - 11
        let funct3 = (inst >> 12) & 0x7;             // 12 - 14
        let rs1 = ((inst >> 15) & 0x1f) as usize;      // 15 - 19
        let imm = ((inst as i32 as i64) >> 20) as u64; // 20 - 31

        Self {
        	rd,
        	funct3,
        	rs1,
        	imm
        }
    }
}

impl fmt::Display for IType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, 
            "I | rd: {:02} | funct3: {:02} | rs1: {:02} | imm[11:0]: {}", 
            self.rd, self.funct3, self.rs1, self.imm
        )
    }
}

// S Type Instruction
#[derive(Debug)]
pub struct SType {
    pub imm0: usize,
    pub funct3: u32,
    pub rs1: usize,
    pub rs2: usize,
    pub imm1: u32
}

impl SType {
    pub fn new(inst: u32) -> Self {
		let imm0 = ((inst >> 7) & 0x1f) as usize;       // 7 - 11
        let funct3 = (inst >> 12) & 0x7;                // 12 - 14
        let rs1 = ((inst >> 15) & 0x1f) as usize;       // 15 - 19
        let rs2 = ((inst >> 20) & 0x1f) as usize;       // 20 -24
        let imm1 = (inst >> 25) & 0x7f;                 // 24 -31

		Self {
			imm0,
			funct3,
			rs1,
			rs2,
			imm1 
		}
    }
}


impl fmt::Display for SType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, 
            "S | imm[4:0]: {:02} | funct3: {:02} | rs1: {:02} | rs2: {:02} | imm[11:0]: {}", 
            self.imm0, self.funct3, self.rs1, self.rs2, self.imm1
        )
    }
}

// U-Type Instruction 
#[derive(Debug)]
pub struct UType {
    pub rd: usize,
    pub imm: u64
}

impl UType {
    pub fn new(inst: u32) -> Self {
        let rd = ((inst >> 7) & 0x1f) as usize;        		// 7 - 11
        let imm = (inst & 0xfffff000) as i32 as i64 as u64;	// 12 - 31

        Self {
            rd,
            imm
        }
    }
}

impl fmt::Display for UType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, 
            "U | rd: {:02} | imm[31:12]: {}", 
            self.rd, self.imm
        )
    }
}
