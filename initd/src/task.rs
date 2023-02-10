use crate::error::Error;
use crate::util;
use async_trait::async_trait;

#[derive(Debug, Clone, Copy)]
pub enum RestartStrategy {
    Never,
    RestartProcess,
    Reboot,
}

impl From<config::RestartStrategy> for RestartStrategy {
    fn from(other: config::RestartStrategy) -> Self {
	match other {
	    config::RestartStrategy::Never => Self::Never,
	    config::RestartStrategy::RestartProcess => Self::RestartProcess,
	    config::RestartStrategy::Reboot => Self::Reboot,
	}
    }
}

impl std::fmt::Debug for dyn Task {
    fn fmt(&self, fmt: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(fmt, "Task(name:{})", self.name())
    }
}

#[async_trait]
pub trait Task: Send {
    fn name<'a>(&'a self) -> &'a str;
    async fn is_alive(&mut self) -> Result<util::ProcessStatus, Error>;
    async fn wait(&mut self) -> Result<util::ProcessStatus, Error>;
    fn restart_strategy(&self) -> RestartStrategy;
    fn restart(&mut self) -> Result<(), Error>;
}

pub struct SubprocessTask {
    name: String,
    executable: String,
    arguments: Vec<String>,
    restart_strategy: RestartStrategy,
    child: util::Child,
    is_shell: bool,
}

impl SubprocessTask {
    pub fn new(executable: impl AsRef<str>, arguments: &[&str], restart_strategy: RestartStrategy, name: &str, shell: bool) -> Result<Self, Error> {
        Ok(Self {
            executable: executable.as_ref().to_owned(),
            arguments: arguments.iter().map(|s| String::from(*s)).collect(),
            child: Self::spawn(executable, arguments, shell)?,
	    restart_strategy,
	    name: name.to_string(),
            is_shell: shell,
        })
    }

    fn spawn<T: AsRef<str>>(
        command: impl AsRef<str>,
        arguments: &[T],
        is_shell: bool,
    ) -> Result<util::Child, Error> {
        util::async_spawn_child(
            command.as_ref(),
            &arguments.iter().map(|x| x.as_ref()).collect::<Vec<_>>(),
            is_shell,
        )
    }

    fn restart(&mut self) -> Result<(), Error> {
        self.child = Self::spawn(&self.executable, self.arguments.as_slice(), self.is_shell)?;
        Ok(())
    }
}

pub struct ShellTask;
impl ShellTask {
    pub fn new() -> Result<SubprocessTask, Error> {
	SubprocessTask::new("/bin/sh", &[], RestartStrategy::Reboot, "shell", true)
    }
}


#[async_trait]
impl Task for SubprocessTask {
    fn name<'a>(&'a self) -> &'a str {
        &self.name
    }

    #[inline]
    async fn is_alive(&mut self) -> Result<util::ProcessStatus, Error> {
        self.child.is_alive().await
    }

    #[inline]
    async fn wait(&mut self) -> Result<util::ProcessStatus, Error> {
        self.child.wait().await
    }

    fn restart_strategy(&self) -> RestartStrategy {
	self.restart_strategy
    }

    fn restart(&mut self) -> Result<(), Error> {
        self.restart()
    }
}
