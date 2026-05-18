//!# log2
//!
//!`log2` is an out-of-the-box logging library for Rust. It writes to stdout or to file asynchronously, and automatically rotates based on file size.
//!
//!## Features
//!
//!- **stdout logging** - Log to console with color support
//!- **file logging** - Log to file with automatic rotation
//!- **log rotation** - Rotate logs based on file size (default: 100MB, 10 files)
//!- **tee support** - Log to both file and stdout simultaneously
//!- **module filtering** - Filter logs by module path
//!- **custom formatting** - Customize log output format
//!- **gzip compression** - Compress rotated log files
//!- **globally static** - No need to store the logger handle, lives for entire program duration
//!
//!## Add dependency
//!
//!```bash
//!cargo add log2
//!```
//!
//!## Quick Start
//!
//!### Log to stdout
//!
//!The `run!()` macro is the fastest way to get started. It starts logging to stdout
//!and automatically filters to only show logs from your own crate, hiding noise from
//!dependencies:
//!
//!```rust
//!use log2::*;
//!
//!fn main() {
//!run!();
//!
//!info!("hello world");
//!}
//!```
//!
//!This is equivalent to:
//!
//!```rust
//!log2::stdout()
//!.module_filter(|m: &str| {
//!let pkg = env!("CARGO_PKG_NAME");
//!m == pkg || m.starts_with(&format!("{}::", pkg))
//!})
//!.start();
//!```
//!
//!For full control, use the `log2::stdout()` builder:
//!
//!```rust
//!use log2::*;
//!
//!fn main() {
//!log2::stdout()
//!.level("info")
//!.module(true)
//!.start();
//!
//!info!("hello world");
//!}
//!```
//!
//!### Log to file
//!
//!```rust
//!use log2::*;
//!
//!fn main() {
//!log2::open("app.log").start();
//!
//!info!("hello world");
//!}
//!```
//!
//!## Configuration
//!
//!### stdout with options
//!
//!```rust
//!use log2::*;
//!
//!fn main() {
//!log2::stdout()
//!.level("info")           // set log level: trace, debug, info, warn, error, off
//!.module(false)           // hide module path
//!.module_with_line(true)  // show module path with line number
//!.start();
//!
//!trace!("verbose details");
//!debug!("debug info");
//!info!("general info");
//!warn!("warning message");
//!error!("error occurred");
//!}
//!```
//!
//!### file with options
//!
//!```rust
//!use log2::*;
//!
//!fn main() {
//!log2::open("app.log")
//!.size(100 * 1024 * 1024)  // max file size (default: 100MB)
//!.rotate(20)               // max rotation count (default: 10)
//!.tee(true)                // also log to stdout
//!.module(true)             // show module path (default: true)
//!.module_with_line(true)   // show module path with line number (default: false)
//!.level("debug")           // set log level
//!.compress(true)           // compress rotated files with gzip (default: false)
//!.start();
//!
//!info!("logging to file with rotation");
//!}
//!```
//!
//!## API Reference
//!
//!### Functions
//!
//!| Function | Description |
//!|----------|-------------|
//!| `log2::start()` | Start logging to stdout with default settings |
//!| `log2::stdout() -> Log2` | Create a stdout logger for configuration |
//!| `log2::open(path) -> Log2` | Create a file logger for configuration |
//!| `log2::set_level(level)` | Set global log level |
//!| `log2::handle() -> Option<RwLockWriteGuard>` | Get the global handle for manipulation |
//!| `log2::reset()` | Reset the global logger (useful for testing) |
//!
//!### Macros
//!
//!| Macro | Description |
//!|-------|-------------|
//!| `log2::run!()` | Start logging to stdout, filtered to the current package |
//!| `log2::app!()` | Get the package name from `Cargo.toml` |
//!
//!### Log2 Builder Methods
//!
//!| Method | Description | Default |
//!|--------|-------------|---------|
//!| `.level(name)` | Set log level | "trace" |
//!| `.module(show)` | Show/hide module path | true |
//!| `.module_with_line(show)` | Show module path with line number | false |
//!| `.tee(stdout)` | Also output to stdout | false |
//!| `.size(bytes)` | Max file size before rotation | 100MB |
//!| `.rotate(count)` | Number of rotated files to keep | 10 |
//!| `.compress(on)` | Compress rotated files with gzip | false |
//!| `.module_filter(fn)` | Filter logs by module path | none |
//!| `.format(fn)` | Custom log format function | built-in |
//!| `.start()` | Start the logger | - |
//!
//!### Log Levels
//!
//!- `trace` - Most verbose
//!- `debug` - Debug information
//!- `info` - General information (default)
//!- `warn` - Warning messages
//!- `error` - Error messages
//!- `off` - Disable all logging
//!
//!### Log Macros
//!
//!```rust
//!use log2::*;
//!
//!trace!("verbose: {}", value);
//!debug!("debug: {}", value);
//!info!("info: {}", value);
//!warn!("warning: {}", value);
//!error!("error: {}", value);
//!```
//!
//!## Handle API
//!
//!You can manipulate the logger after starting:
//!
//!```rust
//!use log2::*;
//!
//!fn main() {
//!log2::open("app.log").start();
//!
//!// Get handle to manipulate
//!if let Some(mut handle) = log2::handle() {
//!handle.flush();
//!// handle.redirect("new.log");
//!// handle.stop();
//!}
//!}
//!```
//!
//!### Handle Methods
//!
//!| Method | Description |
//!|--------|-------------|
//!| `stop()` | Stop the logger thread |
//!| `flush()` | Flush all pending logs |
//!| `redirect(path)` | Redirect log to a new file |
//!| `set_level(level)` | Change log level |
//!
//!## Module Filtering
//!
//!The `run!()` macro is the easiest way to filter logs to your own package:
//!
//!```rust
//!use log2::*;
//!
//!fn main() {
//!run!();
//!
//!my_crate::do_something();  // will be logged
//!other_crate::do_something(); // filtered out (not from your package)
//!}
//!```
//!
//!For custom filtering, use `.module_filter()`:
//!
//!```rust
//!use log2::*;
//!
//!fn main() {
//!log2::stdout()
//!.module_filter(|module| module.contains("my_app"))
//!.start();
//!
//!my_crate::do_something();  // will be logged
//!other_crate::do_something(); // will be filtered out
//!}
//!```
//!
//!## Custom Formatter
//!
//!Create a custom log format:
//!
//!```rust
//!use chrono::Local;
//!use log::Record;
//!use log2::*;
//!
//!fn custom_format(record: &Record, tee: bool) -> String {
//!if tee {
//!// stdout format (with colors)
//!format!(
//!"[{}] [{}] {}n",
//!Local::now().format("%Y-%m-%d %H:%M:%S"),
//!record.level(),
//!record.args()
//!)
//!} else {
//!// file format
//!format!(
//!"[{}] [{}] [{}] {}n",
//!Local::now().format("%Y-%m-%d %H:%M:%S"),
//!record.level(),
//!record.module_path().unwrap_or("unknown"),
//!record.args()
//!)
//!}
//!}
//!
//!fn main() {
//!log2::open("app.log")
//!.tee(true)
//!.format(custom_format)
//!.start();
//!
//!info!("custom formatted log");
//!}
//!```
//!
//!## File Rotation
//!
//!When file size reaches the limit, log2 rotates files:
//!
//!```
//!app.log      <- current log
//!app.1.log    <- most recent
//!app.2.log    <- older
//!...
//!app.9.log    <- oldest
//!```
//!
//!With compression enabled:
//!```
//!app.log
//!app.1.log.gz
//!app.2.log.gz
//!...
//!app.9.log.gz
//!```
//!
//!## Testing
//!
//!```rust
//!use log2::*;
//!
//!#[test]
//!fn test_logging() {
//!log2::stdout().start();
//!info!("test message");
//!log2::reset();  // clean up for next test
//!}
//!```
//!
//!## Dependencies
//!
//!Add these to your `Cargo.toml` if you use custom features:
//!
//!```toml
//!chrono = "0.4"   # for timestamp formatting
//!colored = "2"     # for colored output
//!flate2 = "1"      # for gzip compression
use chrono::Local;
use colored::*;
use core::fmt;
use flate2::write::GzEncoder;
use flate2::Compression;
use log::{LevelFilter, Metadata, Record};
use std::fs::File;
use std::io::{self, Read, Write};
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use std::sync::RwLock;
use std::thread::JoinHandle;

