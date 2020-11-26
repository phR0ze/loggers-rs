use slog::Drain;
use slog::*;
use slog_term;
use std::env;
use std::sync::Mutex;

fn main() {
    let decorator = slog_term::TermDecorator::new().build();
    let drain = Mutex::new(slog_term::FullFormat::new(decorator).build()).fuse();
    let log = slog::Logger::root(drain, o!("root" => env!("CARGO_PKG_VERSION")));

    // // configure global collector based on the max level given in the env variable LOG_LEVEL
    // let loglevel = match env::var("LOG_LEVEL") {
    //     Ok(val) => val.parse().unwrap_or(Level::INFO),
    //     Err(_e) => Level::INFO,
    // };
    // tracing_subscriber::fmt()
    //     .with_max_level(loglevel)
    //     //.json()
    //     .init();

    // // including number_of_yaks in this way will add a structured field to the event
    // // at the end using the variables name and value `number_of_yaks=2`
    let number_of_yaks = 2;
    trace!(log, "preparing to shave yaks"; "nubmer_of_yaks" => number_of_yaks);
    debug!(log, "preparing to shave yaks"; "nubmer_of_yaks" => number_of_yaks);
    info!(log, "preparing to shave yaks"; "nubmer_of_yaks" => number_of_yaks);
    warn!(log, "preparing to shave yaks"; "nubmer_of_yaks" => number_of_yaks);
    error!(log, "preparing to shave yaks"; "nubmer_of_yaks" => number_of_yaks);
    crit!(log, "preparing to shave yaks"; "nubmer_of_yaks" => number_of_yaks);

    // // structured field with name and value called out
    // let number_shaved = tracing_example::shave_all(number_of_yaks);
    // info!(
    //     all_yaks_shaved = number_shaved == number_of_yaks,
    //     "yak shaving completed."
    // );

    // // log facade example
    // tracing_example::logfacade::shave_yak_facade();
}
