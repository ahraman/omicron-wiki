use std::{
    ffi::OsStr,
    fs,
    path::{Path, PathBuf},
};

use tera::{Context, Tera, Value};

use crate::{Error, asset::AssetManager};

pub struct RenderManager {
    tera: Tera,
}

impl RenderManager {
    pub fn new(asset_manager: &AssetManager) -> Result<Self, Error> {
        Ok(Self {
            tera: build_tera_instance(asset_manager)?,
        })
    }

    pub fn render(&self, template_name: &str, context: Value) -> Result<String, Error> {
        Ok(self
            .tera
            .render(template_name, &Context::from_value(context)?)?)
    }
}

fn build_tera_instance(asset_manager: &AssetManager) -> Result<Tera, Error> {
    let mut tera = Tera::default();
    register_tera_templates(&mut tera, &asset_manager.root_dir)?;
    Ok(tera)
}

fn register_tera_templates(tera: &mut Tera, assets_dir: &Path) -> Result<(), Error> {
    register_tera_templates_recursive(tera, assets_dir.join("templates"))
}

fn register_tera_templates_recursive(tera: &mut Tera, dir: PathBuf) -> Result<(), Error> {
    for file in fs::read_dir(dir)? {
        let file = file?;
        let kind = file.file_type()?;
        if kind.is_dir() {
            register_tera_templates_recursive(tera, file.path())?;
        } else if kind.is_file() {
            let path = file.path();
            if path.extension().is_some_and(|ext| ext == "tera") {
                let name = path
                    .file_stem()
                    .map(OsStr::to_str)
                    .flatten()
                    .ok_or(Error::AssetName(file.path()))?;

                tera.add_template_file(file.path(), Some(name))?;
            }
        }
    }

    Ok(())
}
