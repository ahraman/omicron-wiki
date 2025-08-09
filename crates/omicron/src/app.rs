use std::{ops::Deref, sync::Arc};

use axum::extract::{FromRequestParts, State};

pub use crate::core::app::Config;
use crate::{Error, asset::AssetManager, db::DbManager, render::RenderManager};

pub struct App {
    pub config: Config,

    pub asset_manager: AssetManager,
    pub db_manager: DbManager,
    pub render_manager: RenderManager,
}

impl App {
    pub async fn new(config: Config) -> Result<Self, Error> {
        let asset_manager = AssetManager::new(&config)?;
        Ok(Self {
            db_manager: DbManager::new(&config).await?,
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
