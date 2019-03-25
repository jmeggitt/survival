#![warn(clippy::pedantic, clippy::all)]
#![feature(custom_attribute, concat_idents)]
#![allow(dead_code)]

fn main() -> amethyst::Result<()> {
    amethyst::start_logger(Default::default());
    survival::run()
}
