use crate::db::error::DatabaseError::DieselError;
use crate::runtime::error::RuntimeError;
use diesel::PgConnection;
use diesel::r2d2::{ConnectionManager, Pool};

pub type DbPool = Pool<ConnectionManager<PgConnection>>;

pub async fn spawn_db<F, T>(pool: DbPool, f: F) -> Result<T, RuntimeError>
where
    F: FnOnce(&mut PgConnection) -> Result<T, diesel::result::Error> + Send + 'static,
    T: Send + 'static,
{
    tokio::task::spawn_blocking(move || {
        let mut conn = pool.get().map_err(|_| diesel::result::Error::NotFound)?;
        f(&mut conn)
    })
    .await
    .map_err(RuntimeError::JoinError)?
    .map_err(DieselError)
    .map_err(RuntimeError::from)
}
