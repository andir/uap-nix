pub mod config;
mod netlink;

pub use self::config::{ConfigOperState, LinkConfig, NetworkConfiguration};
use self::netlink::{LinkAddExt, LinkExt};

use crate::task::{RestartStrategy, Task};
use crate::util::AutoCloseFD;
use futures::TryStreamExt;
use log::{debug, error, info};
use rtnetlink::packet::LinkMessage;

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("IO: {0:?}")]
    IO(#[from] std::io::Error),

    #[error("Netlink: {0:?}")]
    Netlink(#[from] rtnetlink::Error),

    #[error("Netlink encoding: {0:?}")]
    NetlinEncoding(#[from] rtnetlink::packet::DecodeError),
}

pub struct NetworkTask {
    child: crate::util::Child,
    configuration: NetworkConfiguration,
}

impl Task for NetworkTask {
    fn name<'a>(&'a self) -> &'a str {
        "network"
    }

    fn is_alive(&self) -> Result<bool, crate::error::Error> {
        self.child.is_alive()
    }

    fn restart_strategy(&self) -> RestartStrategy {
        RestartStrategy::RestartProcess
    }

    fn restart(&mut self) -> Result<(), crate::error::Error> {
        self.child = self.exec()?;
	Ok(())
    }

    fn poll_fd<'a>(&'a self) -> Option<(&'a AutoCloseFD, i16)> {
        Some((&self.child.fd, nc::POLLIN as i16))
    }
}

impl NetworkTask {
    pub fn new(configuration: NetworkConfiguration) -> Result<Self, crate::error::Error> {
        let mut s = Self {
            child: crate::util::Child::default(),
            configuration,
        };
        s.child = s.exec()?;
        Ok(s)
    }

    fn exec(&self) -> Result<crate::util::Child, crate::error::Error> {
        crate::util::fork_and_exec(move || NetworkTaskImpl::new(self.configuration.clone()).run())
    }
}

struct NetworkTaskImpl {
    configuration: NetworkConfiguration,
    interfaces: std::collections::HashMap<String, LinkMessage>,
}

impl NetworkTaskImpl {
    fn new(configuration: NetworkConfiguration) -> Self {
        Self {
            configuration,
            interfaces: std::collections::HashMap::new(),
        }
    }

    fn run(&mut self) {
        let rt = tokio::runtime::Runtime::new().expect("failed to launch tokio runtime");
        rt.block_on(async {
            if let Err(e) = self.run_async().await {
                panic!("network task died: {:?}", e);
            }
        });
    }

    async fn run_async(&mut self) -> Result<(), Error> {
        let (connection, handle, _rx) = rtnetlink::new_connection()?;

        tokio::spawn(connection);

        loop {
            // for each of our configured interfaces try to reach the desired state
            let mut interfaces = std::collections::HashMap::new();
            let mut missing_interfaces = vec![];
            for (name, iface) in self.configuration.interfaces.iter() {
                let mut f = handle.link().get().match_name(name.to_string()).execute();
                match f.try_next().await {
                    Ok(Some(link)) => {
                        info!("Interface {} found: {:?}", name, link);
                        // FIXME: reduce memory requirements by only getting the required fields
                        interfaces.insert(name.clone(), link.clone());
                        self.ensure_oper_state(handle.clone(), name, iface.oper_state, &link)
                            .await?;
			self.ensure_accept_ra(handle.clone(), name, iface.accept_ra, &link)
    .await?;
                    }
                    Ok(None) => {
                        if !missing_interfaces.contains(name) {
                            missing_interfaces.push(name.clone());
                        }
                    }
                    Err(e) => match e {
                        rtnetlink::Error::NetlinkError(msg) => {
                            if msg.code == -nc::ENODEV {
                                if !missing_interfaces.contains(name) {
                                    missing_interfaces.push(name.clone());
                                }
                            }
                        }
                        x => {
                            error!("Failed to search for interface {}: {:?}", name, x);
                        }
                    },
                }
            }

            self.interfaces = interfaces;

            // Now determine if we know what to do to create the missing interfaces
            for name in missing_interfaces {
                info!("Interface {} not found", name);
                let iface = self.configuration.interfaces.get(&name);
                let iface = if let Some(iface) = iface {
                    iface
                } else {
                    error!("Interface vanished from configuration?!?");
                    continue;
                };

                match &iface.link_config {
                    LinkConfig::None => {}
                    LinkConfig::Bridge { vlan_filtering } => {
                        info!(
                            "Creating bridge {} with vlan_filtering={}",
                            name, vlan_filtering
                        );
                        self.create_bridge(handle.clone(), name, *vlan_filtering)
                            .await?;
                    }
                    LinkConfig::BridgeMember { bridge_name } => {
                        self.join_bridge(handle.clone(), bridge_name, &name).await?;
                    }
                }
            }

            tokio::time::sleep(std::time::Duration::from_secs(90)).await;
        }
        unreachable!();
        Ok(())
    }

    async fn join_bridge(
        &self,
        handle: rtnetlink::Handle,
        bridge_name: &str,
        name: &str,
    ) -> Result<(), Error> {
        let bridge_interface = self.interfaces.get(bridge_name);
        if !bridge_interface
            .map(|iface| iface.header.interface_family == rtnetlink::packet::AF_BRIDGE as u8)
            .unwrap_or(false)
        {
            error!(
                "Interface {} isn't a bridge. Can't add a member",
                bridge_name
            );
            return Ok(());
        }

        // get the master link id
        let bridge_id = match bridge_interface.map(|iface| iface.header.index) {
            Some(m) => m,
            None => {
                error!("Bridge interface isn't known yet");
                return Ok(());
            }
        };

        let interface = self.interfaces.get(name);
        let interface_id = match interface.map(|iface| iface.header.index) {
            Some(i) => i,
            None => {
                error!("Interface {} not found", name);
                return Ok(());
            }
        };

        handle
            .link()
            .set(interface_id)
            .master(bridge_id)
            .execute()
            .await?;

        Ok(())
    }

    async fn create_bridge(
        &self,
        handle: rtnetlink::Handle,
        name: String,
        vlan_filtering: bool,
    ) -> Result<(), Error> {
        let msg = handle
            .link()
            .add()
            .bridge(name)
            .vlan_filtering(vlan_filtering)?;
        msg.execute().await?;
        Ok(())
    }

    async fn ensure_accept_ra(&self,
			      handle: rtnetlink::Handle,
			      name: &str,
			      accept_ra: bool,
			      link: &LinkMessage) -> Result<(), Error> {

	Ok(())
    }

    async fn ensure_oper_state(
        &self,
        handle: rtnetlink::Handle,
        name: &str,
        oper_state: ConfigOperState,
        link: &LinkMessage,
    ) -> Result<(), Error> {
        match (oper_state, link.is_link_up()) {
            (ConfigOperState::Up, true) => {}
            (ConfigOperState::Up, _) => {
                debug!("Setting operstate to UP for {}", name);
                handle.link().set(link.header.index).up().execute().await?;
            }
            (ConfigOperState::Down, true) => {
                debug!("Setting operstate to DOWN for {}", name);
                handle
                    .link()
                    .set(link.header.index)
                    .down()
                    .execute()
                    .await?;
            }
            _ => {}
        };

        Ok(())
    }
}
