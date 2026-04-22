use crate::db::schema::accounts;
use diesel::prelude::*;
use std::time::SystemTime;

#[derive(Identifiable, Queryable, AsChangeset)]
#[diesel(table_name = accounts)]
pub struct Account {
    pub id: i64,
    pub username: String,
    pub password: String,
    pub pin: String,
    pub pic: String,
    pub last_login_at: Option<SystemTime>,
    pub created_at: SystemTime,
    pub character_slots: i16,
    pub gender: i16,
    pub accepted_tos: bool,
    pub banned: bool,
    pub playing: bool,
    pub updated_at: SystemTime,
}
