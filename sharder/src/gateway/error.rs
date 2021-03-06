use thiserror::Error;
use std::fmt::Display;
use crate::gateway::outbound_message::OutboundMessage;
use crate::manager::FatalError;

#[derive(Error, Debug)]
pub enum GatewayError {
    #[error("t value on dispatch was not a string")]
    MissingEventType,

    #[error("d value on dispatch was not an object")]
    MissingEventData,

    #[error("shard had a None bot_id field")]
    NoneId,

    #[error("{0}")]
    GenericError(String),

    #[error("error while operating on json (serde): {0}")]
    JsonError(#[from] serde_json::Error),

    #[error("error while operating on json (simd): {0}")]
    SimdJsonError(#[from] simd_json::Error),

    #[error("error while operating on Redis: {0}")]
    RedisError(#[from] redis::RedisError),

    #[error("error while getting redis conn: {0}")]
    PoolError(#[from] deadpool::managed::PoolError<redis::RedisError>),

    #[error("error while operating on websocket: {0}")]
    WebsocketError(#[from] tokio_tungstenite::tungstenite::Error),

    #[error("error while reading oneshot channel: {0}")]
    RecvError(#[from] tokio::sync::oneshot::error::RecvError),

    #[error("error while sending message to writer: {0}")]
    SendMessageError(#[from] tokio::sync::mpsc::error::SendError<OutboundMessage>),

    #[error("error while sending message to error chan: {0}")]
    SendErrorError(#[from] tokio::sync::mpsc::error::SendError<FatalError>),

    #[error("error while sending message to chan: {0}")]
    SendError(#[from] tokio::sync::mpsc::error::SendError<()>),

    #[error("error while sending message to chan: {0}")]
    SendU16Error(#[from] tokio::sync::mpsc::error::SendError<u16>),

    #[error("error occurred while compressing payload: {0}")]
    CompressError(#[from] flate2::CompressError),

    #[error("error occurred while decompressing payload: {0}")]
    DecompressError(#[from] flate2::DecompressError),

    #[error("error occurred while operating on the cache: {0}")]
    CacheError(#[from] cache::CacheError),

    #[error("bot ID was missing on whitelabel identify")]
    MissingBotID(),

    #[error("error occurred while parsing utf8 bytes: {0}")]
    Utf8Error(#[from] std::str::Utf8Error),
}

impl GatewayError {
    pub fn custom(msg: impl Display) -> Self {
        GatewayError::GenericError(msg.to_string())
    }
}

impl<T> Into<Result<T, Self>> for GatewayError {
    fn into(self) -> Result<T, Self> {
        Err(self)
    }
}