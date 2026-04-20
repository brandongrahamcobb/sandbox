use crate::net::error::NetworkError;
use crate::net::packet::core::Packet;
use crate::net::packet::error::PacketReadWriteError::PacketWriteError;
use crate::sec::aes::AES;
use crate::sec::custom;
use byteorder::{LittleEndian, WriteBytesExt};
use std::io::Write;
use tokio::io::{AsyncWriteExt, BufWriter};
use tokio::net::tcp::OwnedWriteHalf;

pub struct PacketWriter {
    writer: BufWriter<OwnedWriteHalf>,
    aes: AES,
}

impl PacketWriter {
    pub fn new(write_half: OwnedWriteHalf, send_iv: &[u8]) -> Self {
        Self {
            writer: BufWriter::new(write_half),
            aes: AES::new(&send_iv.to_vec(), 83),
        }
    }

    pub async fn send_handshake(&mut self, packet: &[u8]) -> Result<(), NetworkError> {
        self.writer.write_all(packet).await?;
        self.writer.flush().await?;
        Ok(())
    }

    pub async fn send_packet(&mut self, packet: &mut Packet) -> Result<(), NetworkError> {
        let header = self.aes.gen_packet_header(packet.len() + 2);
        custom::encrypt(&mut packet.bytes);
        self.aes.crypt(&mut packet.bytes);
        self.writer.write_all(&header).await?;
        self.writer.write_all(&packet.bytes).await?;
        self.writer.flush().await?;
        Ok(())
    }
}

pub trait PktWrite: WriteBytesExt {
    fn write_byte(&mut self, byte: u8) -> std::io::Result<usize> {
        self.write(&[byte])
    }

    fn write_bytes(&mut self, bytes: &[u8]) -> std::io::Result<usize> {
        self.write(bytes)
    }

    fn write_short(&mut self, short: i16) -> std::io::Result<()> {
        self.write_u16::<LittleEndian>(short as u16)
    }

    fn write_int(&mut self, int: i32) -> std::io::Result<()> {
        self.write_u32::<LittleEndian>(int as u32)
    }

    fn write_long(&mut self, long: i64) -> std::io::Result<()> {
        self.write_u64::<LittleEndian>(long as u64)
    }

    fn write_str(&mut self, string: &str) -> std::io::Result<usize> {
        self.write(string.as_bytes())
    }

    fn write_str_with_length(&mut self, string: &str) -> std::io::Result<usize> {
        match self.write_short(string.len() as i16) {
            Ok(_) => self.write_str(string),
            Err(e) => Err(e),
        }
    }
}

impl<W: Write> PktWrite for W {}
