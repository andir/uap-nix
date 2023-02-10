use serde::Deserialize;

#[derive(Debug, Deserialize)]
#[cfg_attr(feature = "schema-generator", derive(schemars::JsonSchema))]
pub struct Configuration {
    pub network: NetworkConfiguration,
    pub services: std::collections::BTreeMap<String, ServiceConfiguration>,
}

#[derive(Debug, Deserialize, Clone, Copy)]
#[cfg_attr(feature = "schema-generator", derive(schemars::JsonSchema))]
pub enum RestartStrategy {
    Never,
    RestartProcess,
    Reboot,
}


fn default_restart_strategy() -> RestartStrategy {
    RestartStrategy::Reboot
}

#[derive(Debug, Deserialize)]
#[cfg_attr(feature = "schema-generator", derive(schemars::JsonSchema))]
pub struct ServiceConfiguration {
    pub file: String,
    pub args: Vec<String>,
    #[serde(default="default_restart_strategy")]
    pub restart_strategy: RestartStrategy,
}

#[derive(Debug, Deserialize, Clone, Copy, PartialEq)]
#[cfg_attr(test, derive(serde::Serialize))]
#[cfg_attr(feature="schema-generator", derive(schemars::JsonSchema))]
pub enum ConfigOperState {
    Up,
    Down,
}

#[derive(Debug, Deserialize, PartialEq, Clone)]
#[cfg_attr(test, derive(serde::Serialize))]
#[cfg_attr(feature="schema-generator", derive(schemars::JsonSchema))]
pub struct NetworkConfiguration {
    pub interfaces: std::collections::HashMap<String, NetworkInterfaceConfiguration>,
}

fn true_value() -> bool {
    true
}

#[derive(Debug, Deserialize, PartialEq, Clone)]
#[cfg_attr(test, derive(serde::Serialize))]
#[cfg_attr(feature="schema-generator", derive(schemars::JsonSchema))]
pub struct NetworkInterfaceConfiguration {
    pub oper_state: ConfigOperState,
    #[serde(default="true_value")]
    pub accept_ra: bool,
    #[serde(default)]
    pub link_config: LinkConfig,
}

#[derive(Debug, Deserialize, PartialEq, Clone)]
#[cfg_attr(test, derive(serde::Serialize))]
#[cfg_attr(feature="schema-generator", derive(schemars::JsonSchema))]
pub enum LinkConfig {
    None,
    Bridge {
        #[serde(default)]
        vlan_filtering: bool,
    },
    BridgeMember {
        bridge_name: String,
    },
}

impl Default for LinkConfig {
    fn default() -> Self {
        LinkConfig::None
    }
}

pub fn load_config(config_file: &str) -> Result<Configuration, serde_json::Error> {
    if !std::path::Path::new(config_file).exists() {
        log::error!(
            "Missing configuration file at {}. Can't proceed.",
            config_file
        );
    }

    let fh = std::fs::File::open(config_file).expect("Failed to read config");
    serde_json::from_reader(&fh)
}

#[cfg(test)]
mod tests {
    use serde_json::{from_str as from_json, to_string as to_json};

    use super::*;

    macro_rules! map {
	($($key:expr, $val:expr),*) => {
	    {
		let map = [
		    $(
			($key.into(), $val),
		    )*
		].into_iter().collect::<std::collections::HashMap<_, _>>();
		map
	    }
	}
    }

    #[test]
    fn test_configuration_wan_up() {
        let expected = NetworkConfiguration {
            interfaces: map!(
                "wan".to_string(),
                NetworkInterfaceConfiguration {
                    oper_state: super::ConfigOperState::Up,
		    accept_ra: true,
                    link_config: super::LinkConfig::None,
                }
            ),
        };

        let config = r#"{"interfaces":{"wan": {"oper_state":"Up"}}}"#;
        let x: NetworkConfiguration = from_json(config).unwrap();
        assert_eq!(x, expected);
    }

    #[test]
    fn test_configuration_bridge_no_vlan_filtering_config() {
        let expected = NetworkConfiguration {
            interfaces: map!(
                "wan".to_string(),
                NetworkInterfaceConfiguration {
                    oper_state: super::ConfigOperState::Up,
		    accept_ra: true,
                    link_config: super::LinkConfig::Bridge {
                        vlan_filtering: false
                    },
                }
            ),
        };
        let config = r#"
{
  "interfaces": {
     "wan": {
       "oper_state": "Up",
       "link_config": {"Bridge": {}}
     }
   }
}
"#;

        let x: NetworkConfiguration = from_json(config).unwrap();
        assert_eq!(x, expected);
    }

    #[test]
    fn test_configuration_bridge_vlan_filtering_off() {
        let expected = NetworkConfiguration {
            interfaces: map!(
                "wan".to_string(),
                NetworkInterfaceConfiguration {
                    oper_state: super::ConfigOperState::Up,
		    accept_ra: true,
                    link_config: super::LinkConfig::Bridge {
                        vlan_filtering: false
                    },
                }
            ),
        };
        let config = r#"
{
  "interfaces": {
     "wan": {
       "oper_state": "Up",
       "link_config": {"Bridge": {"vlan_filtering": false}}
     }
   }
}
"#;

        let x: NetworkConfiguration = from_json(config).unwrap();
        assert_eq!(x, expected);
    }

    #[test]
    fn test_configuration_bridge_vlan_filtering_on() {
        let expected = NetworkConfiguration {
            interfaces: map!(
                "wan".to_string(),
                NetworkInterfaceConfiguration {
                    oper_state: super::ConfigOperState::Up,
		    accept_ra: true,
                    link_config: super::LinkConfig::Bridge {
                        vlan_filtering: true
                    },
                }
            ),
        };
        let config = r#"
{
  "interfaces": {
     "wan": {
       "oper_state": "Up",
       "link_config": {"Bridge": {"vlan_filtering": true}}
     }
   }
}
"#;

        let x: NetworkConfiguration = from_json(config).unwrap();
        assert_eq!(x, expected);
    }

    #[test]
    fn test_configuration_bridge_member() {
        let expected = NetworkConfiguration {
            interfaces: map!(
                "wan".to_string(),
                NetworkInterfaceConfiguration {
                    oper_state: super::ConfigOperState::Up,
		    accept_ra: true,
                    link_config: super::LinkConfig::BridgeMember {
                        bridge_name: "foo".to_string(),
                    }
                }
            ),
        };
        let config = r#"
{
  "interfaces": {
     "wan": {
       "oper_state": "Up",
       "link_config": {"BridgeMember": {"bridge_name": "foo"}}
     }
   }
}
"#;

        let x: NetworkConfiguration = from_json(config).unwrap();
        assert_eq!(x, expected);
    }
}

