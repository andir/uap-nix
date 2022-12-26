use rtnetlink::packet::nlas::link::Inet6DevConfBuffer;
use rtnetlink::{
    packet::{
        nlas::link::{AfSpecInet, Inet6, Inet6DevConf, Info, InfoBridge, InfoData, Nla, State},
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
    fn vlan_filtering(self, vlan_filtering: bool) -> Self;
}

fn info_nlas(request: &mut LinkAddRequest) -> Option<&mut Vec<Info>> {
    request
        .message_mut()
        .nlas
        .iter_mut()
        .find_map(|nla| match nla {
            Nla::Info(i) => Some(i),
            _ => None,
        })
}

fn info_data_bridge(infos: &mut Vec<Info>) -> Option<&mut Vec<InfoBridge>> {
    infos.iter_mut().find_map(|info| match info {
        Info::Data(InfoData::Bridge(info_bridge)) => Some(info_bridge),
        _ => None,
    })
}

fn info_bridge_vlan_filtering(info: &mut Vec<InfoBridge>) -> Option<&mut InfoBridge> {
    info.iter_mut().find_map(|i| match i {
        x @ InfoBridge::VlanFiltering(_) => Some(x),
        _ => None,
    })
}

impl LinkAddExt for LinkAddRequest {
    fn vlan_filtering(
        mut self,
        vlan_filtering: bool,
    ) -> Self {
        let x = InfoBridge::VlanFiltering(if vlan_filtering { 1 } else { 0 });
        if let Some(infos) = info_nlas(&mut self) {
            if let Some(info) = info_data_bridge(infos) {
                // search for an already existing vlan option
                if let Some(v) = info_bridge_vlan_filtering(info) {
                    *v = x;
                } else {
                    info.push(x);
                }
            } else {
                infos.push(Info::Data(InfoData::Bridge(vec![x])));
            }
        } else {
            self.message_mut()
                .nlas
                .push(Nla::Info(vec![Info::Data(InfoData::Bridge(vec![x]))]))
	}

        self
    }
}