static HANDLE: RwLock<Option<Handle>> = RwLock::new(None);

/// log macros
pub use log::{debug, error, info, trace, warn};

/// Get the application name from Cargo.toml package name.
/// Expands to `env!("CARGO_PKG_NAME")`.
///
/// # Example
///
/// ```rust
/// use log2::*;
///
/// fn main() {
///     println!("Package name: {}", app!());
/// }
/// ```
#[macro_export]
macro_rules! app {
    () => {
        env!("CARGO_PKG_NAME")
    };
}

/// Start logging to stdout, filtered to logs from the current package only.
///
/// `run!()` is the quickest way to get started with `log2`. It configures the
/// logger to write to stdout and filters out all logs from dependencies, so
/// only records from your own crate are shown.
///
/// The filter uses `env!("CARGO_PKG_NAME")`, which resolves to the package name
/// of the crate that invokes the macro — not `log2` itself.
///
/// # Example
///
/// ```rust
/// use log2::*;
///
/// fn main() {
///     run!();
///
///     info!("this message is from your application and will be shown");
///
///     // logs from dependencies (e.g., `tokio`, `reqwest`) are filtered out
/// }
/// ```
///
/// For more control (file output, log levels, compression, etc.), use the
/// builder APIs [`stdout()`] and [`open()`] instead.
#[macro_export]
macro_rules! run {
    () => {
        $crate::stdout()
            .module_filter(|m: &str| {
                let pkg = env!("CARGO_PKG_NAME");
                m == pkg || m.starts_with(&format!("{}::", pkg))
            })
            .start();
    };
}

