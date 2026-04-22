#[derive(Clone, Debug)]
pub struct Channel {
    pub world_id: u8,
    pub channel_id: u8,
    pub name: String,
    pub capacity: u16,
}
