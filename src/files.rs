static DEFAULT_CONFIG_CONTENT: &'static str = r#"
debug = false

[global]
repository_path = "/tmp/utilitips"
content_root_path = "topics"
"#;

use std::io::prelude::*;
use std::fs::{File, create_dir_all};
use std::path::Path;

pub fn create_default_config() -> std::io::Result<()> {
    let path = Path::new(super::settings::DEFAULT_CONF_FILE);
    let mut file_w = File::create(path.display().to_string())?;
    file_w.write_all(DEFAULT_CONFIG_CONTENT.as_bytes())?; 
    Ok(())
}

pub fn validate_path(path: &Path, kind: &str) -> std::io::Result<()> {
    if !&path.exists() {
        println!("\tPath does not exist: {}, creating...", &path.display());
        match &kind[..] {
            "folder" => {
                create_dir_all(&path)?;
                Ok(())
            },
            "file" => {
                let file = format!("{}{}", &path.display().to_string(), ".md".to_string());
                File::create(file)?;
                Ok(())
            },
            _ => { 
                panic!("Error, {} type does not exists...", kind);
            }
        }
    }
    else {
        println!("Path {} of type {} already exists", path.display(), kind);
        Ok(())
    }
}
