use log2::*;

const PATH: &str = "tests/log.txt";

#[test]
fn log_to_file() {
    let _log2 = log2::open(PATH).module(true).tee(true).start();
    trace!("send order request to server");
    debug!("receive order response");
    info!("order was executed");
    warn!("network speed is slow");
    error!("network connection was broken");
}
