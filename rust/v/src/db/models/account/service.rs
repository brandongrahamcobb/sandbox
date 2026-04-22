use crate::db::models::account::core::Account;
use crate::db::schema::accounts;
use crate::runtime::relay::RuntimeContext;
use diesel::expression_methods::*;
use diesel::{QueryDsl, QueryResult, RunQueryDsl, SaveChangesDsl};

pub fn get_account_by_username(user: &str, ctx: &RuntimeContext) -> QueryResult<Account> {
    let mut conn = ctx.state.db.get().map_err(|e| {
        diesel::result::Error::DatabaseError(
            diesel::result::DatabaseErrorKind::UnableToSendCommand,
            Box::new(e.to_string()),
        )
    })?;
    accounts::table
        .filter(accounts::username.eq(&user))
        .first::<Account>(&mut conn)
}

pub fn get_account_by_id(id: i64, ctx: &RuntimeContext) -> QueryResult<Account> {
    let mut conn = ctx.state.db.get().map_err(|e| {
        diesel::result::Error::DatabaseError(
            diesel::result::DatabaseErrorKind::UnableToSendCommand,
            Box::new(e.to_string()),
        )
    })?;
    accounts::table
        .filter(accounts::id.eq(id))
        .first::<Account>(&mut conn)
}

pub fn update(acc: &Account, ctx: &RuntimeContext) -> QueryResult<Account> {
    let mut conn = ctx.state.db.get().map_err(|e| {
        diesel::result::Error::DatabaseError(
            diesel::result::DatabaseErrorKind::UnableToSendCommand,
            Box::new(e.to_string()),
        )
    })?;
    acc.save_changes(&mut conn)
}
