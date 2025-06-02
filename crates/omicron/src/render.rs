use std::{ffi::OsStr, fs, path::PathBuf};

use tera::{Context, Tera, Value};

use crate::{Config, Error};

pub struct RenderManager {
    tera: Tera,
}

impl RenderManager {
    pub fn new(config: &Config) -> Result<Self, Error> {
        Ok(Self {
            tera: build_tera_instance(&config)?,
        })
    }

    pub fn render(&self, template_name: &str, context: Value) -> Result<String, Error> {
        Ok(self
            .tera
            .render(template_name, &Context::from_value(context)?)?)
    }
}

fn build_tera_instance(config: &Config) -> Result<Tera, Error> {
    let mut tera = Tera::default();
    register_tera_templates(&mut tera, &config.assets_dir)?;
    Ok(tera)
}

fn register_tera_templates(tera: &mut Tera, assets_dir: &str) -> Result<(), Error> {
    register_tera_templates_recursive(tera, PathBuf::from(format!("{assets_dir}/templates")))
}

fn register_tera_templates_recursive(tera: &mut Tera, root: PathBuf) -> Result<(), Error> {
    for file in fs::read_dir(root)? {
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
