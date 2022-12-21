use crate::error::Error;

use log::{debug, error, info};

pub struct Child {
    pub fd: AutoCloseFD,
    pub pid: i32,
}

impl Child {
    pub fn is_alive(&self) -> Result<bool, Error> {
        let mut status = 0;
        let mut usage = nc::rusage_t::default();
        match unsafe { nc::wait4(self.pid, &mut status, nc::WNOHANG, &mut usage) } {
            Ok(0) => {
                debug!("PID {} is still alive and kicking", self.pid);
                return Ok(true);
            }
            Ok(n) if n == self.pid => {
                info!("PID {} died.", self.pid);
                return Ok(false);
            }
            Ok(n) => {
                error!(
                    "PID {} returned {} not sure what this means?!? Considering the process dead.",
                    self.pid, n
                );
                return Ok(false);
            }
            Err(e) => {
                return Err(Error::from_errno_with_message(
                    e,
                    "Failed to wait4 for child",
                ));
            }
        }
    }
}

/// Spawn a shell as a subprocess that inherits the TTY
pub fn spawn_child(cmd: &str, args: &[&str]) -> Result<Child, Error> {
    debug!("Forking new child: {} {:?}", cmd, args);
    let f = move || {
        debug!("calling execve {} {:?}", cmd, args);
        execve(cmd, args, &[]).expect("Child must execute");
    };
    fork_and_exec(f)
}

/// Spawn a shell as a subprocess that inherits the TTY
pub fn fork_and_exec(f: impl FnOnce() -> ()) -> Result<Child, Error> {
    let pid = unsafe { nc::fork() }
        .map_err(|errno| Error::from_errno_with_message(errno, "failed to fork"))?;
    if pid == 0 {
        info!("New child alive and kicking. 🎉");
        // set a new process group id
        unsafe { nc::setpgid(0, 0) }.expect("Seting the pgid should work");

        f();
        unreachable!();
    }

    let pidfd = unsafe { nc::pidfd_open(pid, 0) }
        .map_err(|errno| Error::from_errno_with_message(errno, "Failed to create pidfd"))?;

    Ok(Child {
        pid,
        fd: AutoCloseFD::from(pidfd),
    })
}

pub fn resolve_symlink(path: &str) -> Result<String, Error> {
    const BUFFER_SIZE: usize = 512;
    let mut output = [0u8; BUFFER_SIZE];
    let _ = unsafe {
        nc::readlink(path, &mut output, BUFFER_SIZE)
            .map_err(|errno| Error::from_errno_with_message(errno, "Failed to readlink"))
    }?;

    String::from_utf8(output.to_vec())
        .map_err(|e| Error::FailedToConvertReadlinkResultToString { error: e })
}

pub fn execve(prog: &str, args: &[&str], env: &[&str]) -> Result<(), Error> {
    debug!("Resolving symlink for {}", prog);
    let prog = resolve_symlink(prog)?;
    debug!("Resolved symlink to {}", prog);

    let prog_ptr = prog.as_ptr() as usize;

    // translate all the arguments and environment things into
    // properly owned and NULL terminated arrays
    let arguments = args
        .iter()
        .map(|arg| std::ffi::CString::new(*arg))
        .collect::<Result<Vec<_>, _>>()
        .map_err(|_| Error::FailedToConvertStringToCString)?;

    let argument_ptrs = arguments
        .iter()
        .map(|arg| arg.as_ptr() as usize)
        .chain(std::iter::once(0))
        .collect::<Vec<_>>();
    let arguments_ptr_slices = &argument_ptrs[..];

    let env = env
        .iter()
        .map(|e| std::ffi::CString::new(*e))
        .collect::<Result<Vec<_>, _>>()
        .map_err(|_| Error::FailedToConvertStringToCString)?;

    let env_ptrs = env
        .iter()
        .map(|e| e.as_ptr() as usize)
        .chain(std::iter::once(0))
        .collect::<Vec<_>>();
    let env_ptr_slices = &env_ptrs[..];

    unsafe {
        nc::syscalls::syscall3(
            nc::SYS_EXECVE,
            prog_ptr,
            arguments_ptr_slices.as_ptr() as usize,
            env_ptr_slices.as_ptr() as usize,
        )
        .map_err(|errno| Error::from_errno_with_message(errno, "failed to call execve"))?;
    }

    panic!("Execve shouldn't continue executing the calling code");
    unreachable!()
}

