use crate::net::error::NetworkError;
use crate::net::packet::core::MAX_PACKET_LENGTH;
use crate::net::packet::error::PacketError;
use crate::net::packet::validation::error::ValidationError::{InvalidHeader, InvalidPacketLength};
use crate::sec::aes::AES;

pub fn check_header(aes: &AES, header: &[u8]) -> Result<(), NetworkError> {
    if !((header[0] ^ aes.iv[2]) & 0xFF) == ((aes.version >> 8) as u8 & 0xFF)
        && ((header[1] ^ aes.iv[3]) & 0xFF) == (aes.version & 0xFF) as u8
    {
        return Err(NetworkError::from(PacketError::from(InvalidHeader)));
    }
    Ok(())
}

pub fn check_packet_length(length: i16) -> Result<(), NetworkError> {
    if length < 2 || length > MAX_PACKET_LENGTH {
        return Err(NetworkError::from(PacketError::from(InvalidPacketLength(
            length,
        ))));
    }
    Ok(())
}
