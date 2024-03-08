use clap::Parser;

use vpp::Application;

fn main() {
    _ = Application::parse().run();
}
