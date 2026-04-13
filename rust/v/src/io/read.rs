use crate::io::error::PacketError;
use crate::io::packet::{MAX_PACKET_LENGTH, Packet};
use crate::sec::aes::AES;
use crate::sec::custom;
use byteorder::{LittleEndian, ReadBytesExt};
use std::io::Read;
use tokio::io::{AsyncReadExt, BufReader};
use tokio::net::tcp::OwnedReadHalf;

pub struct PacketReader {
    reader: BufReader<OwnedReadHalf>,
    aes: AES,
}

impl PacketReader {
    pub fn new(read_half: OwnedReadHalf, recv_iv: &[u8]) -> Self {
        Self {
            reader: BufReader::new(read_half),
            aes: AES::new(&recv_iv.to_vec(), 83), // Version 83
        }
    }

    pub async fn read_packet(&mut self) -> Result<Packet, PacketError> {
        let length = self.read_header().await?;
        self.read_data(length).await
    }

    async fn read_header(&mut self) -> Result<i16, PacketError> {
        let mut header_buf = [0u8; 4];
        match self.reader.read_exact(&mut header_buf).await {
            Ok(_) => {}
            Err(e) if e.kind() == std::io::ErrorKind::UnexpectedEof => {
                return Err(PacketError::Io(e));
                // Redundant, consider a different error.
            }
            Err(e) => return Err(PacketError::Io(e)),
        }
        if !self.aes.check_header(&header_buf) {
            return Err(PacketError::InvalidHeader);
        }
        let length = self.aes.get_packet_length(&header_buf);
        if length < 2 || length > MAX_PACKET_LENGTH {
            return Err(PacketError::InvalidPacketLength(length));
        }
        Ok(length)
    }

    async fn read_data(&mut self, length: i16) -> Result<Packet, PacketError> {
        let mut buf = vec![0u8; length as usize];
        match self.reader.read_exact(&mut buf).await {
            Ok(_) => {}
            Err(e) if e.kind() == std::io::ErrorKind::UnexpectedEof => {
                return Err(PacketError::Io(e));
                // Redundant, consider a different error.
            }
            Err(e) => return Err(PacketError::Io(e)),
        }
        self.aes.crypt(&mut buf);
        custom::decrypt(&mut buf);
        Ok(Packet::new(&buf))
    }
}

pub trait PktRead: ReadBytesExt {
    fn read_byte(&mut self) -> std::io::Result<u8> {
        self.read_u8()
    }

    fn read_bytes(&mut self, length: usize) -> std::io::Result<Vec<u8>> {
        let mut buf = vec![0u8; length];
        match self.read_exact(&mut buf) {
            Ok(_) => Ok(buf),
            Err(e) => Err(e),
        }
    }

    fn read_short(&mut self) -> std::io::Result<i16> {
        self.read_i16::<LittleEndian>()
    }

    fn read_int(&mut self) -> std::io::Result<i32> {
        self.read_i32::<LittleEndian>()
    }

    fn read_long(&mut self) -> std::io::Result<i64> {
        self.read_i64::<LittleEndian>()
    }

    fn read_str(&mut self, length: usize) -> std::io::Result<String> {
        let mut buf = vec![0u8; length];
        match self.read_exact(&mut buf) {
            Ok(_) => match String::from_utf8(buf) {
                Ok(string) => Ok(string),
                Err(e) => Err(std::io::Error::new(
                    std::io::ErrorKind::InvalidData,
                    e.to_string(),
                )),
            },
            Err(e) => Err(e),
        }
    }

    fn read_str_with_length(&mut self) -> std::io::Result<String> {
        match self.read_short() {
            Ok(length) => self.read_str(length as usize),
            Err(e) => Err(e),
        }
    }
}

impl<R: Read> PktRead for R {}
