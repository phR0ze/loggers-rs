use std::env;
use tracing_example;
use tracing::{Level, error, warn, info, debug, trace};
use tracing_subscriber;

fn main() {
    // configure global collector based on the max level given in the env variable LOG_LEVEL
    let loglevel= match env::var("LOG_LEVEL") {
        Ok(val) => val.parse().unwrap_or(Level::INFO),
        Err(_e) => Level::INFO,
    };
    tracing_subscriber::fmt().with_max_level(loglevel).init();

    // including number_of_yaks in this way will add a structured field to the event
    // at the end using the variables name and value `number_of_yaks=2`
    let number_of_yaks = 2;
    error!(number_of_yaks, "preparing to shave yaks");
    warn!(number_of_yaks, "preparing to shave yaks");
    info!(number_of_yaks, "preparing to shave yaks");
    debug!(number_of_yaks, "preparing to shave yaks");
    trace!(number_of_yaks, "preparing to shave yaks");

    // structured field with name and value called out
    let number_shaved = tracing_example::shave_all(number_of_yaks);
    info!(all_yaks_shaved = number_shaved == number_of_yaks, "yak shaving completed.");

    // log facade example
    tracing_example::logfacade::shave_yak_facade();
}