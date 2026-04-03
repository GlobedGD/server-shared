#[cfg(feature = "srvc")]
use crate::schema::srvc::{srv_user_data, status_message};

use crate::{
    encoding::{DataDecodeError, heapless_str_from_reader},
    schema::shared::{player_display_data, player_icon_data},
};

#[cfg(feature = "srvc")]
pub const SRVC_MAGIC: u64 = 0x92869be51214ba4f;
#[cfg(feature = "srvc")]
pub const SRVC_PROTOCOL_VERSION: u32 = 2;

#[derive(Clone, Debug)]
pub struct GameServerData {
    pub id: u8,
    pub address: heapless::String<64>,
    pub string_id: heapless::String<32>,
    pub name: heapless::String<64>,
    pub region: heapless::String<32>,
}

#[derive(Clone, Copy, Debug)]
pub struct PlayerIconData {
    pub cube: i16,
    pub ship: i16,
    pub ball: i16,
    pub ufo: i16,
    pub wave: i16,
    pub robot: i16,
    pub spider: i16,
    pub swing: i16,
    pub jetpack: i16,
    pub color1: u16,
    pub color2: u16,
    pub glow_color: u16,
    pub death_effect: u8,
    pub trail: u8,
    pub ship_trail: u8,
    pub default_mini: bool,
}

impl Default for PlayerIconData {
    fn default() -> Self {
        Self {
            cube: 0,
            ship: 0,
            ball: 0,
            ufo: 0,
            wave: 0,
            robot: 0,
            spider: 0,
            swing: 0,
            jetpack: 0,
            color1: 0,
            color2: 0,
            glow_color: 0,
            death_effect: 1,
            trail: 255,
            ship_trail: 255,
            default_mini: false,
        }
    }
}

impl PlayerIconData {
    pub fn from_reader(reader: player_icon_data::Reader<'_>) -> Result<Self, DataDecodeError> {
        Ok(Self {
            cube: reader.get_cube(),
            ship: reader.get_ship(),
            ball: reader.get_ball(),
            ufo: reader.get_ufo(),
            wave: reader.get_wave(),
            robot: reader.get_robot(),
            spider: reader.get_spider(),
            swing: reader.get_swing(),
            jetpack: reader.get_jetpack(),
            color1: reader.get_color1(),
            color2: reader.get_color2(),
            glow_color: reader.get_glow_color(),
            death_effect: reader.get_death_effect(),
            trail: reader.get_trail(),
            ship_trail: reader.get_ship_trail(),
            default_mini: reader.get_default_mini(),
        })
    }

    pub fn encode(&self, mut builder: player_icon_data::Builder<'_>) {
        builder.set_cube(self.cube);
        builder.set_ship(self.ship);
        builder.set_ball(self.ball);
        builder.set_ufo(self.ufo);
        builder.set_wave(self.wave);
        builder.set_robot(self.robot);
        builder.set_spider(self.spider);
        builder.set_swing(self.swing);
        builder.set_jetpack(self.jetpack);
        builder.set_color1(self.color1);
        builder.set_color2(self.color2);
        builder.set_glow_color(self.glow_color);
        builder.set_death_effect(self.death_effect);
        builder.set_trail(self.trail);
        builder.set_ship_trail(self.ship_trail);
        builder.set_default_mini(self.default_mini);
    }
}

#[derive(Clone, Debug, Default)]
pub struct PlayerDisplayData {
    pub account_id: i32,
    pub user_id: i32,
    pub username: heapless::String<32>,
    pub icons: PlayerIconData,
}

impl PlayerDisplayData {
    pub fn from_reader(reader: player_display_data::Reader<'_>) -> Result<Self, DataDecodeError> {
        Ok(Self {
            account_id: reader.get_account_id(),
            user_id: reader.get_user_id(),
            username: heapless_str_from_reader(reader.get_username()?)?,
            icons: PlayerIconData::from_reader(reader.get_icons()?)
                .map_err(|_| DataDecodeError::ValidationFailed)?,
        })
    }

    pub fn encode(&self, mut builder: player_display_data::Builder<'_>) {
        builder.set_account_id(self.account_id);
        builder.set_user_id(self.user_id);
        builder.set_username(self.username.as_str());
        self.icons.encode(builder.reborrow().init_icons());
    }
}

#[cfg(feature = "srvc")]
#[derive(Clone, Debug, Default)]
pub struct SrvUserData {
    pub account_id: i32,
    pub can_use_voice: bool,
    pub can_use_quick_chat: bool,
    pub is_banned: bool,
    pub is_muted: bool,
    pub is_linked: bool,
}

#[cfg(feature = "srvc")]
impl SrvUserData {
    pub fn from_reader(reader: srv_user_data::Reader<'_>) -> Result<Self, DataDecodeError> {
        Ok(Self {
            account_id: reader.get_account_id(),
            can_use_voice: reader.get_can_use_voice(),
            can_use_quick_chat: reader.get_can_use_quick_chat(),
            is_banned: reader.get_is_banned(),
            is_muted: reader.get_is_muted(),
            is_linked: reader.get_is_linked(),
        })
    }

    pub fn encode(&self, mut builder: srv_user_data::Builder<'_>) {
        builder.set_account_id(self.account_id);
        builder.set_can_use_voice(self.can_use_voice);
        builder.set_can_use_quick_chat(self.can_use_quick_chat);
        builder.set_is_banned(self.is_banned);
        builder.set_is_muted(self.is_muted);
        builder.set_is_linked(self.is_linked);
    }
}

#[cfg(feature = "srvc")]
#[derive(Clone, Debug, Default)]
pub struct SrvStatusData {
    pub clients: u32,
    pub auth_clients: u32,
    pub rooms: u32,
    pub sessions: u32,
    pub total_connections: u64,
    pub total_data_messages: u64,
}

#[cfg(feature = "srvc")]
impl SrvStatusData {
    pub fn from_reader(reader: status_message::Reader<'_>) -> Result<Self, DataDecodeError> {
        Ok(Self {
            clients: reader.get_clients(),
            auth_clients: reader.get_auth_clients(),
            rooms: reader.get_rooms(),
            sessions: reader.get_sessions(),
            total_connections: reader.get_total_connections(),
            total_data_messages: reader.get_total_data_messages(),
        })
    }

    pub fn encode(&self, mut builder: status_message::Builder<'_>) {
        builder.set_clients(self.clients);
        builder.set_sessions(self.sessions);
        builder.set_auth_clients(self.auth_clients);
        builder.set_rooms(self.rooms);
        builder.set_total_connections(self.total_connections);
        builder.set_total_data_messages(self.total_data_messages);
    }
}
