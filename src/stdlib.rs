use std::{io, process::Child};

use crate::Interruptible;

impl Interruptible for Child {
    fn pid(&mut self) -> io::Result<Option<u32>> {
        match self.try_wait() {
            Ok(Some(_)) => Ok(None),
            Ok(None) => Ok(Some(self.id())),
            Err(e) => Err(e),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_interruptible_command() {
        let mut command = std::process::Command::new("ping");
        #[cfg(windows)]
        command.arg("-t");
        command.arg("127.0.0.1");

        let mut child = command.spawn().unwrap();
        child.interrupt().unwrap();
        child.wait().unwrap();
    }

    #[test]
    fn test_completed_interruptible_command() {
        let mut command = std::process::Command::new("ping");

        let mut child = command.spawn().unwrap();
        child.wait().unwrap();
        assert!(child.interrupt().is_err());
    }
}
