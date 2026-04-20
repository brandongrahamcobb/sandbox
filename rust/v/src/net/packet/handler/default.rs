use crate::net::error::NetworkError;
use crate::net::packet::core::Packet;
use crate::net::packet::handler::auth::AuthenticationHandler;
use crate::net::packet::handler::service::PacketHandlerResult;
use crate::op::recv::RecvOpcode;
use crate::runtime::server::ServerType;

pub struct DefaultHandler;

pub trait PacketHandler: Send + Sync {
    fn handle(&self, packet: &mut Packet) -> Result<PacketHandlerResult, NetworkError>;
}

// impl PacketHandler for DefaultHandler {
//     fn handle(&self, packet: &mut Packet) -> Result<(), NetworkError> {
//         let op = packet.opcode();
//         Err(NetworkError::UnsupportedOpcodeError(op))
//     }
// }

pub fn get_handler(op: i16, server_type: &ServerType) -> Box<dyn PacketHandler> {
    match server_type {
        ServerType::AuthenticationServer => get_authentication_handler(op),
        ServerType::WorldServer => get_world_handler(op),
    }
}

fn get_authentication_handler(op: i16) -> Box<dyn PacketHandler> {
    match num_traits::FromPrimitive::from_i16(op) {
        Some(RecvOpcode::Authentication) => Box::new(AuthenticationHandler::new()),
        // Some(RecvOpcode::GuestLogin) => Box::new(login::GuestLoginHandler::new()),
        // Some(RecvOpcode::ServerListReRequest) => Box::new(login::WorldListHandler::new()),
        // Some(RecvOpcode::CharListRequest) => Box::new(login::CharListHandler::new()),
        // Some(RecvOpcode::ServerStatusRequest) => Box::new(login::ServerStatusHandler::new()),
        // Some(RecvOpcode::AcceptTOS) => Box::new(login::AcceptTOSHandler::new()),
        // Some(RecvOpcode::SetGender) => Box::new(login::SetGenderHandler::new()),
        // Some(RecvOpcode::AfterLogin) => Box::new(DefaultHandler),
        // Some(RecvOpcode::RegisterPin) => Box::new(DefaultHandler),
        // Some(RecvOpcode::ServerListRequest) => Box::new(login::WorldListHandler::new()),
        // Some(RecvOpcode::ViewAllChar) => Box::new(DefaultHandler),
        // Some(RecvOpcode::PickAllChar) => Box::new(DefaultHandler),
        // Some(RecvOpcode::CharSelect) => Box::new(login::CharacterSelectHandler::new()),
        // Some(RecvOpcode::CheckCharName) => Box::new(login::CheckCharNameHandler::new()),
        // Some(RecvOpcode::CreateChar) => Box::new(login::CreateCharacterHandler::new()),
        // Some(RecvOpcode::DeleteChar) => Box::new(login::DeleteCharHandler::new()),
        // Some(RecvOpcode::RegisterPic) => Box::new(DefaultHandler),
        // Some(RecvOpcode::CharSelectWithPic) => Box::new(DefaultHandler),
        // Some(RecvOpcode::ViewAllPicRegister) => Box::new(DefaultHandler),
        // Some(RecvOpcode::ViewAllWithPic) => Box::new(DefaultHandler),
        // Some(RecvOpcode::LoginStarted) => Box::new(login::LoginStartHandler::new()),
        None | Some(_) => Box::new(DefaultHandler),
    }
}

// fn get_world_handler(op: i16) -> Box<dyn PacketHandler> {
//     use crate::packet::handle::world;
//
//     match num::FromPrimitive::from_i16(op) {
//         // Some(RecvOpcode::PlayerMove) => Box::new(world::PlayerMoveHandler::new()),
//         Some(RecvOpcode::PlayerLoggedIn) => Box::new(world::PlayerLoggedInHandler::new()),
//         // Some(RecvOpcode::ChangeChannel) => Box::new(world::ChangeChannelHandler::new()),
//         // Some(RecvOpcode::PlayerMapTransfer) => Box::new(world::PlayerMapTransferHandler::new()),
//         // Some(RecvOpcode::ChangeMap) => Box::new(world::ChangeMapHandler::new()),
//         // Some(RecvOpcode::PartySearch) => Box::new(world::PartySearchHandler::new()),
//         // Some(RecvOpcode::ChangeKeybinds) => Box::new(world::ChangeKeybindsHandler::new()),
//         // Some(RecvOpcode::AllChat) => Box::new(world::AllChatHandler::new()),
//         // Some(RecvOpcode::Whisper) => Box::new(world::WhisperHandler::new()),
//         None | Some(_) => Box::new(DefaultHandler),
//     }
// }
