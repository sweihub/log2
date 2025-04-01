use log2::*;

fn main() {
    // simple
    // let _log2 = log2::start();

    // configurable
    let _log2 = log2::stdout()
        .level("trace")
        .module(false)
        .module_with_line(true)
        .module_filter(|m| !m.is_empty())
        .start();

    /*
    log2::set_level(log2::level::Info);
    log2::set_level("debug");
    log2::set_level("trace");
    */

    trace!("send order request to server");
    debug!("receive order response");
    info!("order was executed");
    warn!("network speed is slow");
    error!("network connection was broken");
}
