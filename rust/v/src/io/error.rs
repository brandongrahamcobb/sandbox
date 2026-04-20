// use thiserror::Error;
//
// use crate::io::packet::{
//     error::{PacketBuildError, PacketReadWriteError},
//     handlers::error::HandlerPacketError,
// };
//
// #[derive(Debug, Error)]
// pub enum IoError {
//     #[error("I/O layer error")]
//     IoGenericError(#[from] std::io::Error),
//
//     #[error("Build packet error")]
//     IoPacketBuildError(#[from] PacketBuildError),
//
//     #[error("Packet handler error in I/O layer")]
//     IoHandlerPacketError(#[from] HandlerPacketError),
//
//     #[error("Read/write packet error in I/O layer")]
//     IoPacketReadWriteError(#[from] PacketReadWriteError),
// }
