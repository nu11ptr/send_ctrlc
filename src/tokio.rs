use std::io;

use crate::Interruptible;

use tokio::process::Child;

impl Interruptible for Child {
    fn pid(&mut self) -> io::Result<Option<u32>> {
        Ok(self.id())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_interruptible_command() {
        let mut command = tokio::process::Command::new("ping");
        #[cfg(windows)]
        command.arg("-t");
        command.arg("127.0.0.1");

        let mut child = command.spawn().unwrap();
        child.interrupt().unwrap();
        child.wait().await.unwrap();
    }

    #[tokio::test]
    async fn test_completed_interruptible_command() {
        let mut command = tokio::process::Command::new("ping");

        let mut child = command.spawn().unwrap();
        child.wait().await.unwrap();
        assert!(child.interrupt().is_err());
    }
}