pub fn getpid() -> nc::pid_t {
    unsafe { nc::getpid() }
}

pub fn stat(dir: &str) -> Result<nc::stat_t, Error> {
    let mut stat = nc::stat_t::default();
    unsafe { nc::stat(dir, &mut stat) }
        .map_err(|errno| Error::from_errno_with_message(errno, "failed to stat"))?;
    Ok(stat)
}

#[inline]
pub fn is_dir(mode: nc::mode_t) -> bool {
    (mode & nc::S_IFMT) == nc::S_IFDIR
}

pub fn create_missing_dir(dir: &str) -> Result<(), Error> {
    match stat(dir) {
        Err(Error::Errno {
            errno,
            message,
        })
        | Err(Error::ErrnoWithMessage { errno, message, .. }) => {
            unsafe { nc::mkdir(dir, 0o755) }.map_err(|error| Error::FailedToCreateDirectory {
                message: "Tried to create missing directory",
                directory: dir.to_string(),
                errno,
                error_message: message,
            })?
        }
        Err(e) => Err(e)?,
        Ok(stat) => {
            if !is_dir(stat.st_mode) {
                return Err(Error::DirectoryAlreadyExistsAsFile {
                    message: "The directory you tried to create already exists as a file",
                    directory: dir.to_string(),
                });
            } else {
                // nothing to do
            }
        }
    }

    Ok(())
}

pub fn mount(what: &str, point: &str, fs_type: &str) -> Result<(), Error> {
    create_missing_dir(point)?;
    unsafe { nc::mount(what, point, fs_type, 0, 0) }.map_err(|e| Error::FailedToMount {
        directory: point.to_string(),
        error: e,
    })?;

    Ok(())
}

pub fn sleep_secs(seconds: nc::time_t) {
    let ts = nc::timespec_t {
        tv_sec: seconds,
        tv_nsec: 0,
    };
    unsafe {
        nc::nanosleep(&ts, None).expect("Sleep must work");
    }
}

pub fn reboot() -> ! {
    unsafe {
        nc::reboot(
            nc::LINUX_REBOOT_MAGIC1,
            nc::LINUX_REBOOT_MAGIC2,
            nc::LINUX_REBOOT_CMD_RESTART,
            0,
        )
        .expect("reboot must succeed");
    };

    loop {
        panic!("unreachable code in reboot function");
    }
}

pub struct AutoCloseFD {
    fd: i32,
}

impl AutoCloseFD {
    pub fn new(fd: i32) -> Self {
        Self { fd }
    }

    pub fn close(&mut self) {
        debug!("Trying to close {}", self.fd);
        if self.fd < 0 {
            return;
        }
        debug!("closing FD {}", self.fd);
        match unsafe { nc::close(self.fd) } {
            Ok(_) => {}
            Err(e) => {
                println!("Failed to close FD: {}", e);
            }
        };
        self.fd = -1;
    }
}

pub trait FD {
    fn get(&self) -> i32;
}

impl std::cmp::PartialEq for AutoCloseFD {
    fn eq(&self, other: &AutoCloseFD) -> bool {
        let o = other.get();
        self.get().eq(&o)
    }
}

impl FD for AutoCloseFD {
    fn get(&self) -> i32 {
        self.fd
    }
}

impl From<i32> for AutoCloseFD {
    fn from(other: i32) -> Self {
        Self::new(other)
    }
}

impl std::ops::Drop for AutoCloseFD {
    fn drop(&mut self) {
        self.close();
    }
}

pub fn pipe() -> Result<(AutoCloseFD, AutoCloseFD), Error> {
    let mut fds = [-1_i32; 2];
    unsafe { nc::pipe2(&mut fds, nc::O_CLOEXEC) }
        .map_err(|errno| Error::from_errno_with_message(errno, "failed to call pipe2"))?;
    debug!("Created to new fds for the pipe: {:?}", fds);

    let (a, b) = (fds[0], fds[1]);

    if a == -1 || b == -1 {
        return Err(Error::PipeCreationFailed);
    }

    Ok((AutoCloseFD::from(fds[0]), AutoCloseFD::from(fds[1])))
}

#[cfg(test)]
mod tests {

    #[test]
    fn test_pipe() {
        let (a, b) = super::pipe().unwrap();
        assert!(a.fd != -1);
        assert!(b.fd != -1);
    }

    #[test]
    fn test_execve() {
        //        super::execve("/bin/sh", &["/bin/sh"], &[]).unwrap()
    }
}
