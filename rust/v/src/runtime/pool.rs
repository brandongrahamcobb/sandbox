use crate::db::error::DatabaseError::DatabaseConnectionError;
use crate::runtime::error::RuntimeError;

pub async fn spawn_db<F, T>(f: F) -> Result<T, RuntimeError>
where
    F: FnOnce() -> Result<T, diesel::result::Error> + Send + 'static,
    T: Send + 'static,
{
    tokio::task::spawn_blocking(f)
        .await
        .map_err(|e| RuntimeError::RuntimeJoinError)
        .map_err(DatabaseConnectionError)
}
