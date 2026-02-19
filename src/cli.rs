use clap::Parser;

/// Fakeit-Tech is a simple decorative tools that is useless. This tool is only for
/// video production to fake the scene related IT or console usage.
#[derive(Parser)]
#[command(version, about, long_about = None)]
pub struct Cli {
    /// Module names seperated by white spaces.
    pub modules: Vec<String>,

    /// List down all the modules.
    #[arg(short, long, exclusive = true)]
    pub list: bool,

    /// Enable looping.
    #[arg(long)]
    pub _loop: bool,
}
