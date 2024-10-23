use log2::*;

// cargo test -- --nocapture
#[test]
fn log_to_stdout() {
    // let _log2 = log2::start();
    let _log2 = log2::stdout()
                .module(false)
                .start();
    trace!("send order request to server");
    debug!("receive order response");
    info!("order was executed");
    warn!("network speed is slow");
    error!("network connection was broken");
}
