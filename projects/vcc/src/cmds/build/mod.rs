use clap::Parser;

#[derive(Parser)]
pub struct BuildCommand {
    /// Running under debug mode
    #[clap(long)]
    debug: bool,
    /// Running under release mode
    #[clap(long)]
    release: bool,
    /// Optimize Level
    #[arg(short, long, action = clap::ArgAction::Count)]
    optimize: u8,
}

impl BuildCommand {
    pub fn run(&self) {
        if self.release {
            println!("Running in release mode: {}", self.optimize);
        } else {
            println!("Building in debug mode");
            // 在 debug 模式下执行构建逻辑
        }
    }
}