use std::ffi::OsStr;
use std::io;
use std::process::{Child, Command};

#[cfg(windows)]
const CREATE_NEW_PROCESS_GROUP: u32 = 0x00000200;

/// Trait for sending interrupts/ctrl-c to child processes
pub trait Interruptable: InterruptablePid {
    #[cfg(all(not(windows), not(unix)))]
    fn send_ctrl_c(&self) -> io::Result<()> {
        unimplemented!("Not implemented for this platform");
    }

    #[cfg(unix)]
    fn send_ctrl_c(&self) -> io::Result<()> {
        if unsafe { libc::kill(self.pid() as i32, libc::SIGINT) } == 0 {
            Ok(())
        } else {
            Err(io::Error::last_os_error())
        }
    }

    #[cfg(windows)]
    fn send_ctrl_c(&self) -> io::Result<()> {
        use windows_sys::Win32::System::Console::{CTRL_C_EVENT, GenerateConsoleCtrlEvent};

        unsafe {
            // NOTE: This only works if the process is in a new process group
            if GenerateConsoleCtrlEvent(CTRL_C_EVENT, self.pid()) == 0 {
                Err(io::Error::last_os_error())
            } else {
                Ok(())
            }
        }
    }
}

impl<T> Interruptable for T where T: InterruptablePid {}

/// Trait for getting the pid of a child process
pub trait InterruptablePid {
    fn pid(&self) -> u32;
}

impl InterruptablePid for Child {
    fn pid(&self) -> u32 {
        self.id()
    }
}

/// Create a new interruptable command
#[cfg(unix)]
pub fn new_interruptable_command<S: AsRef<OsStr>>(program: S) -> Command {
    Command::new(program)
}

/// Create a new interruptable command
#[cfg(windows)]
pub fn new_interruptable_command<S: AsRef<OsStr>>(program: S) -> Command {
    use std::os::windows::process::CommandExt as _;

    let mut command = Command::new(program);
    command.creation_flags(CREATE_NEW_PROCESS_GROUP);
    command
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_interruptable_command() {
        let mut command = new_interruptable_command("ping");
        #[cfg(windows)]
        command.arg("-t");
        command.arg("127.0.0.1");

        let mut child = command.spawn().unwrap();
        child.send_ctrl_c().unwrap();
        child.wait().unwrap();
    }
}
