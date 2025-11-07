#![cfg_attr(docsrs, feature(doc_cfg))]

//! A cross platform crate for sending ctrl-c to child processes

#[doc = include_str!("../README.md")]
mod readme_tests {}

use std::ffi::OsStr;
use std::io;
use std::process::{Child, Command};

#[cfg(windows)]
const CREATE_NEW_PROCESS_GROUP: u32 = 0x00000200;

/// Trait for sending interrupts/ctrl-c to child processes
pub trait Interruptable: InterruptablePid {
    /// Send a ctrl-c interrupt to the child process
    #[cfg(all(not(windows), not(unix)))]
    fn send_ctrl_c(&self) -> io::Result<()> {
        unimplemented!("Not implemented for this platform");
    }

    /// Send a ctrl-c interrupt to the child process
    #[cfg(unix)]
    fn send_ctrl_c(&self) -> io::Result<()> {
        if let Some(pid) = self.pid() {
            if unsafe { libc::kill(pid as i32, libc::SIGINT) } == 0 {
                Ok(())
            } else {
                Err(io::Error::last_os_error())
            }
        } else {
            Err(io::Error::new(io::ErrorKind::Other, "Process has no pid"))
        }
    }

    /// Send a ctrl-c interrupt to the child process
    #[cfg(windows)]
    fn send_ctrl_c(&self) -> io::Result<()> {
        use windows_sys::Win32::System::Console::{CTRL_C_EVENT, GenerateConsoleCtrlEvent};
        if let Some(pid) = self.pid() {
            // NOTE: This only works if the process is in a new process group
            if unsafe { GenerateConsoleCtrlEvent(CTRL_C_EVENT, pid) } != 0 {
                Ok(())
            } else {
                Err(io::Error::last_os_error())
            }
        } else {
            Err(io::Error::new(io::ErrorKind::Other, "Process has no pid"))
        }
    }
}

impl<T> Interruptable for T where T: InterruptablePid {}

/// Trait for getting the pid of a child process
pub trait InterruptablePid {
    /// Get the pid of the child process
    fn pid(&self) -> Option<u32>;
}

impl InterruptablePid for Child {
    fn pid(&self) -> Option<u32> {
        Some(self.id())
    }
}

#[cfg(feature = "tokio")]
impl InterruptablePid for tokio::process::Child {
    fn pid(&self) -> Option<u32> {
        self.id()
    }
}

/// Create a new interruptable command
pub fn new_command<S: AsRef<OsStr>>(program: S) -> Command {
    inner::new_command(program)
}

mod inner {
    use std::{ffi::OsStr, process::Command};

    #[cfg(windows)]
    pub fn new_command<S: AsRef<OsStr>>(program: S) -> Command {
        use std::os::windows::process::CommandExt as _;

        let mut command = Command::new(program);
        command.creation_flags(crate::CREATE_NEW_PROCESS_GROUP);
        command
    }

    #[cfg(unix)]
    pub fn new_command<S: AsRef<OsStr>>(program: S) -> Command {
        use std::process::Command;

        Command::new(program)
    }
}

/// Create a new interruptable tokio command
#[cfg_attr(docsrs, doc(cfg(feature = "tokio")))]
#[cfg(feature = "tokio")]
pub fn new_tokio_command<S: AsRef<OsStr>>(program: S) -> tokio::process::Command {
    inner_tokio::new_tokio_command(program)
}

#[cfg(feature = "tokio")]
mod inner_tokio {
    use std::ffi::OsStr;
    use tokio::process::Command;

    #[cfg(windows)]
    pub fn new_tokio_command<S: AsRef<OsStr>>(program: S) -> Command {
        let mut command = Command::new(program);
        command.creation_flags(crate::CREATE_NEW_PROCESS_GROUP);
        command
    }

    #[cfg(unix)]
    pub fn new_tokio_command<S: AsRef<OsStr>>(program: S) -> Command {
        Command::new(program)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_interruptable_command() {
        let mut command = new_command("ping");
        #[cfg(windows)]
        command.arg("-t");
        command.arg("127.0.0.1");

        let mut child = command.spawn().unwrap();
        child.send_ctrl_c().unwrap();
        child.wait().unwrap();
    }

    #[cfg(feature = "tokio")]
    #[tokio::test]
    async fn test_tokio_interruptable_command() {
        let mut command = new_tokio_command("ping");
        #[cfg(windows)]
        command.arg("-t");
        command.arg("127.0.0.1");

        let mut child = command.spawn().unwrap();
        child.send_ctrl_c().unwrap();
        child.wait().await.unwrap();
    }
}
