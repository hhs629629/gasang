use crate::mmu::{MmuData, Page, HostMemory};
use crate::MMUError;

use std::io::{Read, Write};
use std::sync::Arc;

pub struct MemoryFrame {
    mmu: Arc<MmuData>,
    addr: usize,
}

impl MemoryFrame {
    pub fn new(mmu: Arc<MmuData>, addr: usize) -> Self {
        MemoryFrame {
            mmu: mmu.clone(),
            addr,
        }
    }

    pub fn read(&mut self, buf: &mut [u8]) -> Result<usize, MMUError> {
        let page = self.mmu.query(self.addr)?;
        todo!()
    }

    pub fn write(&mut self, buf: &[u8]) -> Result<(), MMUError> {
        todo!()
    }

    pub fn read_u8(&mut self) -> Result<u8, MMUError> {
        let mut buf = [0u8; 1];
        self.read(&mut buf)?;
        Ok(buf[0])
    }

    pub fn read_u16(&mut self) -> Result<u16, MMUError> {
        let mut buf = [0u8; 2];
        self.read(&mut buf)?;
        Ok(u16::from_le_bytes(buf))
    }

    pub fn read_u32(&mut self) -> Result<u32, MMUError> {
        let mut buf = [0u8; 4];
        self.read(&mut buf)?;
        Ok(u32::from_le_bytes(buf))
    }

    pub fn read_u64(&mut self) -> Result<u64, MMUError> {
        let mut buf = [0u8; 8];
        self.read(&mut buf)?;
        Ok(u64::from_le_bytes(buf))
    }

    pub fn write_u8(&mut self, val: u8) -> Result<(), MMUError> {
        self.write(&[val])
    }

    pub fn write_u16(&mut self, val: u16) -> Result<(), MMUError> {
        self.write(&val.to_le_bytes())
    }

    pub fn write_u32(&mut self, val: u32) -> Result<(), MMUError> {
        self.write(&val.to_le_bytes())
    }

    pub fn write_u64(&mut self, val: u64) -> Result<(), MMUError> {
        self.write(&val.to_le_bytes())
    }
}