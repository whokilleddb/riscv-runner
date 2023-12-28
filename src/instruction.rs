// R Type Instruction
#[derive(Debug)]
struct RType {
    rd: usize,
    funct3: usize,
    rs1: usize,
    rs2: usize,
    funct7: usize
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
        };
    }
}

// I Type Instruction
#[derive(Debug)]
struct IType {
    rd: usize,
    funct3: usize,
    rs1: usize,
    imm: i64
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
        };
    }
}

// S Type Instruction
struct SType {
    imm1: usize,
    funct3: usize,
    rs1: usize,
    rs2: usize,
    imm2: u32
}

impl SType {
    pub fn new(inst: u32) -> Self {
		let imm1 = ((inst >> 7) & 0x1f) as usize;       // 7 - 11
        let funct3 = (inst >> 12) & 0x7;              // 12 - 14
        let rs1 = ((inst >> 15) & 0x1f) as usize;       // 15 - 19
        let rs2 = ((inst >> 20) & 0x1f) as usize;       // 20 -24
        let imm2 = (inst >> 25) & 0x7f;               // 24 -31

		Self {
			imm1,
			funct3,
			rs1,
			rs2,
			imm2 
		};
    }
}

// U-Type Instruction 
struct UType {
    rd: usize,
    imm: i64
}

impl UType {
    pub fn new(inst: u32) -> Self {
        let rd = ((inst >> 7) & 0x1f) as usize;        		// 7 - 11
        let imm = (inst & 0xfffff000) as i32 as i64 as u64;	// 12 - 31

        Self {
            rd,
            imm
        };
    }
}

enum InstructionType {
    r(RType),
    i(IType),
    s(SType),
    u(UType)
}

// Instruction struct
#[derive(Debug)]
pub struct Instruction{ 
    pub instruction: u32,
    pub opcode: u32,
    pub itype: InstructionType
}


impl Instruction  {
    pub fn new(inst: u32) -> Self {
        let opcode = inst  & 0x7f;
        let itype = match opcode {
            0x13 => RType::new(inst),
            0x33 => IType::new(inst),
            _ => UType::new(inst)
        };

        Self {
            instruction: inst,
            opcode,
            itype
        };
    } 
}

