# log2

`log2` is an out-of-the-box logging library for Rust. It writes to stdout or to file asynchronousely, 
and automatically rotates based on file size.

# Usage

## Add dependency
```
cargo add log2
```

## Log to stdout

```rust
use log2:*;

fn main() {
    log2::start();

    trace!("send order request to server");
    debug!("receive order response");
    info!("order was executed");
    warn!("network speed is slow");
    error!("network connection as broken");
}
```

Output

![Screnshot of log2 output](images/output.png)

## Log to file

`log2` with default file size 50MB, max file count 10, you can change as you like. Note the `_log2` will 
stop the log2 when it is out the scope.

```rust
use log2::*;

fn main() {
    // configurable way: 
    // - log to file, file size: 100 MB, rotate: 20
    // - tee to stdout
    let _log2 = log2::open("log.txt")
                .size(100*1024*1024)
                .rotate(20)
                .tee(true)
                .start();

    // out-of-the-box way
    // let _log2 = log2::open("log.txt").start();

    trace!("send order request to server");
    debug!("receive order response");
    info!("order was executed");
    warn!("network speed is slow");
    error!("network connection as broken");
}

```

Output files

```
log.txt
log.1.txt
log.2.txt
log.3.txt
log.4.txt
log.5.txt
log.6.txt
log.7.txt
log.8.txt
log.9.txt
```
