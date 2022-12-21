use crate::error::Error;
use crate::util;

#[derive(Debug)]
pub enum RestartStrategy {
    Never,
    RestartProcess,
    Reboot,
}

impl std::fmt::Debug for dyn Task {
    fn fmt(&self, fmt: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(fmt, "Task {}", self.name())
    }
}

pub trait Task {
    fn name<'a>(&'a self) -> &'a str;
    fn is_alive(&self) -> Result<bool, Error>;
    fn restart_strategy(&self) -> RestartStrategy;
    fn restart(&mut self) -> Result<(), Error>;
    fn poll_fd<'a>(&'a self) -> Option<(&'a util::AutoCloseFD, i16)> {
        None
    }
}

pub struct ShellTask {
    child: util::Child,
}

impl ShellTask {
    pub fn new() -> Result<Self, Error> {
        Ok(Self {
            child: Self::spawn()?,
        })
    }

    fn spawn() -> Result<util::Child, Error> {
        util::spawn_child("/bin/sh", &["sh"])
    }

    fn restart(&mut self) -> Result<(), Error> {
        self.child = Self::spawn()?;
	Ok(())
    }
}

impl Task for ShellTask {
    fn name<'a>(&'a self) -> &'a str {
        "shell"
    }
    fn is_alive(&self) -> Result<bool, Error> {
        self.child.is_alive()
    }
    fn restart_strategy(&self) -> RestartStrategy {
        RestartStrategy::RestartProcess
    }

    fn poll_fd<'a>(&'a self) -> Option<(&'a util::AutoCloseFD, i16)> {
        Some((&self.child.fd, nc::POLLIN as i16))
    }

    fn restart(&mut self) -> Result<(), Error> {
        self.restart()
    }
}
