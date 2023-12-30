// Emulate a system bus to carry data between CPU and peripheral devices
use std::error::Error;
use crate::dram::Dram;
use crate::consts::DRAM_BASE;

#[derive(Debug)]
pub struct Bus {
    pub dram: Dram,
}

impl Bus {
    pub fn new(code: Vec<u8>) -> Self {
        Self {
            dram: Dram::new(code)
        }
    }

    pub fn load(&self, addr: u64, size: u64) -> Result<u64, Box<dyn Error>> {
        if DRAM_BASE <= addr {
            self.dram.load(addr, size)
        }
        else {
            let err_msg = format!("Address({:#?}) out of Range({:#?})", addr, DRAM_BASE);
            Err(err_msg.into())
        }
    }
}


