use clap::Parser;

#[derive(Parser)]
pub struct RunCommand {
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

impl RunCommand {
    pub fn run(&self) {
        if self.release {
            println!("Running in release mode: {}", self.optimize);
        } else {
            println!("Running in debug mode");
            // 在 debug 模式下执行运行逻辑
        }
    }
}