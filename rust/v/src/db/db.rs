use crate::config::settings;
use crate::db::error::DatabaseError;
use diesel::prelude::*;

pub fn establish_connection() -> Result<PgConnection, DatabaseError> {
    let db_url = settings::get_db_url();
    PgConnection::establish(&db_url).unwrap_or_else(|_| {
        panic!(
            "Expected a successful diesel database connection. Received a ConnectionError. URL: {}",
            db_url
        )
    })
}
