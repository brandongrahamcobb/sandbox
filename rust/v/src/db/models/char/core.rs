use crate::db::schema::characters;
use diesel::Insertable;
use std::time::SystemTime;

#[derive(Insertable)]
#[diesel(table_name = characters)]
pub struct Character {
    pub id: i64,
    pub account: i64,
    pub world: i16,
    pub ign: String,
    pub level: i16,
    pub exp: i64,
    pub strength: i32,
    pub dexterity: i32,
    pub luck: i32,
    pub intelligence: i32,
    pub hp: i32,
    pub mp: i32,
    pub max_hp: i32,
    pub max_mp: i32,
    pub ap: i16,
    pub fame: i16,
    pub meso: i32,
    pub job: i16,
    pub face: i32,
    pub hair: i32,
    pub hair_color: i32,
    pub skin: i32,
    pub gender: i16,
    pub created_at: SystemTime,
    pub map: i32,
    pub updated_at: SystemTime,
}
