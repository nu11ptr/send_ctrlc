# send_ctrlc

[![Crate](https://img.shields.io/crates/v/send_ctrlc)](https://crates.io/crates/send_ctrlc)
[![Docs](https://docs.rs/send_ctrlc/badge.svg)](https://docs.rs/send_ctrlc)
[![Build](https://github.com/nu11ptr/send_ctrlc/workflows/CI/badge.svg)](https://github.com/nu11ptr/send_ctrlc/actions)
[![codecov](https://codecov.io/github/nu11ptr/send_ctrlc/graph/badge.svg?token=3M5tvBewE5)](https://codecov.io/github/nu11ptr/send_ctrlc)

A cross platform crate for sending ctrl-c to child processes

## Features

* Cross platform (including Windows)
* Uniform cross platform API
* Both sync and async
* Minimal dependencies: 
    * Synchronous: `libc` on unix, and `windows-sys` on windows
    * Asynchronous: `tokio` (with only `process` feature)

## Examples

```rust
use send_ctrlc::{Interruptable as _};

// Synchronous...

#[cfg(not(feature = "tokio"))]
fn main() {
        // Start a continuous ping using our special command constructor function
        let mut command = send_ctrlc::new_command("ping");
        #[cfg(windows)]
        command.arg("-t");
        command.arg("127.0.0.1");

        // Spawn the ping, interrupt it, and wait for it to complete
        let mut child = command.spawn().unwrap();
        child.send_ctrl_c().unwrap();
        child.wait().unwrap();
}

// or asynchronous...

#[cfg(feature = "tokio")]
#[tokio::main]
async fn main() {
        // Start a continuous ping using our special command constructor function
        let mut command = send_ctrlc::new_tokio_command("ping");
        #[cfg(windows)]
        command.arg("-t");
        command.arg("127.0.0.1");

        // Spawn the ping, interrupt it, and wait for it to complete
        let mut child = command.spawn().unwrap();
        child.send_ctrl_c().unwrap();
        child.wait().await.unwrap();
}

```


## Contributions

Contributions are welcome as long they align with the vision for this crate.