/// log levels
#[allow(non_camel_case_types)]
pub type level = LevelFilter;

fn get_level(level: &str) -> LevelFilter {
    let level = level.to_lowercase();
    match &*level {
        "debug" => level::Debug,
        "trace" => level::Trace,
        "info" => level::Info,
        "warn" => level::Warn,
        "error" => level::Error,
        "off" => level::Off,
        _ => level::Debug,
    }
}

/// set the log level, the input can be both enum or name
pub fn set_level<T: fmt::Display>(level: T) {
    log::set_max_level(get_level(&level.to_string()));
}

enum Action {
    Write(String),
    Tee(String),
    Flush,
    Exit,
    Redirect(String),
}

/// handle for manipulating log2
pub struct Handle {
    tx: std::sync::mpsc::Sender<Action>,
    thread: Option<JoinHandle<()>>,
    persistent: Arc<AtomicBool>, // log to file marker
}

pub struct Log2 {
    tx: std::sync::mpsc::Sender<Action>,
    rx: Option<std::sync::mpsc::Receiver<Action>>,
    levels: [ColoredString; 6],
    path: String,
    persistent: Arc<AtomicBool>, // log to file marker
    tee: bool,
    module: bool,
    line: bool,
    filesize: u64,
    count: usize,
    level: String,
    compression: bool,
    module_filter: Option<Box<dyn Fn(&str) -> bool + Send>>,
    formatter: Option<Box<dyn Fn(&Record, bool) -> String + Send>>,
}

struct Context {
    rx: std::sync::mpsc::Receiver<Action>,
    path: String,
    size: u64,
    count: usize,
    compression: bool,
}

