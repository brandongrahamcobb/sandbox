use crate::config::settings;
use crate::constants::HEADER_SIZE;
use crate::net::error::NetworkError;
use crate::net::packet::core::Packet;
use crate::net::packet::error::PacketError;
use crate::net::packet::io::error::IOError::ReadError;
use crate::net::packet::validation;
use crate::runtime::state::SharedState;
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
    pub fn new(
        read_half: OwnedReadHalf,
        recv_iv: &[u8],
        shared_state: &SharedState,
    ) -> Result<Self, NetworkError> {
        Ok(Self {
            reader: BufReader::new(read_half),
            aes: AES::new(
                &recv_iv.to_vec(),
                settings::get_version(&shared_state.settings)?,
            ),
        })
    }

    // 1st Level
    pub fn get_packet_length(&self, header: &[u8]) -> u16 {
        (header[0] as u16 + ((header[1] as u16) << 8))
            ^ (header[2] as u16 + ((header[3] as u16) << 8))
    }

    // 2nd Level
    async fn read_buffer(&mut self, buf: &mut [u8]) -> Result<(), NetworkError> {
        self.reader
            .read_exact(buf)
            .await
            .map_err(ReadError)
            .map_err(PacketError::from)
            .map_err(NetworkError::from)?;
        Ok(())
    }

    // 3rd Level
    async fn read_header(&mut self) -> Result<[u8; HEADER_SIZE as usize], NetworkError> {
        let mut buf = [0u8; HEADER_SIZE as usize];
        self.read_buffer(&mut buf).await?;
        validation::core::check_header(&self.aes, &buf)?;
        Ok(buf)
    }

    async fn read_payload(&mut self, header: &[u8]) -> Result<Packet, NetworkError> {
        let length = self.get_packet_length(header);
        validation::core::check_packet_length(length)?;
        let mut buf = vec![0u8; length as usize];
        self.read_buffer(&mut buf).await?;
        self.aes.crypt(&mut buf);
        custom::decrypt(&mut buf);
        Ok(Packet::new(&buf))
    }

    // 4th Level
    pub async fn read_packet(&mut self) -> Result<Packet, NetworkError> {
        let header: [u8; HEADER_SIZE as usize] = self.read_header().await?;
        let packet = self.read_payload(&header).await?;
        Ok(packet)
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
