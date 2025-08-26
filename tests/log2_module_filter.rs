const PATH: &str = "tests/log_filter.txt";

#[test]
fn module_filter() {
    let _log2 = log2::open(PATH)
        .tee(true)
        // only log messages from modules that contain "first"
        .module_filter(|module| module.contains("first"))
        .start();

    first_module::foo();
    second_module::bar();

    mod first_module {
        use log2::*;

        pub fn foo() {
            trace!("first: send order request to server");
            debug!("first: receive order response");
            info!("first: order was executed");
            warn!("first: network speed is slow");
            error!("first: network connection was broken");
        }
    }

    mod second_module {
        use log2::*;

        pub fn bar() {
            trace!("second: send order request to server");
            debug!("second: receive order response");
            info!("second: order was executed");
            warn!("second: network speed is slow");
            error!("second: network connection was broken");
        }
    }

    // Read the file to verify its content
    let log_content = std::fs::read_to_string(PATH).expect("Failed to read the log file");
    assert!(
        !log_content.contains("second:"),
        "Log module filter did not work"
    );
}
