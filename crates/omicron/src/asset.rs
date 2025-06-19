use std::{
    collections::HashMap,
    ffi::OsStr,
    path::PathBuf,
    sync::{Arc, RwLock},
};

use crate::{Config, Error};

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct AssetKey {
    pub path: String,
}

impl AssetKey {
    pub fn new(path: String) -> Self {
        Self { path }
    }
}

impl From<String> for AssetKey {
    fn from(value: String) -> Self {
        Self::new(value)
    }
}

pub struct Asset {
    pub data: Box<[u8]>,
    pub meta: AssetMeta,
}

impl Asset {
    fn new(data: Box<[u8]>, meta: AssetMeta) -> Self {
        Self { data, meta }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum ContentType {
    // text
    Text,
    Html,
    Css,
    Javascript,

    // generic
    #[default]
    Unknown,
}

impl ContentType {
    pub fn from_extension(extension: Option<&OsStr>) -> Self {
        match extension.map(OsStr::to_str).flatten() {
            None => Self::Unknown,
            Some(extension) => match extension {
                "txt" => Self::Text,
                "html" => Self::Html,
                "css" => Self::Css,
                "js" => Self::Javascript,
                _ => Self::Unknown,
            },
        }
    }
}

impl std::fmt::Display for ContentType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Self::Css => "text/css",
                Self::Javascript => "text/javascript",
                _ => "application/octet-stream",
            }
        )
    }
}

#[derive(Debug, Clone, Default)]
pub struct AssetMeta {
    pub content_type: ContentType,
}

impl AssetMeta {
    fn new(content_type: ContentType) -> Self {
        Self { content_type }
    }
}

pub struct AssetManager {
    pub root_dir: PathBuf,

    cache: Cache,
}

impl AssetManager {
    pub fn new(config: &Config) -> Result<Self, Error> {
        Ok(Self {
            root_dir: PathBuf::from(&config.assets_dir),

            cache: Cache::new(),
        })
    }

    pub fn load_transient(&self, key: &AssetKey) -> Result<Arc<Asset>, Error> {
        let path = self.root_dir.join(&key.path);
        let meta = AssetMeta::new(ContentType::from_extension(path.extension()));
        println!("Loaded asset: `{}`", key.path);
        Ok(Arc::new(Asset::new(
            std::fs::read(path)?.into_boxed_slice(),
            meta,
        )))
    }

    pub fn load(&self, key: AssetKey) -> Result<Arc<Asset>, Error> {
        self.cache.get_or_else(key, |key| self.load_transient(key))
    }

    pub fn reload(&mut self, key: AssetKey) -> Result<Arc<Asset>, Error> {
        _ = self.cache.remove(&key);
        self.load(key)
    }

    pub fn clear_cache(&mut self) {
        self.cache.clear()
    }
}

struct Cache {
    inner: RwLock<HashMap<AssetKey, Arc<Asset>>>,
}

impl Cache {
    pub fn new() -> Self {
        Self {
            inner: RwLock::new(HashMap::new()),
        }
    }

    pub fn get_or_else<F>(&self, key: AssetKey, load: F) -> Result<Arc<Asset>, Error>
    where
        F: FnOnce(&AssetKey) -> Result<Arc<Asset>, Error>,
    {
        Ok(if let Some(asset) = self.get(&key) {
            asset
        } else {
            let asset = load(&key)?;
            self.inner
                .write()
                .expect("RwLock poisoned")
                .entry(key)
                .insert_entry(asset)
                .get()
                .clone()
        })
    }

    pub fn get(&self, key: &AssetKey) -> Option<Arc<Asset>> {
        self.inner
            .read()
            .expect("RwLock poisoned")
            .get(key)
            .cloned()
    }

    pub fn remove(&mut self, key: &AssetKey) -> Option<Arc<Asset>> {
        self.inner.write().expect("RwLock poisoned").remove(key)
    }

    pub fn clear(&mut self) {
        self.inner.write().expect("RwLock poisoned").clear();
    }
}
