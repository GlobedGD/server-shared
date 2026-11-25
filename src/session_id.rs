use std::fmt::Display;

pub const NS_EDITOR_COLLAB: u32 = 1;

#[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd, Hash, Debug, Default)]
pub struct LevelId {
    pub ns: u32,
    pub id: u64,
}

#[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd, Hash, Debug, Default)]
pub struct SessionId {
    pub level: LevelId,
    pub room_id: u32,
    pub server_id: u8,
}

impl SessionId {
    /// Creates a new `SessionId` from the given server ID, level ID, and room ID.
    pub fn from_parts(server_id: u8, level_id: LevelId, room_id: u32) -> SessionId {
        Self {
            level: level_id,
            room_id,
            server_id,
        }
    }

    pub fn from_parts_vanilla(server_id: u8, level_id: i32, room_id: u32) -> SessionId {
        Self::from_parts(
            server_id,
            LevelId {
                ns: 0,
                id: level_id as u64,
            },
            room_id,
        )
    }

    pub fn from_legacy(id: u64) -> SessionId {
        let server_id = (id >> 56) as u8;
        let room_id = ((id >> 32) & 0x00ffffff) as u32;
        let level_id = (id & 0xffffffff) as i32;

        Self::from_parts_vanilla(server_id, level_id, room_id)
    }

    pub fn from_legacy_ec(id: u64) -> SessionId {
        Self {
            level: LevelId {
                ns: NS_EDITOR_COLLAB,
                id,
            },
            room_id: 0,
            server_id: 0,
        }
    }

    pub fn as_legacy(&self) -> Option<u64> {
        if self.level.ns != 0 {
            return None;
        }

        Some(
            ((self.server_id as u64) << 56)
                | (((self.room_id & 0x00ffffff) as u64) << 32)
                | (self.level.id as u32 as u64),
        )
    }

    pub fn vanilla_level_id(&self) -> Option<i32> {
        if self.level.ns != 0 {
            return None;
        }

        Some(self.level.id as i32)
    }

    pub fn from_reader(reader: crate::schema::shared::session_id::Reader<'_>) -> SessionId {
        Self {
            server_id: reader.get_server_id(),
            level: LevelId {
                ns: reader.get_namespace(),
                id: reader.get_level_id(),
            },
            room_id: reader.get_room_id(),
        }
    }

    pub fn is_zero(&self) -> bool {
        self.level.ns == 0 && self.level.id == 0 && self.server_id == 0 && self.room_id == 0
    }

    pub fn is_vanilla_level(&self) -> bool {
        self.level.ns == 0
    }

    pub fn is_eq_vanilla_level(&self, id: i32) -> bool {
        self.level.ns == 0 && self.level.id == id as u64
    }
}

impl Display for SessionId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if self.level.ns == 0 {
            f.write_str("vanilla")?;
        } else {
            write!(f, "ns {}", self.level.ns)?;
        }

        write!(
            f,
            " level {} (server {}, room {})",
            self.level.id, self.server_id, self.room_id
        )
    }
}
