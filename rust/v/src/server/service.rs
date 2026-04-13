use crate::io::{error::PacketError, packet::Packet, read::PacketReader, write::PacketWriter};
use crate::net::login;
use crate::server::error::RuntimeError;
use rand::{RngExt, rng};
use std::net::SocketAddr;
use tokio::net::TcpStream;
use tracing::{error, warn};

pub struct LoginServerActor;

impl LoginServerActor {
    pub async fn run(addr: &str) -> Result<(), RuntimeError> {
        let listener = tokio::net::TcpListener::bind(addr).await?;
        loop {
            match listener.accept().await {
                Ok((stream, peer_addr)) => {
                    tokio::spawn(async move {
                        match LoginClientActor::new(stream, peer_addr).await {
                            Ok(actor) => actor.run().await,
                            Err(e) => error!(error = %e, "Failed to create LoginClientActor"),
                        }
                    });
                }
                Err(e) => {
                    error!(error = %e, "Error accepting login connection");
                }
            }
        }
    }
}

pub struct LoginClientActor {
    reader: PacketReader,
    writer: PacketWriter,
    peer_addr: SocketAddr,
}

impl LoginClientActor {
    pub async fn new(stream: TcpStream, peer_addr: SocketAddr) -> Result<Self, RuntimeError> {
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
        match login::build_handshake_packet(&recv_iv, &send_iv) {
            Ok(handshake) => {
                if let Err(e) = writer.send_handshake(&handshake.bytes).await {
                    match e {
                        PacketError::Io(pe) => {
                            if pe.kind() == std::io::ErrorKind::UnexpectedEof {
                                return Err(RuntimeError::Handler(pe.to_string()));
                            } else {
                                error!(
                                    "Expected successful login handshake send. Found a PacketError. Error: {}",
                                    pe.to_string()
                                );
                                return Err(RuntimeError::Handler(pe.to_string()));
                            }
                        }
                        _ => {
                            error!(
                                "Expected successful login handshake send. Found an unhandled error type. Error: {}",
                                e.to_string()
                            );
                            return Err(RuntimeError::Handler(e.to_string()));
                        }
                    }
                }
                Ok(Self {
                    reader,
                    writer,
                    peer_addr,
                })
            }
            Err(e) => return Err(RuntimeError::Handler(e.to_string())),
        }
    }

    async fn handle_packet(&mut self, packet: Packet) -> Result<(), RuntimeError> {
        let opcode = packet.opcode();
        match opcode {
            0x01 => match login::build_login_status_packet(0) {
                Ok(mut response) => {
                    if let Err(e) = self.writer.send_packet(&mut response).await {
                        match e {
                            PacketError::Io(pe) => {
                                if pe.kind() == std::io::ErrorKind::UnexpectedEof {
                                    return Err(RuntimeError::Handler(pe.to_string()));
                                } else {
                                    error!(
                                        "Expected successful login packet send. Found a PacketError. Error: {}",
                                        pe.to_string()
                                    );
                                    return Err(RuntimeError::Handler(pe.to_string()));
                                }
                            }
                            _ => {
                                error!(
                                    "Expected successful login packet send. Found an unhandled error type. Error: {}",
                                    e.to_string()
                                );
                                return Err(RuntimeError::Handler(e.to_string()));
                            }
                        }
                    }
                    Ok(())
                }
                Err(e) => Err(RuntimeError::Handler(e.to_string())),
            },
            _ => {
                warn!(
                    "Expected successful login opcode. Found an unhandled opcode. Debug: {}",
                    opcode
                );
                Ok(())
            }
        }
    }

    pub async fn run(mut self) {
        loop {
            match self.reader.read_packet().await {
                Ok(packet) => {
                    if let Err(e) = self.handle_packet(packet).await {
                        match e {
                            RuntimeError::Handler(msg) => {
                                error!(
                                    "Expected successful login packet handling. Found a RuntimeError. Error: {}",
                                    msg
                                );
                                break;
                            }
                            _ => {
                                error!(
                                    "Expected successful login packet handling. Found an unhandled error type: Error: {}",
                                    e.to_string()
                                );
                                break;
                            }
                        }
                    }
                }
                Err(e) => match e {
                    PacketError::Io(pe) => {
                        if pe.kind() == std::io::ErrorKind::UnexpectedEof {
                            break;
                        } else {
                            error!(
                                "Expected successful login packet read. Found a PacketError. Error: {}",
                                pe.to_string()
                            );
                            break;
                        }
                    }
                    _ => {
                        error!(
                            "Expected successful login packet read. Found an unhandled error type. Error: {}",
                            e.to_string()
                        );
                        break;
                    }
                },
            }
        }
    }
}
