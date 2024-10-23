use log2::*;

#[test]
fn redirect_log_file() {
    let mut log2 = log2::open("log2.txt").module(true).tee(true).start();

    trace!("send order request to server");
    debug!("receive order response");
    info!("order was executed");
    warn!("network speed is slow");
    error!("network connection was broken");

    log2.redirect("redirect_log.txt");

    trace!("send order request to server");
    debug!("receive order response");
    info!("order was executed");
    warn!("network speed is slow");
    error!("network connection was broken");

    // Check if the file was created
    let file_path = std::path::Path::new("redirect_log.txt");
    assert!(file_path.exists(), "redirect_log.txt file was not created");

    // Read the file to verify its content
    let log_content = std::fs::read_to_string(file_path).expect("Failed to read the log file");
    assert!(
        log_content.contains("network connection was broken"),
        "Log content does not match"
    );
}
