use crate::net::error::NetworkError::NetworkHandshakeError;
use crate::net::handshake;
use crate::net::packet::handler::{auth, default, tos};
use crate::net::packet::{core::Packet, read::PacketReader, write::PacketWriter};
use crate::runtime::error::RuntimeError;
use rand::{RngExt, rng};
use std::net::SocketAddr;
use tokio::net::TcpStream;
use tracing::{error, warn};

trait RuntimeRelay {
    async fn handle_packet(&mut self, packet: Packet) -> Result<(), RuntimeError> {}

    async fn new(stream: TcpStream, addr: SocketAddr) -> Result<Self, RuntimeError> {
        let (recv_iv, send_iv) = {
            let mut recv_iv = vec![0u8; 4];
            let mut send_iv = vec![0u8; 4];
            let mut rng = rng();
            rng.fill(&mut recv_iv[..]);
            rng.fill(&mut send_iv[..]);
            (recv_iv, send_iv)
        };
        let (read_half, write_half) = stream.into_split();
        let reader = PacketReader::new(read_half, &recv_iv);
        let mut writer = PacketWriter::new(write_half, &send_iv);
        match handshake::build_handshake_packet(&recv_iv, &send_iv) {
            Ok(handshake) => {
                writer.send_handshake(&handshake.bytes).await?;
                Ok(Self {
                    reader,
                    writer,
                    addr,
                })
            }
            Err(e) => Err(HandshakeError(e.to_string())),
        }
    }

    async fn run(mut self) {
        loop {
            match self.reader.read_packet().await {
                Ok(packet) => {
                    self.handle_packet(packet).await?;
                }
                Err(e) => Err(RuntimeError::GenericRuntimeError),
            }
        }
    }
}

pub struct Authentication {
    reader: PacketReader,
    writer: PacketWriter,
    addr: SocketAddr,
}

impl RuntimeRelay for Authentication {
    async fn handle_packet(&mut self, packet: Packet) -> Result<(), RuntimeError> {
        let opcode = packet.opcode();
        let handler = default::DefaultHandler::new();
        match opcode {
            0x01 => {
                handler = auth::AuthenticationHandler::new();
            }
            0x07 => {
                handler = tos::TOSHandler::new();
            }
            _ => {
                warn!(
                    "Expected successful authentication opcode. Found an unhandled opcode. Debug: {}",
                    opcode
                );
            }
        }
        handler.handle(&packet).await?
    }
}

pub struct World {
    reader: PacketReader,
    writer: PacketWriter,
    addr: SocketAddr,
}

impl RuntimeRelay for World {
    async fn handle_packet(&mut self, packet: Packet) -> Result<(), RuntimeError> {
        let opcode = packet.opcode();
        // match opcode {
        //     0x01 => match login::build_login_status_packet(0) {
        //         Ok(mut response) => {
        //             if let Err(e) = self.writer.send_packet(&mut response).await {
        //                 match e {
        //                     PacketError::Io(pe) => {
        //                         if pe.kind() == std::io::ErrorKind::UnexpectedEof {
        //                             return Err(RuntimeError::Handler(pe.to_string()));
        //                         } else {
        //                             error!(
        //                                 "Expected successful world packet send. Found a PacketError. Error: {}",
        //                                 pe.to_string()
        //                             );
        //                             return Err(RuntimeError::Handler(pe.to_string()));
        //                         }
        //                     }
        //                     _ => {
        //                         error!(
        //                             "Expected successful world packet send. Found an unhandled error type. Error: {}",
        //                             e.to_string()
        //                         );
        //                         return Err(RuntimeError::Handler(e.to_string()));
        //                     }
        //                 }
        //             }
        //             Ok(())
        //         }
        //         Err(e) => Err(RuntimeError::Handler(e.to_string())),
        //     },
        //     _ => {
        //         warn!(
        //             "Expected successful world opcode. Found an unhandled opcode. Debug: {}",
        //             opcode
        //         );
        //         Ok(())
        //     }
        // }
        Ok(())
    }
}
