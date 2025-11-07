#![cfg_attr(docsrs, feature(doc_cfg))]

//! A cross platform crate for sending ctrl-c to child processes

mod stdlib;
#[cfg(feature = "tokio")]
mod tokio;
#[doc = include_str!("../README.md")]
mod readme_tests {}

use std::io;

/// A trait for sending interrupts/ctrl-c to child processes.
///
/// NOTE: By implementing this trait, you are stating that the correct steps have been taken to
/// ensure that the child process can actually be interrupted.
pub trait Interruptible {
    /// Get the pid of the child process. It returns `Ok(Some(u32))` if the process is
    /// running and the PID is available. It returns `Ok(None)` if the process is already known
    /// to be completed. An error is returned if one occurs while attempting to get the pid.
    fn pid(&mut self) -> io::Result<Option<u32>>;

    /// Send a ctrl-c interrupt to the child process. It returns an error if the
    /// process is already known to be completed.
    fn interrupt(&mut self) -> io::Result<()> {
        match self.pid()? {
            Some(pid) => inner::interrupt(pid),
            None => Err(io::Error::other("Process is complete or has no pid")),
        }
    }
}

mod inner {
    use std::io;

    #[cfg(all(not(windows), not(unix)))]
    pub fn interrupt(_pid: u32) -> io::Result<()> {
        unimplemented!("Not implemented for this platform");
    }

    #[cfg(unix)]
    pub fn interrupt(pid: u32) -> io::Result<()> {
        // SAFETY: This is the standard POSIX kill function. Any number passed in is memory safe,
        // even if it impacts a process the user hadn't intended.
        if unsafe { libc::kill(pid as i32, libc::SIGINT) } == 0 {
            Ok(())
        } else {
            Err(io::Error::last_os_error())
        }
    }

    #[cfg(windows)]
    pub fn interrupt(pid: u32) -> io::Result<()> {
        use windows_sys::Win32::System::Console::{CTRL_BREAK_EVENT, GenerateConsoleCtrlEvent};

        // NOTE: After testing, it seems CTRL_BREAK_EVENT works in all cases (or all the ones I tried).
        // CTRL_C_EVENT didn't work for the `ctrlc` crate, for example, which is my primary use case.
        // NOTE 2: Despite what the docs say, and I found them confusing, it seems that no special
        // creation flags are actually needed anymore on Windows 10+. According to ChatGPT, this changed
        // around Windows 10+. I don't have anything older to test on to confirm or deny this.
        // SAFETY: This is a standard Windows console function. Any number passed in is memory safe,
        // even if it impacts a process the user hadn't intended.
        if unsafe { GenerateConsoleCtrlEvent(CTRL_BREAK_EVENT, pid) } != 0 {
            Ok(())
        } else {
            Err(io::Error::last_os_error())
        }
    }
}
