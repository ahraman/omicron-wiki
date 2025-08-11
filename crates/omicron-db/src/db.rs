use omicron_core::app::Config;
use sqlx::PgPool;

use crate::Error;

pub struct Db {
    pub conn: PgPool,
}

impl Db {
    async fn new(database_url: &str) -> Result<Self, Error> {
        Ok(Self {
            conn: PgPool::connect(database_url).await?,
        })
    }
}

pub struct DbManager {
    pub db: Db,
}

impl DbManager {
    pub async fn new(config: &Config) -> Result<Self, Error> {
        Ok(Self {
            db: Db::new(&config.database_url).await?,
        })
    }
}
