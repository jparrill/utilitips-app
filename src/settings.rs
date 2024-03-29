pub static DEFAULT_CONF_FILE: &'static str = "conf/default.toml"; 

use config::{ConfigError, Config, File};
use std::path::Path;
use serde_derive::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Global {
    pub repository_path: String,
    pub content_root_path: String,
}

#[derive(Debug, Deserialize)]
pub struct Settings {
    pub debug: bool,
    pub global: Global,
}

impl Settings {
    pub fn new(cfg_arg: &str) -> Result<Self, ConfigError> {
        let mut s = Config::new();
        let cfg_file = Some(cfg_arg).unwrap_or(DEFAULT_CONF_FILE);
        let path = Path::new(cfg_file);

        // If config file does not exists, we creates a new one based on defaults
        if !path.exists() {
            println!("\tPath does not exist: {}, creating a new one with default options...", path.display());
            // Functions separated just to avoid trash in the screen if the file is big
            match super::files::create_default_config() {
                Ok(_)   => {},
                Err(why)  => panic!("Error generating default assets: {}", why),
            }
        }
        
        // We read from the default file and merge on settings, then return
        s.merge(File::with_name(cfg_file))?;
        s.try_into()
    }
}
