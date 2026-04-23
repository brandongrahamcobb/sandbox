use std::io::Result;
use std::io::Write;
use std::ops::{Deref, DerefMut};

pub const INVALID_OPCODE: i16 = 1;

pub const MAX_PACKET_LENGTH: i16 = i16::MAX;

#[derive(Debug)]
pub struct Packet {
    pub bytes: Vec<u8>,
}

impl Packet {
    pub fn new(buffer: &[u8]) -> Packet {
        if buffer.len() > MAX_PACKET_LENGTH as usize {
            panic!(
                "Packet with length {} exceeded max packet length {}",
                buffer.len(),
                MAX_PACKET_LENGTH
            );
        }
        let bytes = buffer.to_vec();
        Packet { bytes }
    }

    pub fn new_empty() -> Packet {
        let bytes = vec![];
        Packet { bytes }
    }

    pub fn opcode(&self) -> i16 {
        if self.bytes.len() > 1 {
            let opcode: i16 = ((self.bytes[0] as u16) | ((self.bytes[1] as u16) << 8)) as i16;
            if opcode >= 0 { opcode } else { INVALID_OPCODE }
        } else {
            INVALID_OPCODE
        }
    }

    pub fn len(&self) -> i16 {
        (self.bytes.len() - 2) as i16
    }
}

impl Write for Packet {
    fn write(&mut self, buf: &[u8]) -> Result<usize> {
        self.bytes.write(buf)
    }

    fn flush(&mut self) -> Result<()> {
        self.bytes.flush()
    }
}

impl Deref for Packet {
    type Target = [u8];

    fn deref(&self) -> &Self::Target {
        &self.bytes
    }
}

impl DerefMut for Packet {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.bytes
    }
}
