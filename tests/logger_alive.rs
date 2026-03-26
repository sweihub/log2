use log2::*;

fn first_call() {
    info!("first call log message");
}

fn second_call() {
    info!("second call log message");
}

fn init() {
    log2::stdout().start();
}

#[test]
fn logger_alive_between_calls() {
    init();
    first_call();
    second_call();
}
