use std::error::Error;
use crate::consts::*;


// Structure to represent a DRAM
#[derive(Debug)]
pub struct Dram {
    pub dram: Vec<u8>,
}

impl Dram {
    pub fn new(code: Vec<u8>) -> Dram {
        let mut dram = vec![0; DRAM_SIZE as usize];
        dram.splice(..code.len(), code.iter().cloned());
        Self {
            dram
        }
    }

     /// Load bytes from the little-endiam dram.
     pub fn load(&self, addr: u64, size: u64) -> Result<u64, Box<dyn Error>> {
        let index = (addr - DRAM_BASE) as usize;
        match size {
            8 => Ok(self.load8(index)),
            16 => Ok(self.load16(index)),
            32 => Ok(self.load32(index)),
            64 => Ok(self.load64(index)),
            _ => {
                let err_msg = format!("Invalid size:\t\t{}", size);
                Err(err_msg.into())
            },
        }
    }

    /// Load a byte from the little-endian dram.
    fn load8(&self, index: usize) -> u64 {
        
        self.dram[index] as u64
    }

    /// Load 2 bytes from the little-endian dram.
    fn load16(&self, index: usize) -> u64 {
        
        return (self.dram[index] as u64) | ((self.dram[index + 1] as u64) << 8);
    }

    /// Load 4 bytes from the little-endian dram.
    fn load32(&self, index: usize) -> u64 {
        return (self.dram[index] as u64)
            | ((self.dram[index + 1] as u64) << 8)
            | ((self.dram[index + 2] as u64) << 16)
            | ((self.dram[index + 3] as u64) << 24);
    }

    /// Load 8 bytes from the little-endian dram.
    fn load64(&self, index: usize) -> u64 {
        
        return (self.dram[index] as u64)
            | ((self.dram[index + 1] as u64) << 8)
            | ((self.dram[index + 2] as u64) << 16)
            | ((self.dram[index + 3] as u64) << 24)
            | ((self.dram[index + 4] as u64) << 32)
            | ((self.dram[index + 5] as u64) << 40)
            | ((self.dram[index + 6] as u64) << 48)
            | ((self.dram[index + 7] as u64) << 56);
    }
}
