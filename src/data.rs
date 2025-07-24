use crate::{
    encoding::{DataDecodeError, heapless_str_from_reader},
    schema::shared::{player_display_data, player_icon_data},
};

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
