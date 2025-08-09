use sqlx::PgPool;

use crate::{Config, Error};

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
