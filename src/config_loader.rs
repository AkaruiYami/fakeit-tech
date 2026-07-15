#![allow(dead_code)]
use std::{fs, path::PathBuf};

use crate::config::Config;

pub fn load_config() -> Option<Config> {
    let mut path = dirs::config_dir().unwrap_or_else(|| PathBuf::from("."));

    path.push("fakeit-tech");
    path.push("config.toml");

    if !path.exists() {
        create_default(&path);
    }

    let content = fs::read_to_string(&path).ok()?;

    toml::from_str(&content).ok()?
}

fn create_default(path: &PathBuf) {
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent).ok();
    }

    let default = r#"
[core]
delay_min = 200
delay_max = 800

[modules]
build = true
hack = true
ai = true
cypher_square = true
matrix = true
"#;

    fs::write(path, default).ok();

    println!("Created default config at {:?}", path);
}
