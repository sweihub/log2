use chrono::Local;
use colored::*;
use log::{Level, LevelFilter, Metadata, Record};
use std::{io::Write, thread::JoinHandle};

/// log macros
pub use log::{debug, error, info, trace, warn};

/// log levels
#[allow(non_camel_case_types)]
pub type level = LevelFilter;

/// set the log level
pub fn set_level(level: level) {
    log::set_max_level(level);
}

enum Action {
    Write(String),
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
    filesize: u64,
    count: usize,
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
            filesize: 50 * 1024 * 1024,
            count: 10,
        }
    }

    pub fn tee(mut self, stdout: bool) -> Log2 {
        self.tee = stdout;
        self
    }

    pub fn size(mut self, filesize: u64) -> Log2 {
        self.filesize = filesize;
        self
    }

    pub fn rotate(mut self, count: usize) -> Log2 {
        self.count = count;
        self
    }

    pub fn start(self) -> Handle {
        start_log2(self)
    }
}

unsafe impl Sync for Log2 {}

impl log::Log for Log2 {
    fn enabled(&self, metadata: &Metadata) -> bool {
        metadata.level() >= Level::Debug
    }

    fn log(&self, record: &Record) {
        let level = &self.levels[record.level() as usize];
        if self.tee {
            println!(
                "{}{}{} [{}] {}",
                "[".truecolor(0x9a, 0x9a, 0x9a),
                Local::now().format("%Y-%m-%d %H:%M:%S%.3f"),
                "]".truecolor(0x9a, 0x9a, 0x9a),
                level,
                record.args()
            );
        }
        if self.path.len() > 0 {
            let line = format!(
                "[{}] [{}] {}\n",
                Local::now().format("%Y-%m-%d %H:%M:%S%.3f"),
                record.level(),
                record.args()
            );
            let _ = self.tx.send(Action::Write(line));
        }
    }

    fn flush(&self) {
        println!("flush now");
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
}

impl Drop for Handle {
    fn drop(&mut self) {
        self.stop();
    }
}

fn rotate(ctx: &Context) -> Result<std::fs::File, std::io::Error> {
    // log.txt, log.1.txt, log.2.txt, ..., log.9.txt

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
    let mut file = rotate(&ctx)?;
    let mut size = file.metadata()?.len();
    let timeout = std::time::Duration::from_secs(1);
    let mut ts = now();
    let mut last = size;

    loop {
        if let Ok(action) = ctx.rx.recv_timeout(timeout) {
            match action {
                Action::Write(line) => {
                    let buf = line.as_bytes();
                    file.write_all(buf)?;
                    size += buf.len() as u64;
                    if size >= ctx.size {
                        drop(file);
                        file = rotate(&ctx)?;
                        size = file.metadata()?.len();
                    }
                }
                Action::Flush => {
                    file.flush()?;
                }
                Action::Exit => {
                    file.flush()?;
                    break;
                }
            }
        }
        // flush every 1s
        if size > last {
            let n = now();
            if n - ts >= 1 {
                ts = n;
                file.flush()?;
                last = size;
            }
        }
    }

    Ok(())
}

/// start the log2 instance
pub fn start() -> Handle {
    let mut logger = Log2::new();
    logger.tee = true;
    start_log2(logger)
}

/// log to file
pub fn open(path: &str) -> Log2 {
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

    if ctx.path.len() > 0 {
        let thread = std::thread::spawn(move || {
            if let Err(message) = worker(ctx) {
                println!("error: {message}");
            }
        });
        handle.thread = Some(thread);
    }

    log::set_boxed_logger(Box::new(logger)).expect("error to initialize log2");
    log::set_max_level(LevelFilter::Trace);

    return handle;
}
