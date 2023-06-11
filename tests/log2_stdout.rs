use log::*;

#[test]
fn test_terminal() {
    log2::start();
    log2::set_level(log2::level::Trace);

    trace!("trace line here");
    debug!("debug line");
    info!("hello log");
    warn!("warning");
    error!("oops");

    std::thread::sleep(std::time::Duration::from_secs(10));
}