impl Log2 {
    pub fn new() -> Self {
        let (tx, rx) = std::sync::mpsc::channel();
        let levels = [
            "OFF".black(),
            "ERROR".bright_red(),
            "WARN".yellow(),
            "INFO".green(),
            "DEBUG".bright_blue(),
            "TRACE".cyan(),
        ];
        Self {
            tx,
            rx: Some(rx),
            levels,
            path: String::new(),
            persistent: Arc::new(AtomicBool::new(false)),
            tee: false,
            module: true,
            line: true,
            filesize: 100 * 1024 * 1024,
            count: 10,
            level: String::new(),
            compression: false,
            module_filter: None,
            formatter: None,
        }
    }

    pub fn module(mut self, show: bool) -> Self {
        self.module = show;
        self.line = false;
        self
    }

    pub fn module_with_line(mut self, show: bool) -> Self {
        self.module = show;
        self.line = show;
        self
    }

    // split the output to stdout
    pub fn tee(mut self, stdout: bool) -> Self {
        self.tee = stdout;
        self
    }

    /// setup the maximum size for each file
    pub fn size(mut self, filesize: u64) -> Self {
        if self.count <= 1 {
            self.filesize = u64::MAX;
        } else {
            self.filesize = filesize;
        }
        self
    }

    /// setup the rotate count
    pub fn rotate(mut self, count: usize) -> Self {
        self.count = count;
        if self.count <= 1 {
            self.filesize = u64::MAX;
        }
        self
    }

    /// provide a way to filter by module name
    /// return true to include.
    pub fn module_filter(mut self, filter: impl Fn(&str) -> bool + Send + 'static) -> Self {
        self.module_filter = Some(Box::new(filter));
        self
    }

    /// custom content formatter (record:&Record, tee:bool)
    /// you can return different content for the tee flag, maybe colorful output.
    pub fn format<F: Fn(&Record, bool) -> String + Send + 'static>(mut self, formatter: F) -> Self {
        self.formatter = Some(Box::new(formatter));
        self
    }

    pub fn level<T: fmt::Display>(mut self, name: T) -> Self {
        self.level = name.to_string();
        self
    }

    /// start the log2 instance
    /// the logger lives for the entire program duration, no need to store the handle
    pub fn start(self) {
        let n = self.level.clone();
        let handle = start_log2(self);
        if !n.is_empty() {
            set_level(n);
        }
        *HANDLE.write().unwrap() = Some(handle);
    }

    /// enable compression for aged file
    pub fn compress(mut self, on: bool) -> Self {
        self.compression = on;
        self
    }
}

unsafe impl Sync for Log2 {}

impl log::Log for Log2 {
    fn enabled(&self, metadata: &Metadata) -> bool {
        // for macro: log_enabled!
        let n = get_level(&self.level);
        metadata.level() >= n
    }

    fn log(&self, record: &Record) {
        let module = record.module_path().unwrap_or("unknown");

        // module filter
        if let Some(filter) = &self.module_filter {
            if !filter(module) {
                return;
            }
        }

        // module
        let mut origin = String::new();
        if self.module {
            let mut marker = String::new();
            marker.push_str(module);
            if self.line {
                let num = record.line().map(|l| l.to_string()).unwrap_or_default();
                marker.push_str(&format!(":{}", num));
            }
            origin.push_str(&format!("[{}] ", marker));
        }

        // stdout
        if self.tee {
            let content;
            // custom formatter
            if let Some(format) = &self.formatter {
                content = format(record, true);
            } else {
                let level = &self.levels[record.level() as usize];
                let open = "[".truecolor(0x87, 0x87, 0x87);
                let close = "]".truecolor(0x87, 0x87, 0x87);
                content = format!(
                    "{open}{}{close} {open}{}{close} {origin}{}\n",
                    Local::now().format("%Y-%m-%d %H:%M:%S%.3f"),
                    level,
                    record.args()
                );
            }
            let _ = self.tx.send(Action::Tee(content));
        }

        // file
        if self.persistent.load(Ordering::SeqCst) {
            let content;
            // custom formatter
            if let Some(format) = &self.formatter {
                content = format(record, false);
            } else {
                content = format!(
                    "[{}] [{}] {origin}{}\n",
                    Local::now().format("%Y-%m-%d %H:%M:%S%.3f"),
                    record.level(),
                    record.args()
                );
            }
            let _ = self.tx.send(Action::Write(content));
        }
    }

