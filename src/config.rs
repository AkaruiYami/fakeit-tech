use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct Config {
    pub core: CoreConfig,
    pub modules: ModuleConfig,
}

#[derive(Deserialize, Debug)]
pub struct CoreConfig {
    pub delay_min: u64,
    pub delay_max: u64,
}

#[derive(Deserialize, Debug)]
pub struct ModuleConfig {
    pub build: bool,
    pub hack: bool,
    pub ai: bool,
    pub network: bool,
}
