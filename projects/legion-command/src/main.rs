use clap::Parser;

use vpm::Application;

fn main() {
    _ = Application::parse().run();
}
