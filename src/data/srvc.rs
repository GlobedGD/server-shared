//! srvc is the name for shared structs used in the inter-(s)e(rv)er (c)ommunication,
//! aka game server <=> central server
//!
//! Unlike the client communication, this uses serde msgpack since both servers are Rust and this is much more maintainable.

use serde::{Deserialize, Serialize};

use crate::{
    BinaryMultiColor, UsernameString,
    data::{ClientAccountData, GameServerData},
};

pub use rmp_serde::{decode::Error as SrvDecodeError, encode::Error as SrvEncodeError};

pub const SRVC_MAGIC: u64 = 0x92869be51214ba4f;
pub const SRVC_PROTOCOL_VERSION: u32 = 3;

#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct SrvStatusData {
    pub clients: u32,
    pub auth_clients: u32,
    pub rooms: u32,
    pub sessions: u32,
    pub total_connections: u64,
    pub total_data_messages: u64,
    pub server_load: f32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SrvRole {
    pub id: u8,
    pub string_id: String,
    pub can_moderate: bool,
}

impl SrvRole {
    pub fn new(id: u8, string_id: &str, can_moderate: bool) -> Self {
        Self {
            id,
            string_id: string_id.to_owned(),
            can_moderate,
        }
    }
}

#[derive(Clone, Default, Debug, Serialize, Deserialize)]
pub struct SrvUserData {
    pub account_id: i32,
    pub user_id: i32,
    pub username: UsernameString,
    pub role_ids: Vec<u8>,
    pub name_color: Option<BinaryMultiColor>,
    pub is_moderator: bool,

    pub is_linked: bool,
    pub can_use_voice: bool,
    pub can_use_quick_chat: bool,
    pub is_banned: bool,
    pub is_muted: bool,
}

impl SrvUserData {
    pub fn account_data(&self) -> ClientAccountData {
        ClientAccountData {
            account_id: self.account_id,
            user_id: self.user_id,
            username: self.username.clone(),
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub enum SrvMessage {
    Login {
        password: String,
        data: GameServerData,
    },

    LoginFailed {
        reason: String,
    },

    LoginOk {
        script_key: String,
        roles: Vec<SrvRole>,
    },

    NotifyRoomCreated {
        room_id: u32,
        passcode: u32,
        owner: i32,
    },

    NotifyRoomDeleted {
        room_id: u32,
    },

    RoomCreatedAck {
        room_id: u32,
    },

    // TODO ?
    NotifyUserData {
        data: SrvUserData,
    },

    NotifyKickUser {
        id: i32,
    },

    UserLogin {
        request_id: u64,
        account_id: i32,
        secret: u64,
        // TODO: deprecated
        token: String,
    },

    UserLoginResponse {
        request_id: u64,
        data: Option<SrvUserData>,
    },

    //
    ReloadConfig,

    Status(SrvStatusData),
}

impl SrvMessage {
    pub fn encode(&self) -> Result<Vec<u8>, rmp_serde::encode::Error> {
        rmp_serde::to_vec_named(self)
    }

    pub fn decode(data: &[u8]) -> Result<Self, rmp_serde::decode::Error> {
        rmp_serde::from_slice(data)
    }
}