    fn flush(&self) {
        let _ = self.tx.send(Action::Flush);
    }
}

impl Handle {
    pub fn stop(&mut self) {
        if let Some(thread) = self.thread.take() {
            let _ = self.tx.send(Action::Exit);
            let _ = thread.join();
        }
    }

    pub fn set_level<T: fmt::Display>(&self, level: T) {
        crate::set_level(level);
    }

    /// redirect the output file
    pub fn redirect(&mut self, path: &str) {
        // create directory
        let dir = std::path::Path::new(path);
        if let Some(dir) = dir.parent() {
            let _ = std::fs::create_dir_all(dir);
        }

        // check file, panic if error
        std::fs::OpenOptions::new()
            .create(true)
            .append(true)
            .open(path)
            .expect("error to open file");

        // update file marker, allow redirect stdout to file
        self.persistent.store(true, Ordering::SeqCst);

        // redirect log file
        let _ = self.tx.send(Action::Redirect(path.into()));
    }

    pub fn flush(&self) {
        let _ = self.tx.send(Action::Flush);
    }
}

impl Drop for Handle {
    fn drop(&mut self) {
        self.stop();
    }
}

fn rotate(ctx: &Context) -> Result<std::fs::File, std::io::Error> {
    let size = std::fs::metadata(&ctx.path)?.len();
    let dot = ctx.path.rfind(".").unwrap_or(0);
    let mut suffix = "";
    let mut prefix = &ctx.path[..];
    if dot > 0 {
        suffix = &ctx.path[dot..];
        prefix = &ctx.path[0..dot];
    }

    if size >= ctx.size {
        // maintain:
        // log.8.txt -> log.9.txt
        // log.7.txt -> log.8.txt
        // ...
        // log.txt   -> log.1.txt
        for i in (0..ctx.count - 1).rev() {
            let mut from = format!("{prefix}.{}{suffix}", i);
            if i == 0 {
                from = ctx.path.clone();
            }
            let to = format!("{prefix}.{}{suffix}", i + 1);
            maintain(ctx, &from, &to, i);
        }
    }

    let file = std::fs::OpenOptions::new()
        .create(true)
        .append(true)
        .open(&ctx.path)?;

    Ok(file)
}

fn maintain(ctx: &Context, from: &str, to: &str, index: usize) {
    if ctx.compression {
        // compress:
        // log.8.txt.gz -> log.9.txt.gz
        // log.7.txt.gz -> log.8.txt.gz
        // ...
        // log.txt      -> log.1.txt.gz
        if index == 0 {
            // log.txt -> log.1.txt.gz
            if compress_file(from, to).is_ok() {
                let _ = std::fs::remove_file(from);
            }
        } else {
            let from = format!("{}.gz", from);
            let to = format!("{}.gz", to);
            let _ = std::fs::rename(&from, &to);
        }
    } else {
        // rename:
        // log.8.txt -> log.9.txt
        // log.7.txt -> log.8.txt
        // ...
        // log.txt   -> log.1.txt
        let _ = std::fs::rename(from, to);
    }
}

fn compress_file(from: &str, to: &str) -> Result<(), io::Error> {
    let to = if to.ends_with(".gz") {
        to.to_string()
    } else {
        format!("{}.gz", to)
    };

    let mut input = File::open(from)?;
    let output = File::create(&to)?;
    let mut encoder = GzEncoder::new(output, Compression::default());
    let mut buffer = vec![0; 8192];

    loop {
        let bytes_read = input.read(&mut buffer)?;
        if bytes_read == 0 {
            break;
        }
        encoder.write_all(&buffer[0..bytes_read])?;
    }

    encoder.finish()?;

    Ok(())
}

