use crate::config::settings;
use crate::net::error::NetworkError;
use crate::net::packet::core::Packet;
use crate::net::packet::error::PacketError;
use crate::net::packet::io::error::IOError::WriteError;
use crate::runtime::state::SharedState;
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
    pub fn new(
        write_half: OwnedWriteHalf,
        send_iv: &[u8],
        state: &SharedState,
    ) -> Result<Self, NetworkError> {
        Ok(Self {
            writer: BufWriter::new(write_half),
            aes: AES::new(&send_iv.to_vec(), settings::get_version(&state.settings)?),
        })
    }

    pub async fn send_packet(&mut self, packet: &mut Packet) -> Result<(), NetworkError> {
        let header = self.aes.gen_packet_header(packet.len() + 2);
        self.aes.crypt(&mut packet.bytes);
        custom::encrypt(&mut packet.bytes);
        self.writer
            .write_all(&header)
            .await
            .map_err(WriteError)
            .map_err(PacketError::from)
            .map_err(NetworkError::from);
        self.writer
            .write_all(&packet.bytes)
            .await
            .map_err(WriteError)
            .map_err(PacketError::from)
            .map_err(NetworkError::from);
        self.writer
            .flush()
            .await
            .map_err(WriteError)
            .map_err(PacketError::from)
            .map_err(NetworkError::from);

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
