use crate::error::Error;
use crate::util;

#[derive(Debug, Clone, Copy)]
pub enum RestartStrategy {
    Never,
    RestartProcess,
    Reboot,
}

impl std::fmt::Debug for dyn Task {
    fn fmt(&self, fmt: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(fmt, "Task(name:{})", self.name())
    }
}

pub trait Task {
    fn name<'a>(&'a self) -> &'a str;
    fn is_alive(&self) -> Result<util::ProcessStatus, Error>;
    fn restart_strategy(&self) -> RestartStrategy;
    fn restart(&mut self) -> Result<(), Error>;
    fn poll_fd<'a>(&'a self) -> Option<(&'a util::AutoCloseFD, i16)> {
        None
    }
}

pub struct SubprocessTask {
    name: String,
    executable: String,
    arguments: Vec<String>,
    restart_strategy: RestartStrategy,
    child: util::Child,
}

impl SubprocessTask {
    pub fn new(executable: impl AsRef<str>, arguments: &[&str], restart_strategy: RestartStrategy, name: &str) -> Result<Self, Error> {
        Ok(Self {
            executable: executable.as_ref().to_owned(),
            arguments: arguments.iter().map(|s| String::from(*s)).collect(),
            child: Self::spawn(executable, arguments)?,
	    restart_strategy,
	    name: name.to_string(),
        })
    }

    fn spawn<T: AsRef<str>>(
        command: impl AsRef<str>,
        arguments: &[T],
    ) -> Result<util::Child, Error> {
        util::spawn_child(
            command.as_ref(),
            &arguments.iter().map(|x| x.as_ref()).collect::<Vec<_>>(),
        )
    }

    fn restart(&mut self) -> Result<(), Error> {
        self.child = Self::spawn(&self.executable, self.arguments.as_slice())?;
        Ok(())
    }
}

pub struct ShellTask {
    task: SubprocessTask,
}

impl ShellTask {
    pub fn new() -> Result<SubprocessTask, Error> {
	SubprocessTask::new("/bin/sh", &["sh"], RestartStrategy::Reboot, "shell")
    }
}

impl Task for ShellTask {
    fn name<'a>(&'a self) -> &'a str {
        self.task.name()
    }

    fn is_alive(&self) -> Result<util::ProcessStatus, Error> {
	self.task.is_alive()
    }

    fn restart_strategy(&self) -> RestartStrategy {
	self.task.restart_strategy()
    }

    fn poll_fd<'a>(&'a self) -> Option<(&'a util::AutoCloseFD, i16)> {
	self.task.poll_fd()
    }

    fn restart(&mut self) -> Result<(), Error> {
	self.task.restart()
    }
}


impl Task for SubprocessTask {
    fn name<'a>(&'a self) -> &'a str {
        &self.name
    }

    fn is_alive(&self) -> Result<util::ProcessStatus, Error> {
        self.child.is_alive()
    }

    fn restart_strategy(&self) -> RestartStrategy {
	self.restart_strategy
    }

    fn poll_fd<'a>(&'a self) -> Option<(&'a util::AutoCloseFD, i16)> {
        Some((&self.child.fd, nc::POLLIN as i16))
    }

    fn restart(&mut self) -> Result<(), Error> {
        self.restart()
    }
}
