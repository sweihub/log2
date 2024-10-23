use log2::*;

#[test]
fn module_filter() {
    let _log2 = Log2::new()
        .tee(true)
        // only log messages from modules that contain "first"
        .module_filter(|module| module.contains("first"))
        .start();

    first_module::foo();
    second_module::bar();

    mod first_module {
        use log2::*;

        pub fn foo() {
            trace!("send order request to server");
            debug!("receive order response");
            info!("order was executed");
            warn!("network speed is slow");
            error!("network connection was broken");
        }
    }

    mod second_module {
        use log2::*;

        pub fn bar() {
            trace!("send order request to server");
            debug!("receive order response");
            info!("order was executed");
            warn!("network speed is slow");
            error!("network connection was broken");
        }
    }
}
