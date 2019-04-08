#![feature(custom_attribute)]
use amethyst::LoggerConfig;
use log::LevelFilter;
use std::env;

fn main() -> amethyst::Result<()> {
    env::set_var("RUST_BACKTRACE", "1");
    amethyst::start_logger(LoggerConfig{
        level_filter: LevelFilter::Debug,
        ..Default::default()
    });
    survival::run()
}
