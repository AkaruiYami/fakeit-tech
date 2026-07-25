use serde::Deserialize;

#[derive(Deserialize, Debug, Default)]
pub struct Config {
    #[serde(default)]
    pub core: CoreConfig,
    #[serde(default)]
    pub modules: ModuleConfig,
}

#[derive(Deserialize, Debug)]
pub struct CoreConfig {
    pub delay_min: u64,
    pub delay_max: u64,
}

impl Default for CoreConfig {
    fn default() -> Self {
        Self {
            delay_min: 200,
            delay_max: 800,
        }
    }
}

#[derive(Deserialize, Debug, Default)]
pub struct ModuleConfig {
    pub build: Option<bool>,
    pub hack: Option<bool>,
    pub ai: Option<bool>,
    pub cypher_square: Option<bool>,
    pub matrix: Option<bool>,
}
