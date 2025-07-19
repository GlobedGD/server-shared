#[derive(Clone, Debug)]
pub struct GameServerData {
    pub id: u8,
    pub address: heapless::String<64>,
    pub string_id: heapless::String<32>,
    pub name: heapless::String<64>,
    pub region: heapless::String<32>,
}
