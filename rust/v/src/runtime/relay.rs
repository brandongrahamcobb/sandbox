use crate::net::channel;
use crate::net::error::NetworkError::UnsupportedOpcodeError;
use crate::net::packet::build;
use crate::net::packet::core::Packet;
use crate::net::packet::handler::login::action::{LoginAction, RejectReason};
use crate::net::packet::handler::login::{credentials, tos};
use crate::net::packet::handler::result::HandlerResult;
use crate::net::packet::handler::world::action::WorldAction;
use crate::net::packet::handler::world::cc;
use crate::net::packet::io::{read::PacketReader, write::PacketWriter};
use crate::op::recv::RecvOpcode;
use crate::runtime::error::RuntimeError;
use crate::runtime::session::{Session, SessionState};
use crate::runtime::state::SharedState;
use rand::{RngExt, rng};
use tokio::net::TcpStream;

pub struct RuntimeContext {
    pub session_id: u32,
    pub shared_state: SharedState,
}

pub struct Runtime<T: RuntimeRelay> {
    reader: PacketReader,
    writer: PacketWriter,
    shared_state: SharedState,
    relay: T,
    session_id: u32,
}

impl<T: RuntimeRelay + Default + Send> Runtime<T> {
    pub async fn new(shared_state: SharedState, stream: TcpStream) -> Result<Self, RuntimeError> {
        let (recv_iv, send_iv) = {
            let mut recv_iv = vec![0u8; 4];
            let mut send_iv = vec![0u8; 4];
            let mut rng = rng();
            rng.fill(&mut recv_iv[..]);
            rng.fill(&mut send_iv[..]);
            (recv_iv, send_iv)
        };
        let (read_half, write_half) = stream.into_split();
        let reader = PacketReader::new(read_half, &recv_iv, &shared_state)?;
        let mut writer = PacketWriter::new(write_half, &send_iv, &shared_state)?;
        let mut handshake = build::core::build_handshake_packet(&recv_iv, &send_iv, &shared_state)?;
        writer.send_unencrypted_packet(&mut handshake).await?;
        let session_id = shared_state.sessions.insert(Session {
            id: 0,
            account_id: None,
            hwid: None,
            session_state: SessionState::BeforeLogin,
            selected_world_id: None,
            selected_channel_id: None,
        });
        Ok(Self {
            reader,
            writer,
            relay: T::default(),
            shared_state,
            session_id,
        })
    }

    pub async fn run(self: &mut Self) -> Result<(), RuntimeError> {
        loop {
            let packet = self.reader.read_packet().await?;
            let ctx = RuntimeContext {
                session_id: self.session_id,
                shared_state: self.shared_state.clone(),
            };
            let result = self.relay.handle_packet(&ctx, &packet).await?;
            self.relay.execute(&ctx, result, &mut self.writer).await?
        }
    }
}

#[allow(async_fn_in_trait)]
pub trait RuntimeRelay {
    type HandlerAction;

    async fn handle_packet(
        self: &mut Self,
        ctx: &RuntimeContext,
        packet: &Packet,
    ) -> Result<HandlerResult<Self::HandlerAction>, RuntimeError>;

    async fn execute(
        &mut self,
        ctx: &RuntimeContext,
        result: HandlerResult<Self::HandlerAction>,
        writer: &mut PacketWriter,
    ) -> Result<(), RuntimeError>;
}

#[derive(Default)]
pub struct Credentials;

impl RuntimeRelay for Credentials {
    type HandlerAction = LoginAction;

    async fn handle_packet(
        self: &mut Self,
        ctx: &RuntimeContext,
        packet: &Packet,
    ) -> Result<HandlerResult<LoginAction>, RuntimeError> {
        let opcode = packet.opcode();
        match opcode {
            x if x == RecvOpcode::RequestLogin as i16 => {
                let handler = credentials::CredentialsHandler::new();
                handler
                    .handle(ctx, packet)
                    .await
                    .map_err(RuntimeError::from)
            }
            x if x == RecvOpcode::AcceptTOS as i16 => {
                let handler = tos::TOSHandler::new();
                handler
                    .handle(ctx, packet)
                    .await
                    .map_err(RuntimeError::from)
            }
            _ => Err(RuntimeError::NetworkError(UnsupportedOpcodeError(opcode))),
        }
    }

