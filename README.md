# log2

`log2` is an out-of-the-box logging library for Rust. It writes to stdout or to file asynchronously, and automatically rotates based on file size.

## Features

- **stdout logging** - Log to console with color support
- **file logging** - Log to file with automatic rotation
- **log rotation** - Rotate logs based on file size (default: 100MB, 10 files)
- **tee support** - Log to both file and stdout simultaneously
- **module filtering** - Filter logs by module path
- **custom formatting** - Customize log output format
- **gzip compression** - Compress rotated log files
- **globally static** - No need to store the logger handle, lives for entire program duration

## Add dependency

```bash
cargo add log2
```

## Quick Start

### Log to stdout

The `run!()` macro is the fastest way to get started. It starts logging to stdout
and automatically filters to only show logs from your own crate, hiding noise from
dependencies:

```rust
use log2::*;

fn main() {
    run!();

    info!("hello world");
}
```

This is equivalent to:

```rust
log2::stdout()
    .module_filter(|m: &str| {
        let pkg = env!("CARGO_PKG_NAME");
        m == pkg || m.starts_with(&format!("{}::", pkg))
    })
    .start();
```

For full control, use the `log2::stdout()` builder:

```rust
use log2::*;

fn main() {
    log2::stdout()
        .level("info")
        .module(true)
        .start();

    info!("hello world");
}
```

### Log to file

```rust
use log2::*;

fn main() {
    log2::open("app.log").start();

    info!("hello world");
}
```

## Configuration

### stdout with options

```rust
use log2::*;

fn main() {
    log2::stdout()
        .level("info")           // set log level: trace, debug, info, warn, error, off
        .module(false)           // hide module path
        .module_with_line(true)  // show module path with line number
        .start();

    trace!("verbose details");
    debug!("debug info");
    info!("general info");
    warn!("warning message");
    error!("error occurred");
}
```

### file with options

```rust
use log2::*;

fn main() {
    log2::open("app.log")
        .size(100 * 1024 * 1024)  // max file size (default: 100MB)
        .rotate(20)               // max rotation count (default: 10)
        .tee(true)                // also log to stdout
        .module(true)             // show module path (default: true)
        .module_with_line(true)   // show module path with line number (default: false)
        .level("debug")           // set log level
        .compress(true)           // compress rotated files with gzip (default: false)
        .start();

    info!("logging to file with rotation");
}
```

## API Reference

### Functions

| Function | Description |
|----------|-------------|
| `log2::start()` | Start logging to stdout with default settings |
| `log2::stdout() -> Log2` | Create a stdout logger for configuration |
| `log2::open(path) -> Log2` | Create a file logger for configuration |
| `log2::set_level(level)` | Set global log level |
| `log2::handle() -> Option<RwLockWriteGuard>` | Get the global handle for manipulation |
| `log2::reset()` | Reset the global logger (useful for testing) |

### Macros

| Macro | Description |
|-------|-------------|
| `log2::run!()` | Start logging to stdout, filtered to the current package |
| `log2::app!()` | Get the package name from `Cargo.toml` |

### Log2 Builder Methods

| Method | Description | Default |
|--------|-------------|---------|
| `.level(name)` | Set log level | "trace" |
| `.module(show)` | Show/hide module path | true |
| `.module_with_line(show)` | Show module path with line number | false |
| `.tee(stdout)` | Also output to stdout | false |
| `.size(bytes)` | Max file size before rotation | 100MB |
| `.rotate(count)` | Number of rotated files to keep | 10 |
| `.compress(on)` | Compress rotated files with gzip | false |
| `.module_filter(fn)` | Filter logs by module path | none |
| `.format(fn)` | Custom log format function | built-in |
| `.start()` | Start the logger | - |

### Log Levels

- `trace` - Most verbose
- `debug` - Debug information
- `info` - General information (default)
- `warn` - Warning messages
- `error` - Error messages
- `off` - Disable all logging

### Log Macros

```rust
use log2::*;

trace!("verbose: {}", value);
debug!("debug: {}", value);
info!("info: {}", value);
warn!("warning: {}", value);
error!("error: {}", value);
```

## Handle API

You can manipulate the logger after starting:

```rust
use log2::*;

fn main() {
    log2::open("app.log").start();

    // Get handle to manipulate
    if let Some(mut handle) = log2::handle() {
        handle.flush();
        // handle.redirect("new.log");
        // handle.stop();
    }
}
```

### Handle Methods

| Method | Description |
|--------|-------------|
| `stop()` | Stop the logger thread |
| `flush()` | Flush all pending logs |
| `redirect(path)` | Redirect log to a new file |
| `set_level(level)` | Change log level |

## Module Filtering

The `run!()` macro is the easiest way to filter logs to your own package:

```rust
use log2::*;

fn main() {
    run!();

    my_crate::do_something();  // will be logged
    other_crate::do_something(); // filtered out (not from your package)
}
```

For custom filtering, use `.module_filter()`:

```rust
use log2::*;

fn main() {
    log2::stdout()
        .module_filter(|module| module.contains("my_app"))
        .start();

    my_crate::do_something();  // will be logged
    other_crate::do_something(); // will be filtered out
}
```

## Custom Formatter

Create a custom log format:

```rust
use chrono::Local;
use log::Record;
use log2::*;

fn custom_format(record: &Record, tee: bool) -> String {
    if tee {
        // stdout format (with colors)
        format!(
            "[{}] [{}] {}\n",
            Local::now().format("%Y-%m-%d %H:%M:%S"),
            record.level(),
            record.args()
        )
    } else {
        // file format
        format!(
            "[{}] [{}] [{}] {}\n",
            Local::now().format("%Y-%m-%d %H:%M:%S"),
            record.level(),
            record.module_path().unwrap_or("unknown"),
            record.args()
        )
    }
}

fn main() {
    log2::open("app.log")
        .tee(true)
        .format(custom_format)
        .start();

    info!("custom formatted log");
}
```

## File Rotation

When file size reaches the limit, log2 rotates files:

```
app.log      <- current log
app.1.log    <- most recent
app.2.log    <- older
...
app.9.log    <- oldest
```

With compression enabled:
```
app.log
app.1.log.gz
app.2.log.gz
...
app.9.log.gz
```

## Testing

```rust
use log2::*;

#[test]
fn test_logging() {
    log2::stdout().start();
    info!("test message");
    log2::reset();  // clean up for next test
}
```

## Dependencies

Add these to your `Cargo.toml` if you use custom features:

```toml
chrono = "0.4"   # for timestamp formatting
colored = "2"     # for colored output
flate2 = "1"      # for gzip compression
```