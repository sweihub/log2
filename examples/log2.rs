use log2::*;

fn main() {
    let _log2 = log2::stdout()
        .level("warn")
        .module(false)
        .start();

    log2::set_level(log2::level::Info);
    log2::set_level("debug");
    _log2.set_level("trace");

    trace!("send order request to server");
    debug!("receive order response");
    info!("order was executed");
    warn!("network speed is slow");
    error!("network connection was broken");
}
