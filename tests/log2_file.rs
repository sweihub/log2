use log2::*;

#[test]
fn log_to_file() {
    let _log2 = log2::open("log.txt")
        .module(true)
        .tee(true)
        .start();
    trace!("send order request to server");
    debug!("receive order response");
    info!("order was executed");
    warn!("network speed is slow");
    error!("network connection was broken");
}
