use crate::error::Error;
use crate::task::{RestartStrategy, Task};
use crate::util::AutoCloseFD;
use futures::TryStreamExt;

use log::{debug, error, info};

use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub enum ConfigOperState {
    Up,
    Down,
}

#[derive(Debug, Deserialize)]
pub struct NetworkConfiguration {
    interfaces: std::collections::HashMap<String, NetworkInterfaceConfiguration>,
}

#[derive(Debug, Deserialize)]
pub struct NetworkInterfaceConfiguration {
    oper_state: ConfigOperState,
}

#[derive(Debug, thiserror::Error)]
pub enum NetlinkError {
    #[error("Failed to send netlink request: {error:?}")]
    FailedToSendNetlinkRequest { error: std::io::Error },

    #[error("Failed to read from netlink socket: {error:?}")]
    FailedTOReadFromNetlinkSocket { error: std::io::Error },

    #[error("Failed to deserialize netlink message: {error:?}")]
    FailedToDeserialize {
        error: netlink_packet_core::DecodeError,
    },

    #[error("Response didn't fit in the provided buffer")]
    ResponseOverrun,

    #[error("Missing field {field} from netlink response")]
    MissingNetlinkResponseField { field: &'static str },

    #[error("Missing response for request")]
    MissingResponse,

    #[error("Request failed with errno {errno}: {message}")]
    Errno {
        errno: nc::Errno,
        message: &'static str,
    },
}

impl NetlinkError {
    fn from_errno(errno: nc::Errno) -> Self {
        Self::Errno {
            errno,
            message: nc::strerror(errno),
        }
    }
}

pub struct NetworkTask {
    child: crate::util::Child,
}

impl Task for NetworkTask {
    fn name<'a>(&'a self) -> &'a str {
        "network"
    }

    fn is_alive(&self) -> Result<bool, Error> {
        self.child.is_alive()
    }

    fn restart_strategy(&self) -> RestartStrategy {
        RestartStrategy::Reboot
    }

    fn restart(&mut self) -> Result<(), Error> {
        panic!("The network task can't be restarted");
    }

    fn poll_fd<'a>(&'a self) -> Option<(&'a AutoCloseFD, i16)> {
        Some((&self.child.fd, nc::POLLIN as i16))
    }
}

impl NetworkTask {
    pub fn new(configuration: NetworkConfiguration) -> Result<Self, Error> {
        let child = crate::util::fork_and_exec(move || NetworkTaskImpl::new(configuration).run())?;

        Ok(Self { child })
    }
}

#[derive(Debug)]
struct NetworkInterface {
    id: u32,
    name: String,
    oper_state: netlink_packet_route::rtnl::link::nlas::State,
    physical_port_name: Option<String>,
    physical_switch_name: Option<String>,
}

struct NetworkInterfaces(Vec<NetworkInterface>);

impl NetworkInterfaces {
    fn find_by_name<'a>(&'a self, name: impl AsRef<str>) -> Option<&'a NetworkInterface> {
        let name = name.as_ref();
        self.0.iter().find(|elem| elem.name == name)
    }
}

struct NetworkTaskImpl {
    configuration: NetworkConfiguration,
}

impl NetworkTaskImpl {
    fn new(configuration: NetworkConfiguration) -> Self {
        Self { configuration }
    }

    fn run(&mut self) {
	let rt = tokio::runtime::Runtime::new().expect("failed to launch tokio runtime");
	rt.block_on(self.run_async()).unwrap();
    }

    async fn run_async(&mut self) -> Result<(), std::io::Error> {

	let (_connection, handle, _rx) = rtnetlink::new_connection()?;

	loop {
	    error!("foo");
	    // for each of our configured interfaces try to reach the desired state
	    for (name, _iface) in self.configuration.interfaces.iter() {
		let mut f = handle.link().get().match_name(name.to_string()).execute();
		if let Some(link) = f.try_next().await.expect("Failed") {
		    info!("Interface {} found: {:?}", name, link);
		} else {
		    info!("Interfaces {} not found", name);
		}
	    }

	    tokio::time::sleep(std::time::Duration::from_secs(1)).await;
	}
	Ok(())
    }
}
