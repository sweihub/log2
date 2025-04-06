use chrono::Local;
use colored::Colorize;
use log::Record;
use log2::*;

fn custom(record: &Record, tee: bool) -> String {
    let content;
    let module = record.module_path().unwrap_or("unknown");
    let line = record.line().map(|l| l.to_string()).unwrap_or_default();
    let origin = format!("[{}:{}]", module, line);

    if tee {
        // stdout with colors
        let levels = [
            "OFF".black(),
            "ERROR".bright_red(),
            "WARN".yellow(),
            "INFO".green(),
            "DEBUG".bright_blue(),
            "TRACE".cyan(),
        ];
        let level = &levels[record.level() as usize];
        let open = "[".truecolor(0x87, 0x87, 0x87);
        let close = "]".truecolor(0x87, 0x87, 0x87);
        content = format!(
            "CUSTOM {open}{}{close} {open}{}{close} {origin}{}\n",
            Local::now().format("%Y-%m-%d %H:%M:%S%.3f"),
            level,
            record.args()
        );
    } else {
        // file
        content = format!(
            "CUSTOM [{}] [{}] {origin}{}\n",
            Local::now().format("%Y-%m-%d %H:%M:%S%.3f"),
            record.level(),
            record.args()
        );
    }

    return content;
}

fn main() {
    let _log2 = log2::open("custom.txt")
        .tee(true)
        .level("trace")
        .module(false)
        .module_with_line(true)
        .module_filter(|m| !m.is_empty())
        .format(custom)
        .start();

    trace!("send order request to server");
    debug!("receive order response");
    info!("order was executed");
    warn!("network speed is slow");
    error!("network connection was broken");
}
