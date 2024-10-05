//!# log2
//!
//!`log2` is an out-of-the-box logging library for Rust. It writes to stdout or to file asynchronousely,
//!and automatically rotates based on file size.
//!
//!# Usage
//!
//!## Add dependency
//!```
//!cargo add log2
//!```
//!
//!## Log to stdout
//!
//!Simple to start.
//!
//!```rust
//!use log2::*;
//!
//!fn main() {
//!let _log2 = log2::start();
//!
//!trace!("send order request to server");
//!debug!("receive order response");
//!info!("order was executed");
//!warn!("network speed is slow");
//!error!("network connection was broken");
//!}
//!```
//!
//!Output
//!
//!![Screnshot of log2 output](images/output.png)
//!
//!Show module path, and set log level.
//!
//!```rust
//!use log2::*;
//!
//!fn main() {
//!let _log2 = log2::stdout()
//!.module(true)
//!.level("info")
//!.start();
//!
//!trace!("send order request to server");
//!debug!("receive order response");
//!info!("order was executed");
//!warn!("network speed is slow");
//!error!("network connection was broken");
//!}
//!
//!```
//!
//!## Log to file
//!
//!`log2` with default file size 100MB, max file count 10, you can change as you like. Note the `_log2` will
//!stop the log2 instance when it is out of the scope
//!
//!```rust
//!use log2::*;
//!
//!fn main() {
//!// configurable way:
//!// - log to file, file size: 100 MB, rotate: 20
//!// - tee to stdout
//!// - show module path
//!let _log2 = log2::open("log.txt")
//!.size(100*1024*1024)
//!.rotate(20)
//!.tee(true)
//!.module(true)
//!.start();
//!
//!// out-of-the-box way
//!// let _log2 = log2::open("log.txt").start();
//!
//!trace!("send order request to server");
//!debug!("receive order response");
//!info!("order was executed");
//!warn!("network speed is slow");
//!error!("network connection was broken");
//!}
//!
//!```
//!
//!Output files
//!
//!```
//!log.txt
//!log.1.txt
//!log.2.txt
//!log.3.txt
//!log.4.txt
//!log.5.txt
//!log.6.txt
//!log.7.txt
//!log.8.txt
//!log.9.txt
//!```
use chrono::Local;
use colored::*;
use core::fmt;
use log::{Level, LevelFilter, Metadata, Record};
use std::{io::Write, thread::JoinHandle};

/// log macros
pub use log::{debug, error, info, trace, warn};

/// log levels
#[allow(non_camel_case_types)]
pub type level = LevelFilter;

fn get_level(level: String) -> LevelFilter {
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
    log::set_max_level(get_level(level.to_string()));
}

enum Action {
    Write(String),
    Tee(String),
    Flush,
    Exit,
}

/// handle for terminating log2
pub struct Handle {
    tx: std::sync::mpsc::Sender<Action>,
    thread: Option<JoinHandle<()>>,
}

pub struct Log2 {
    tx: std::sync::mpsc::Sender<Action>,
    rx: Option<std::sync::mpsc::Receiver<Action>>,
    levels: [ColoredString; 6],
    path: String,
    tee: bool,
    module: bool,
    filesize: u64,
    count: usize,
    level: String,
}

struct Context {
    rx: std::sync::mpsc::Receiver<Action>,
    path: String,
    size: u64,
    count: usize,
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
            tee: false,
            module: true,
            filesize: 100 * 1024 * 1024,
            count: 10,
            level: String::new(),
        }
    }

    pub fn module(mut self, show: bool) -> Log2 {
        self.module = show;
        self
    }

    // split the output to stdout
    pub fn tee(mut self, stdout: bool) -> Log2 {
        self.tee = stdout;
        self
    }

    /// setup the maximum size for each file
    pub fn size(mut self, filesize: u64) -> Log2 {
        if self.count <= 1 {
            self.filesize = std::u64::MAX;
        } else {
            self.filesize = filesize;
        }
        self
    }

    /// setup the rotate count
    pub fn rotate(mut self, count: usize) -> Log2 {
        self.count = count;
        if self.count <= 1 {
            self.filesize = std::u64::MAX;
        }
        self
    }

    pub fn level<T: fmt::Display>(mut self, name: T) -> Self {
        self.level = name.to_string();
        self
    }

    /// start the log2 instance
    pub fn start(self) -> Handle {
        let n = self.level.clone();
        let handle = start_log2(self);
        if !n.is_empty() {
            set_level(n);
        }
        handle
    }
}

