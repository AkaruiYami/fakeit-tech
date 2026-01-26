pub mod ai;
pub mod build;
pub mod hack;

use crate::config::ModuleConfig;
use crate::engine::FakeModule;

pub fn load_modules(cfg: &ModuleConfig) -> Vec<Box<dyn FakeModule>> {
    let mut modules: Vec<Box<dyn FakeModule>> = Vec::new();

    if cfg.build {
        modules.push(Box::new(build::BuildModule));
    }

    if cfg.hack {
        modules.push(Box::new(hack::HackModule));
    }

    if cfg.ai {
        modules.push(Box::new(ai::AiModule));
    }

    modules
}
