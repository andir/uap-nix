/// Code that is required for early system init, e.g. mounting common paths
use crate::util::mount;

use crate::error::Error;

use log::{info, warn};

pub struct SystemInit {
    mount_dev: bool,
    mount_proc: bool,
    mount_sys: bool,
    mount_debug: bool,
    mount_run: bool,
    mount_tmp: bool,
}

impl Default for SystemInit {
    fn default() -> Self {
        SystemInit {
            mount_dev: true,
            mount_proc: true,
            mount_sys: true,
            mount_debug: true,
            mount_run: true,
            mount_tmp: true,
        }
    }
}

impl SystemInit {
    fn mount(&self, what: &str, point: &str, fstype: &str) -> Result<(), Error> {
        info!("💽 Mounting {}", point);
        match mount(what, point, fstype) {
            Ok(_) => {}
            Err(Error::FailedToMount {
                error: nc::EBUSY, ..
            }) => {
                warn!("The mountpoint is already occupied, failed to mount.");
            }
            Err(e) => return Err(e),
        }

        Ok(())
    }

    pub fn init(&self) -> Result<(), Error> {
        if self.mount_dev {
            self.mount("none", "/dev", "devtmpfs")?;
        }
        if self.mount_proc {
            self.mount("proc", "/proc", "proc")?;
        }
        if self.mount_sys {
            self.mount("sys", "/sys", "sysfs")?;
        }
        if self.mount_debug {
            self.mount("debug", "/sys/kernel/debug", "debugfs")?;
        }
        if self.mount_run {
            self.mount("tmpfs", "/run", "tmpfs")?;
        }
        if self.mount_tmp {
            self.mount("tmpfs", "/tmp", "tmpfs")?;
        }

        Ok(())
    }
}