unsafe impl Sync for Log2 {}

impl log::Log for Log2 {
    fn enabled(&self, metadata: &Metadata) -> bool {
        // this seems no effect at all
        metadata.level() >= Level::Error
    }

    fn log(&self, record: &Record) {
        // cheap way to ignore other crates with absolute files (UNIX)
        // TODO: filter by crate/module name?
        let file = record.file().unwrap_or("unknown");
        if file.starts_with("/") {
            return;
        }

        // module
        let mut module = "".into();
        if self.module {
            module = format!("[{}]", record.module_path().unwrap_or(&file));
        }

        // stdout
        if self.tee {
            let level = &self.levels[record.level() as usize];
            let open = "[".truecolor(0x87, 0x87, 0x87);
            let close = "]".truecolor(0x87, 0x87, 0x87);
            let line = format!(
                "{open}{}{close} {open}{}{close} {module} {}",
                Local::now().format("%Y-%m-%d %H:%M:%S%.3f"),
                level,
                record.args()
            );
            let _ = self.tx.send(Action::Tee(line));
        }

        // file
        if self.path.len() > 0 {
            let line = format!(
                "[{}] [{}] {module}{}\n",
                Local::now().format("%Y-%m-%d %H:%M:%S%.3f"),
                record.level(),
                record.args()
            );
            let _ = self.tx.send(Action::Write(line));
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
        for i in (0..ctx.count - 1).rev() {
            let mut a = format!("{prefix}.{}{suffix}", i);
            if i == 0 {
                a = ctx.path.clone();
            }
            let b = format!("{prefix}.{}{suffix}", i + 1);
            let _ = std::fs::rename(&a, &b);
        }
    }

    let file = std::fs::OpenOptions::new()
        .create(true)
        .append(true)
        .open(&ctx.path)?;

    Ok(file)
}

fn now() -> u64 {
    std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap()
        .as_secs()
}

fn worker(ctx: Context) -> Result<(), std::io::Error> {
    let mut target: Option<std::fs::File> = None;
    let mut size: u64 = 0;
    let mut last = size;

    if ctx.path.len() > 0 {
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
                        let f = rotate(&ctx)?;
                        size = f.metadata()?.len();
                        target = Some(f);
                    }
                }
                Action::Tee(line) => {
                    println!("{line}");
                }
                Action::Flush => {
                    if target.is_some() {
                        let file = target.as_mut().unwrap();
                        file.flush()?;
                    }
                }
                Action::Exit => {
                    if target.is_some() {
                        let file = target.as_mut().unwrap();
                        file.flush()?;
                    }
                    break;
                }
            }
        }
        // flush every 1s
        if size > last && target.is_some() {
            let n = now();
            if n - ts >= 1 {
                ts = n;
                let file = target.as_mut().unwrap();
                file.flush()?;
                last = size;
            }
        }
    }

    Ok(())
}

/// start the log2 instance by default
pub fn start() -> Handle {
    let mut logger = Log2::new();
    logger.tee = true;
    start_log2(logger)
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
        let _ = std::fs::create_dir_all(&dir);
    }

    // check file, panic if error
    std::fs::OpenOptions::new()
        .create(true)
        .append(true)
        .open(path)
        .expect("error to open file");

    let mut logger = Log2::new();
    logger.path = path.into();
    logger
}

fn start_log2(mut logger: Log2) -> Handle {
    let rx = logger.rx.take().unwrap();

    let ctx = Context {
        rx,
        path: logger.path.clone(),
        size: logger.filesize,
        count: logger.count,
    };

    let mut handle = Handle {
        tx: logger.tx.clone(),
        thread: None,
    };

    let thread = std::thread::spawn(move || {
        if let Err(message) = worker(ctx) {
            println!("error: {message}");
        }
    });

    handle.thread = Some(thread);

    log::set_boxed_logger(Box::new(logger)).expect("error to initialize log2");
    log::set_max_level(LevelFilter::Trace);

    return handle;
}
