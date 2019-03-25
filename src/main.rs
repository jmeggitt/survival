#![feature(custom_attribute)]

use amethyst::LoggerConfig;

fn main() -> amethyst::Result<()> {
    amethyst::start_logger(LoggerConfig::default());
    survival::run()
}
