use crate::bus::*;
use crate::trap::*;

pub const CLINT_MTIMECMP: u64 = CLINT_BASE + 0x4000;
pub const CLINT_MTIME: u64 = CLINT_BASE + 0xbff8;

pub struct Clint {
    pub mtimecmp: u64,
    pub mtime: u64,
}

impl Device for Clint {
    fn load(&mut self, addr: u64, size: u64) -> Result<u64, Exception> {
        match size {
            64 => Ok(self.load64(addr)),
            _ => Err(Exception::LoadAccessFault),
        }
    }

    fn store(&mut self, addr: u64, size: u64, value: u64) -> Result<(), Exception> {
        match size {
            64 => Ok(self.store64(addr, value)),
            _ => Err(Exception::StoreAMOAccessFault),
        }
    }
}

impl Clint {
    pub fn new() -> Self {
        Self {
            mtimecmp: 0,
            mtime: 0,
        }
    }

    fn load64(&self, addr: u64) -> u64 {
        match addr {
            CLINT_MTIMECMP => self.mtimecmp,
            CLINT_MTIME => self.mtime,
            _ => 0,
        }
    }

    fn store64(&mut self, addr: u64, value: u64) {
        match addr {
            CLINT_MTIMECMP => self.mtimecmp = value,
            CLINT_MTIME => self.mtime = value,
            _ => {}
        }
    }
}