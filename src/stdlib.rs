use std::{
    io,
    ops::{Deref, DerefMut},
    process::{Child, Command},
};

use crate::{Interruptible, InterruptibleCommand};

#[cfg(windows)]
fn set_creation_flags(command: &mut Command) {
    use std::os::windows::process::CommandExt as _;
    command.creation_flags(crate::CREATE_NEW_PROCESS_GROUP);
}

impl InterruptibleCommand for Command {
    type Child = InterruptibleChild;

    fn spawn_interruptible(&mut self) -> io::Result<InterruptibleChild> {
        #[cfg(windows)]
        set_creation_flags(self);
        self.spawn().map(InterruptibleChild)
    }
}

/// A child process that can be interrupted
pub struct InterruptibleChild(Child);

impl Interruptible for InterruptibleChild {
    fn pid(&self) -> Option<u32> {
        Some(self.0.id())
    }
}

impl Deref for InterruptibleChild {
    type Target = Child;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for InterruptibleChild {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
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

        let mut child = command.spawn_interruptible().unwrap();
        child.interrupt().unwrap();
        child.wait().unwrap();
    }
}
