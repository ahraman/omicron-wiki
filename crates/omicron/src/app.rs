use std::{ops::Deref, sync::Arc};

use axum::extract::{FromRequestParts, State};

use crate::{Error, asset::AssetManager, render::RenderManager};

pub struct Config {
    pub server_url: String,

    pub assets_dir: String,
}

impl Config {
    pub fn from_env() -> Result<Self, Error> {
        Ok(Self {
            server_url: std::env::var("SERVER_URL")?,

            assets_dir: std::env::var("ASSETS_DIR")?,
        })
    }
}

pub struct App {
    pub config: Config,

    pub asset_manager: AssetManager,
    pub render_manager: RenderManager,
}

impl App {
    pub fn new(config: Config) -> Result<Self, Error> {
        let asset_manager = AssetManager::new(&config)?;
        Ok(Self {
            render_manager: RenderManager::new(&asset_manager)?,
            asset_manager,

            config,
        })
    }
}

#[derive(Clone, FromRequestParts)]
#[from_request(via(State))]
pub struct AppState(pub Arc<App>);

impl Deref for AppState {
    type Target = App;

    fn deref(&self) -> &App {
        &self.0
    }
}