    async fn execute(
        self: &mut Self,
        ctx: &RuntimeContext,
        result: HandlerResult<LoginAction>,
        writer: &mut PacketWriter,
    ) -> Result<(), RuntimeError> {
        let actions = result.actions;
        for action in actions {
            match action {
                LoginAction::AcceptLogin { acc, hwid } => {
                    let mut packet = build::core::build_successful_login_packet(&acc, ctx)?;
                    ctx.shared_state.sessions.update(ctx.session_id, |session| {
                        session.account_id = Some(acc.id);
                        session.hwid = Some(hwid);
                        session.session_state = SessionState::Transition;
                    });
                    writer.send_encrypted_packet(&mut packet).await?;
                    ctx.shared_state.sessions.update(ctx.session_id, |session| {
                        session.session_state = SessionState::AfterLogin;
                    });
                }
                LoginAction::RejectLogin { reason, acc } => {
                    if let Some(acc) = acc {
                        ctx.shared_state.sessions.update(ctx.session_id, |session| {
                            session.account_id = Some(acc.id);
                            session.session_state = SessionState::Transition;
                        });
                    }
                    match reason {
                        RejectReason::InvalidCredentials => {
                            let mut packet = build::core::build_failed_login_packet(
                                credentials::StatusCode::InvalidCredentials as u8,
                            )?;
                            writer.send_encrypted_packet(&mut packet).await?
                        }
                        RejectReason::Banned => {
                            let mut packet = build::core::build_failed_login_packet(
                                credentials::StatusCode::Banned as u8,
                            )?;
                            writer.send_encrypted_packet(&mut packet).await?
                        }
                        RejectReason::PendingTOS => {
                            let mut packet = build::core::build_failed_login_packet(
                                credentials::StatusCode::PendingTOS as u8,
                            )?;
                            writer.send_encrypted_packet(&mut packet).await?
                        }
                        RejectReason::Playing => {
                            let mut packet = build::core::build_failed_login_packet(
                                credentials::StatusCode::Playing as u8,
                            )?;
                            writer.send_encrypted_packet(&mut packet).await?
                        }
                    }
                }
            }
        }
        Ok(())
    }
}

#[derive(Default)]
pub struct World;

impl RuntimeRelay for World {
    type HandlerAction = WorldAction;

    async fn handle_packet(
        self: &mut Self,
        ctx: &RuntimeContext,
        packet: &Packet,
    ) -> Result<HandlerResult<WorldAction>, RuntimeError> {
        let opcode = packet.opcode();
        match opcode {
            x if x == RecvOpcode::RequestLogin as i16 => {
                let handler = cc::ChangeChannelHandler::new();
                handler
                    .handle(ctx, packet)
                    .await
                    .map_err(RuntimeError::from)
            }
            _ => Err(RuntimeError::NetworkError(UnsupportedOpcodeError(opcode))),
        }
    }

    async fn execute(
        self: &mut Self,
        ctx: &RuntimeContext,
        result: HandlerResult<WorldAction>,
        writer: &mut PacketWriter,
    ) -> Result<(), RuntimeError> {
        let actions = result.actions;
        for action in actions {
            match action {
                WorldAction::ChangeChannel {
                    world_id,
                    channel_id,
                } => {
                    let channel =
                        channel::core::resolve_channel(channel_id, world_id, &ctx.shared_state)
                            .map_err(RuntimeError::from)?;
                    ctx.shared_state.sessions.update(ctx.session_id, |session| {
                        session.session_state = SessionState::Transition;
                    });
                    let mut packet = build::core::build_channel_change_packet(
                        &channel,
                        &ctx.shared_state.settings,
                    )?;
                    //         .map_err(|e| RuntimeError::Handler(e.to_string()))?;
                    // self.writer.send_packet(&mut redirect_packet).await?;
                    //
                    // self.world_tx
                    //     .send(ClientEvent::Disconnected {
                    //         client_id: self.client_id,
                    //     })
                    //     .await
                    //     .map_err(|_| RuntimeError::ChannelSend)?;
                    // self.client_id = 0;
                    // return Err(RuntimeError::ClientDisconnected);
                    // packet = build::core::build_disconnect_packet()?;
                    writer.send_encrypted_packet(&mut packet).await?
                }
                _ => (),
            }
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {

    #[test]
    fn test_credentials_relay() {
        println!("Relay test is not implemented");
    }
}
