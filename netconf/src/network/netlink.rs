use rtnetlink::packet::nlas::link::Inet6DevConfBuffer;
use rtnetlink::packet::nlas::NlaBuffer;
use rtnetlink::{
    packet::{
        nlas::link::{AfSpecInet, Inet6, Inet6DevConf, InfoBridge, Nla, State},
        LinkMessage,
    },
    LinkAddRequest,
};

pub trait LinkExt {
    fn get_oper_state(&self) -> Option<State>;
    fn get_af_spec_inet(&self) -> Option<&[AfSpecInet]>;
    fn is_link_up(&self) -> bool;
    fn name(&self) -> Option<&str>;
    fn accept_ra(&self) -> Option<bool>;
}

impl LinkExt for LinkMessage {
    fn get_oper_state(&self) -> Option<State> {
        self.nlas.iter().find_map(|nla| match nla {
            Nla::OperState(x) => Some(*x),
            _ => None,
        })
    }

    fn is_link_up(&self) -> bool {
        self.get_oper_state()
            .map(|s| s == State::Up)
            .unwrap_or(false)
    }

    fn name(&self) -> Option<&str> {
        self.nlas.iter().find_map(|nla| match nla {
            Nla::IfName(n) => Some(n.as_ref()),
            _ => None,
        })
    }
    fn get_af_spec_inet(&self) -> Option<&[AfSpecInet]> {
        self.nlas.iter().find_map(|nla| match nla {
            Nla::AfSpecInet(xs) => Some(xs.as_slice()),
            _ => None,
        })
    }

    fn accept_ra(&self) -> Option<bool> {
        let afspec_inet = self.get_af_spec_inet()?;
        let inet6s = afspec_inet.iter().find_map(|item| match item {
            AfSpecInet::Inet6(x) => Some(x),
            _ => None,
        })?;
        let devconf = inet6s.iter().find_map(|item| match item {
            Inet6::DevConf(devconf) => Some(devconf),
            _ => None,
        })?;

        let buffer = Inet6DevConfBuffer::new_checked(devconf)
            .map(Some)
            .unwrap_or(None)?;

	use rtnetlink::packet::traits::Parseable;
        let inet6devconf = Inet6DevConf::parse(&buffer).map(Some).unwrap_or(None)?;
	Some(inet6devconf.accept_ra > 0)
    }
}

pub trait LinkAddExt: Sized {
    fn vlan_filtering(self, vlan_filtering: bool) -> Result<Self, rtnetlink::packet::DecodeError>;
}

impl LinkAddExt for LinkAddRequest {
    fn vlan_filtering(
        mut self,
        vlan_filtering: bool,
    ) -> Result<Self, rtnetlink::packet::DecodeError> {
        use rtnetlink::packet::nlas::DefaultNla;
        use rtnetlink::packet::traits::{Emitable, Parseable};

        let mut buffer = vec![0u8; 32];
        let x = InfoBridge::VlanFiltering(1);
        x.emit(&mut buffer);
        let r = NlaBuffer::new_checked(&buffer)?;
        let default_nla = DefaultNla::from(DefaultNla::parse(&r)?);

        self.message_mut().nlas.push(Nla::Other(default_nla));

        Ok(self)
    }
}
