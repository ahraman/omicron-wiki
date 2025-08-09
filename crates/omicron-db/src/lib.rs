use omicron_core::app::Config;
use sqlx::PgPool;

pub struct DbManager {
    pub conn: PgPool,
}

impl DbManager {
    pub async fn new(config: &Config) -> Result<Self, Error> {
        Ok(Self {
            conn: PgPool::connect(&config.database_url).await?,
        })
    }
}

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error(transparent)]
    Sqlx(#[from] sqlx::Error),
}
