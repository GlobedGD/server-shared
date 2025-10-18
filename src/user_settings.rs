#[derive(Clone, Copy, Debug, Default)]
pub struct UserSettings {
    pub hide_in_level: bool,
    pub hide_in_menus: bool,
    pub hide_roles: bool,
}

impl UserSettings {
    pub fn from_reader(reader: crate::schema::shared::user_settings::Reader) -> Self {
        Self {
            hide_in_level: reader.get_hide_in_level(),
            hide_in_menus: reader.get_hide_in_menus(),
            hide_roles: reader.get_hide_roles(),
        }
    }
}