fn now() -> u64 {
    std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap()
        .as_secs()
}

fn worker(mut ctx: Context) -> Result<(), std::io::Error> {
    let mut target: Option<std::fs::File> = None;
    let mut size: u64 = 0;
    let mut last = size;

    if !ctx.path.is_empty() {
        let file = rotate(&ctx)?;
        size = file.metadata()?.len();
        target = Some(file);
    }

    let timeout = std::time::Duration::from_secs(1);
    let mut ts = now();

    loop {
        if let Ok(action) = ctx.rx.recv_timeout(timeout) {
            match action {
                Action::Write(line) => {
                    let file = target.as_mut().unwrap();
                    let buf = line.as_bytes();
                    file.write_all(buf)?;
                    size += buf.len() as u64;
                    if size >= ctx.size {
                        drop(target);
                        let f = rotate(&ctx)?;
                        size = f.metadata()?.len();
                        target = Some(f);
                    }
                }
                Action::Tee(line) => {
                    print!("{line}");
                }
                Action::Flush => {
                    if let Some(file) = &mut target {
                        file.flush()?;
                    }
                }
                Action::Exit => {
                    if let Some(file) = &mut target {
                        file.flush()?;
                    }
                    break;
                }
                Action::Redirect(path) => {
                    ctx.path = path;
                    drop(target);
                    let file = rotate(&ctx)?;
                    size = file.metadata()?.len();
                    target = Some(file);
                }
            }
        }
        // flush every 1s
        if let Some(file) = &mut target {
            let n: u64 = now();
            if size > last && n - ts >= 1 {
                ts = n;
                file.flush()?;
                last = size;
            }
        }
    }

    Ok(())
}

/// start the log2 instance by default
/// the logger lives for the entire program duration, no need to store the handle
pub fn start() {
    let mut logger = Log2::new();
    logger.tee = true;
    let handle = start_log2(logger);
    *HANDLE.write().unwrap() = Some(handle);
}

/// get the global log2 handle for manipulation
pub fn handle() -> Option<std::sync::RwLockWriteGuard<'static, Option<Handle>>> {
    HANDLE.write().ok()
}

/// reset the global log2 handle (useful for testing)
pub fn reset() {
    *HANDLE.write().unwrap() = None;
}

/// create a log2 instance to stdout
pub fn stdout() -> Log2 {
    let mut logger = Log2::new();
    logger.tee = true;
    logger
}

/// log to file
pub fn open(path: &str) -> Log2 {
    // create directory
    let dir = std::path::Path::new(path);
    if let Some(dir) = dir.parent() {
        let _ = std::fs::create_dir_all(dir);
    }

    // check file, panic if error
    std::fs::OpenOptions::new()
        .create(true)
        .append(true)
        .open(path)
        .expect("error to open file");

    let mut logger = Log2::new();
    logger.path = path.to_string();
    logger.persistent = Arc::new(AtomicBool::new(true));
    logger
}

fn start_log2(mut logger: Log2) -> Handle {
    let rx = logger.rx.take().unwrap();

    let ctx = Context {
        rx,
        path: logger.path.clone(),
        size: logger.filesize,
        count: logger.count,
        compression: logger.compression,
    };

    let mut handle = Handle {
        tx: logger.tx.clone(),
        thread: None,
        persistent: logger.persistent.clone(),
    };

    let thread = std::thread::spawn(move || {
        if let Err(message) = worker(ctx) {
            println!("error: {message}");
        }
    });

    handle.thread = Some(thread);

    if log::set_boxed_logger(Box::new(logger)).is_ok() {
        log::set_max_level(LevelFilter::Trace);
    }

    handle
}
