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
use crate::runtime::session::{LoginSession, LoginSessionState};
use crate::runtime::state::SharedState;
use rand::{RngExt, rng};
use std::net::SocketAddr;
use tokio::net::TcpStream;

pub struct RuntimeContext {
    pub session_id: u32,
    pub state: SharedState,
}

pub struct Runtime<T: RuntimeRelay> {
    reader: PacketReader,
    writer: PacketWriter,
    addr: SocketAddr,
    state: SharedState,
    relay: T,
    session_id: u32,
}

impl<T: RuntimeRelay + Default + Send> Runtime<T> {
    pub async fn new(
        state: SharedState,
        stream: TcpStream,
        addr: SocketAddr,
    ) -> Result<Self, RuntimeError> {
        let (recv_iv, send_iv) = {
            let mut recv_iv = vec![0u8; 4];
            let mut send_iv = vec![0u8; 4];
            let mut rng = rng();
            rng.fill(&mut recv_iv[..]);
            rng.fill(&mut send_iv[..]);
            (recv_iv, send_iv)
        };
        let (read_half, write_half) = stream.into_split();
        let reader = PacketReader::new(read_half, &recv_iv, &state)?;
        let writer = PacketWriter::new(write_half, &send_iv, &state)?;
        let session_id = state.sessions.insert(LoginSession {
            id: 0,
            account_id: None,
            hwid: None,
            login_state: LoginSessionState::BeforeLogin,
            selected_world_id: None,
            selected_channel_id: None,
        });
        Ok(Self {
            reader,
            writer,
            addr,
            relay: T::default(),
            state,
            session_id,
        })
    }

    pub async fn run(self: &mut Self) -> Result<(), RuntimeError> {
        loop {
            let packet = self.reader.read_packet().await?;
            let ctx = RuntimeContext {
                session_id: self.session_id,
                state: self.state.clone(),
            };
            let result = self.relay.handle_packet(&ctx, &packet).await?;
            self.relay.execute(&ctx, result, &mut self.writer).await?
        }
    }
}

trait RuntimeRelay {
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
                    ctx.state.sessions.update(ctx.session_id, |session| {
                        session.account_id = Some(acc.id);
                        session.hwid = Some(hwid);
                        session.login_state = LoginSessionState::Transition;
                    });
                    writer.send_packet(&mut packet).await?;
                    ctx.state.sessions.update(ctx.session_id, |session| {
                        session.login_state = LoginSessionState::AfterLogin;
                    });
                }
                LoginAction::RejectLogin { reason, acc } => {
                    if let Some(acc) = acc {
                        ctx.state.sessions.update(ctx.session_id, |session| {
                            session.account_id = Some(acc.id);
                            session.login_state = LoginSessionState::Transition;
                        });
                    }
                    match reason {
                        RejectReason::InvalidCredentials => {
                            let mut packet = build::core::build_failed_login_packet(
                                credentials::StatusCode::InvalidCredentials as u8,
                            )?;
                            writer.send_packet(&mut packet).await?
                        }
                        RejectReason::Banned => {
                            let mut packet = build::core::build_failed_login_packet(
                                credentials::StatusCode::Banned as u8,
                            )?;
                            writer.send_packet(&mut packet).await?
                        }
                        RejectReason::PendingTOS => {
                            let mut packet = build::core::build_failed_login_packet(
                                credentials::StatusCode::PendingTOS as u8,
                            )?;
                            writer.send_packet(&mut packet).await?
                        }
                        RejectReason::Playing => {
                            let mut packet = build::core::build_failed_login_packet(
                                credentials::StatusCode::Playing as u8,
                            )?;
                            writer.send_packet(&mut packet).await?
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
        let packet = Packet::new_empty();
        let actions = result.actions;
        for action in actions {
            match action {
                WorldAction::ChangeChannel {
                    world_id,
                    channel_id,
                } => {
                    let channel = resolve_login_channel(world_id, channel_id)
                        .map_err(ChannelNotFound)
                        .map_err(RuntimeError::from)?;
                    //
                    // session.selected_world_id = Some(i16::from(world_id));
                    // session.selected_channel_id = Some(i16::from(channel_id));
                    // session.state = SessionState::Transition;
                    // db::session::update_session(session)
                    //     .map_err(|e| RuntimeError::Handler(e.to_string()))?;
                    //
                    // let mut redirect_packet =
                    //     build::world::channel::build_channel_change(channel.host, channel.port)
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
                    writer.send_packet(&mut packet).await?
                }
                _ => (),
            }
        }
        Ok(())
    }
}
