#![feature(custom_attribute)]
//#![allow(dead_code)]

use amethyst::LoggerConfig;

fn main() -> amethyst::Result<()> {
    amethyst::start_logger(LoggerConfig::default());
    survival::run()
}
