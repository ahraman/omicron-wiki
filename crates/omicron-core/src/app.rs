pub struct Config {
    pub server_url: String,
    pub database_url: String,

    pub assets_dir: String,
}

impl Config {
    pub fn from_env() -> Result<Self, std::env::VarError> {
        Ok(Self {
            server_url: std::env::var("SERVER_URL")?,
            database_url: std::env::var("DATABASE_URL")?,

            assets_dir: std::env::var("ASSETS_DIR")?,
        })
    }
}
