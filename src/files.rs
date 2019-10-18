static DEFAULT_CONFIG_CONTENT: &'static str = r#"
debug = false

[global]
repository_path = "/tmp/utilitips"
content_root_path = "topics"
"#;

use std::io::prelude::*;
use std::fs::File;
use std::path::Path;

pub fn create_default_config() -> std::io::Result<()> {
    let path = Path::new(super::settings::DEFAULT_CONF_FILE);
    let mut file_w = File::create(path.display().to_string())?;
    file_w.write_all(DEFAULT_CONFIG_CONTENT.as_bytes())?; 
    Ok(())
}
