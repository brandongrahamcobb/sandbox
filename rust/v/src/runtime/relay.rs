use crate::net::error::NetworkError;
use crate::net::error::NetworkError::{NetworkHandshakeError, UnsupportedOpcodeError};
use crate::net::handshake;
use crate::net::packet::error::GenericPacketError;
use crate::net::packet::handler::error::PacketHandlerError;
use crate::net::packet::handler::login::{credentials, tos};
use crate::net::packet::handler::service::PacketHandlerResult;
use crate::net::packet::{core::Packet, read::PacketReader, write::PacketWriter};
use crate::op::recv::RecvOpcode;
use crate::runtime::error::{
    RuntimeError, RuntimeError::RuntimeNetworkError, RuntimeRelayCreationError,
    RuntimeServerConnectionError,
};
use rand::{RngExt, rng};
use std::net::SocketAddr;
use tokio::net::TcpStream;
use tracing::{error, warn};

pub struct Runtime<T: RuntimeRelay> {
    reader: PacketReader,
    writer: PacketWriter,
    addr: SocketAddr,
    relay: T,
}

impl<T: RuntimeRelay + Default + Send> Runtime<T> {
    pub async fn new(stream: TcpStream, addr: SocketAddr) -> Result<Self, RuntimeError> {
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
        let writer = PacketWriter::new(write_half, &send_iv);
        Ok(Self {
            reader,
            writer,
            addr,
            relay: T::default(),
        })
    }

    pub async fn run(&mut self) -> Result<(), RuntimeError> {
        loop {
            let packet = self.reader.read_packet().await?;
            let result = self.relay.handle_packet(packet).await?;
            result.reply(packet);
        }
    }
}

trait RuntimeRelay {
    async fn handle_packet(&mut self, packet: Packet) -> Result<PacketHandlerResult, RuntimeError>;
}

// match handshake::build_handshake_packet(&recv_iv, &send_iv) {
//     Ok(handshake_packet) => {
//         writer.send_packet(&mut handshake_packet).await?;
//     }
//     Err(e) => Err(HandshakeError(e.to_string())),
// }

#[derive(Default)]
pub struct Credentials;

impl RuntimeRelay for Credentials {
    async fn handle_packet(&mut self, packet: Packet) -> Result<PacketHandlerResult, RuntimeError> {
        let opcode = packet.opcode();
        match opcode {
            x if x == RecvOpcode::RequestLogin as i16 => {
                let handler = credentials::CredentialsHandler::new();
                let result = handler.handle(&packet)?;
                Ok(result)
            }
            x if x == RecvOpcode::AcceptTOS as i16 => {
                let handler = tos::TOSHandler::new();
                let result = handler.handle(&packet)?;
                Ok(result)
            }
            _ => Err(RuntimeNetworkError(UnsupportedOpcodeError(opcode))),
        }
    }
}

#[derive(Default)]
pub struct World;
//
// // #[async_trait]
impl RuntimeRelay for World {
    async fn handle_packet(&mut self, packet: Packet) -> Result<PacketHandlerResult, RuntimeError> {
        let opcode = packet.opcode();
        match opcode {
            x if x == RecvOpcode::RequestLogin as i16 => {
                let handler = credentials::CredentialsHandler::new();
                let result = handler.handle(&packet)?;
                Ok(result)
            }
            // x if x == RecvOpcode::AcceptTOS => {
            //     let handler = tos::TOSHandler::new();
            //     let result = handler.handle(&packet)?;
            //     Ok(result)
            // }
            _ => Err(RuntimeNetworkError(UnsupportedOpcodeError(opcode))),
        }
    }
}

//         //     0x01 => match login::build_login_status_packet(0) {
//         //         Ok(mut response) => {
//         //             if let Err(e) = self.writer.send_packet(&mut response).await {
//         //                 match e {
//         //                     PacketError::Io(pe) => {
//         //                         if pe.kind() == std::io::ErrorKind::UnexpectedEof {
//         //                             return Err(RuntimeError::Handler(pe.to_string()));
//         //                         } else {
//         //                             error!(
//         //                                 "Expected successful world packet send. Found a PacketError. Error: {}",
//         //                                 pe.to_string()
//         //                             );
//         //                             return Err(RuntimeError::Handler(pe.to_string()));
//         //                         }
//         //                     }
//         //                     _ => {
//         //                         error!(
//         //                             "Expected successful world packet send. Found an unhandled error type. Error: {}",
//         //                             e.to_string()
//         //                         );
//         //                         return Err(RuntimeError::Handler(e.to_string()));
//         //                     }
//         //                 }
//         //             }
//         //             Ok(())
//         //         }
//         //         Err(e) => Err(RuntimeError::Handler(e.to_string())),
//         //     },
//         //     _ => {
//         //         warn!(
//         //             "Expected successful world opcode. Found an unhandled opcode. Debug: {}",
//         //             opcode
//         //         );
//         //         Ok(())
//         //     }
//         // }
//         Ok(())
//     }
// }
