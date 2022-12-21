use crate::error::Error;
use crate::task::{RestartStrategy, Task};
use crate::util::AutoCloseFD;
use log::{debug, error, info};
use netlink_packet_core::{NetlinkDeserializable, NetlinkSerializable};
use netlink_packet_route::{
    constants::*, LinkMessage, NeighbourMessage, NetlinkHeader, NetlinkMessage, NetlinkPayload,
    RtnlMessage,
};

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
    pub fn new() -> Result<Self, Error> {
        let child = crate::util::fork_and_exec(move || NetworkTaskImpl::new().run())?;

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

struct NetworkTaskImpl {}
impl NetworkTaskImpl {
    fn new() -> Self {
        Self {}
    }

    fn get_interfaces(
        socket: &mut netlink_sys::Socket,
    ) -> Result<Vec<NetworkInterface>, NetlinkError> {
        let mut lm = LinkMessage::default();
        lm.header.interface_family = netlink_packet_route::AF_PACKET as u8;
        let rm = RtnlMessage::GetLink(lm);
        let request = NetlinkRequest::new(NetlinkMessage {
            header: NetlinkHeader {
                flags: NLM_F_DUMP | NLM_F_REQUEST,
                ..Default::default()
            },
            payload: NetlinkPayload::from(rm),
        });
        let response = request.run(socket)?;

        // Filter for messages with content
        let responses = response
            .responses
            .into_iter()
            .filter_map(|r| match r.payload {
                NetlinkPayload::InnerMessage(RtnlMessage::NewLink(m)) => Some(m),
                NetlinkPayload::InnerMessage(m) => {
                    error!("Get different kind of message for GetLink request: {:?}", m);
                    None
                }
                _ => None,
            });

        let interfaces = responses
            .into_iter()
            .map(|r| {
                use netlink_packet_route::nlas::link::Nla;
                let id = r.header.index;
                let name = r
                    .nlas
                    .iter()
                    .find_map(|nla| match nla {
                        Nla::IfName(name) => Some(name.clone()),
                        _ => None,
                    })
                    .ok_or(NetlinkError::MissingNetlinkResponseField { field: "name" })?;

                let state = r
                    .nlas
                    .iter()
                    .find_map(|nla| match nla {
                        Nla::OperState(state) => Some(*state),
                        _ => None,
                    })
                    .ok_or(NetlinkError::MissingNetlinkResponseField {
                        field: "oper_state",
                    })?;

                let physical_port_name = r.nlas.iter().find_map(|nla| match nla {
                    Nla::PhysPortName(x) => Some(x.clone()),
                    _ => None,
                });

                let physical_switch_name = r.nlas.iter().find_map(|nla| match nla {
                    Nla::PhysPortName(x) => Some(x.clone()),
                    _ => None,
                });

                Ok(NetworkInterface {
                    id,
                    name,
                    oper_state: state,
                    physical_port_name,
                    physical_switch_name,
                })
            })
            .collect::<Result<Vec<_>, NetlinkError>>()?;

        Ok(interfaces)
    }

    fn run(&mut self) {
        let mut socket = match netlink_sys::Socket::new(netlink_sys::protocols::NETLINK_ROUTE) {
            Ok(o) => o,
            Err(e) => {
                error!("Failed to create netlink socket: {:?}", e);
                return;
            }
        };

        match socket.connect(&netlink_sys::SocketAddr::new(0, 0)) {
            Ok(_) => {}
            Err(e) => {
                error!("Failed to connect to netlink route socket: {:?}", e);
                return;
            }
        };

        loop {
            let interfaces =
                Self::get_interfaces(&mut socket).expect("Must be able to request interface data.");

            info!("interfaces: {:?}", interfaces);

            let request = NetlinkRequest::new(NetlinkMessage {
                header: NetlinkHeader {
                    flags: NLM_F_DUMP | NLM_F_REQUEST,
                    ..Default::default()
                },
                payload: NetlinkPayload::from(RtnlMessage::GetNeighbour(
                    NeighbourMessage::default(),
                )),
            });

            match request.run(&mut socket) {
                Err(e) => {
                    error!("Failed to request data from netlink: {:?}", e);
                    return;
                }
                Ok(response) => {
                    for message in response.responses.iter() {
                        debug!("Response: {:?}", message);
                    }
                }
            };

            std::thread::sleep(std::time::Duration::from_secs(90));
        }
    }
}

struct NetlinkRequest<I> {
    request: NetlinkMessage<I>,
}

struct NetlinkResponse<I> {
    responses: Vec<NetlinkMessage<I>>,
}

impl<I> NetlinkRequest<I>
where
    I: NetlinkSerializable + NetlinkDeserializable + std::fmt::Debug,
{
    pub fn new(request: NetlinkMessage<I>) -> Self {
        Self { request }
    }

    pub fn run(
        mut self,
        socket: &mut netlink_sys::Socket,
    ) -> Result<NetlinkResponse<I>, NetlinkError> {
        self.request.finalize();

        let mut send_buf = vec![0; self.request.header.length as usize];
        self.request.serialize(&mut send_buf[..]);
        socket
            .send(&send_buf[..], 0)
            .map_err(|error| NetlinkError::FailedToSendNetlinkRequest { error })?;
        drop(send_buf);

        let mut responses = vec![];
        let mut done = false;
        loop {
            if done {
                break;
            }
            debug!("Waiting for netlink packets");
            let (receive_buffer, _) = socket
                .recv_from_full()
                .map_err(|error| NetlinkError::FailedTOReadFromNetlinkSocket { error })?;

            // deserialize all the potential messages
            let mut offset = 0;
            loop {
                let bytes = &receive_buffer[offset..];
                let msg: NetlinkMessage<I> = NetlinkMessage::deserialize(bytes)
                    .map_err(|error| NetlinkError::FailedToDeserialize { error })?;

                match msg.payload {
                    NetlinkPayload::Overrun(_) => {
                        return Err(NetlinkError::ResponseOverrun);
                    }
                    NetlinkPayload::Done => {
                        debug!("done with netlink responses");
                        done = true;
                        break;
                    }
                    _ => {}
                }

                let msg_hdr_length = msg.header.length as usize;
                offset += msg_hdr_length;

                responses.push(msg);

                if offset >= receive_buffer.len() || msg_hdr_length == 0 {
                    break;
                }
            }
        }

        Ok(NetlinkResponse { responses })
    }
}
