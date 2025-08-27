use log2::*;
use std::time::Duration;

const REDIRECT: &str = "tests/redirect_stdout.txt";

#[test]
fn redirect_stdout_file() {
    let mut log2 = log2::start();

    trace!("send order request to server");
    debug!("receive order response");
    info!("order was executed");
    warn!("network speed is slow");
    error!("network connection was broken");

    log2.redirect(REDIRECT);

    trace!("file: send order request to server");
    debug!("file: receive order response");
    info!("file: order was executed");
    warn!("file: network speed is slow");
    error!("file: network connection was broken");

    // data should be flushed
    log2.flush();
    std::thread::sleep(Duration::from_millis(1000));

    // Check if the file was created
    let file_path = std::path::Path::new(REDIRECT);
    assert!(file_path.exists(), "{REDIRECT} file was not created");

    // Read the file to verify its content
    let log_content = std::fs::read_to_string(file_path).expect("Failed to read the log file");
    assert!(
        log_content.contains("file: network connection was broken"),
        "Log content does not match"
    );
}
