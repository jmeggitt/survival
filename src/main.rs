#![warn(clippy::pedantic, clippy::all)]
//#![allow(non_upper_case_globals)]
#![feature(custom_attribute, concat_idents)]
#![allow(dead_code)]

fn main() -> amethyst::Result<()> {
    use slog::Drain;

    amethyst::start_logger(amethyst::LoggerConfig::default());

    // Make sure to save the guard, see documentation for more information
    let decorator = slog_term::TermDecorator::new().force_color().build();
    let drain = slog_term::FullFormat::new(decorator).build().fuse();
    let async_drain = slog_async::Async::new(drain).build().fuse();
    let root_log = slog::Logger::root(
        async_drain,
        slog::o!("@" =>
         slog::FnValue(move |info| {
             format!("{}({}:{})",
                     info.module(),
                     info.file(),
                     info.line(),
                     )
         })
        ),
    );

    survival::run(&root_log)
}
