use log2::*;

fn main() {
    let _log2 = log2::stdout().module(true).start();
    trace!("send order request to server");
    debug!("receive order response");
    info!("order was executed");
    warn!("network speed is slow");
    error!("network connection was broken");
}
