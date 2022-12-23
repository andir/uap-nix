use serde::Deserialize;

use clap::Parser;

mod neighbours;

mod early;
mod epoll;
mod error;
mod network;
mod task;
mod util;

use util::{reboot, sleep_secs, AutoCloseFD, FD};

use log::{debug, error, info};

#[derive(Parser, Clone, Copy)]
pub enum Command {
    Init,
    VerifyConfig,
    Network,

    #[cfg(feature="schema-generator")]
    GenerateSchema,
}

#[derive(Parser)]
pub struct Arguments  {
    #[clap(short, long, default_value="/config.json")]
    config_file: String,

    #[clap(subcommand)]
    command: Command,
}

fn init_logging() {
    simplelog::CombinedLogger::init(vec![simplelog::TermLogger::new(
        log::LevelFilter::Debug,
        simplelog::Config::default(),
        simplelog::TerminalMode::Mixed,
        simplelog::ColorChoice::Auto,
    )])
    .expect("Must be able to initialize the logging");
}

fn load_config(config_file: &str) -> Result<Configuration, serde_json::Error> {
    info!("⌛ Parsing configuration ⌛");
    if !util::stat(config_file).is_ok() {
        error!("Missing configuration file at {}. Can't proceed.", config_file);
    }

    let fh = std::fs::File::open(config_file).expect("Failed to read config");
    serde_json::from_reader(&fh)
}

fn main() {
    let pid = util::getpid();
    init_logging();

    let args = Arguments::parse();
    match args.command {
        Command::Init => {
            if let Err(e) = run(&args.config_file) {
                error!("System failed: {:?}", e);
            }

            if pid == 1 {
                error!("No more work to watch out for. Rebooting in 15s");
                sleep_secs(15);
                reboot();
            }
        },
	Command::Network => {

	},
	Command::VerifyConfig => {
	    load_config(&args.config_file).unwrap();
	}
	#[cfg(feature="schema-generator")]
	Command::GenerateSchema => {
	    let schema = schemars::schema_for!(Configuration);
	    println!("{}", serde_json::to_string_pretty(&schema).unwrap());
	}
    };
}

#[derive(Debug, Deserialize)]
#[cfg_attr(feature="schema-generator", derive(schemars::JsonSchema))]
struct Configuration {
    network: network::NetworkConfiguration,
}

fn run(config_file: &str) -> Result<(), error::Error> {
    info!("🛜 System init started 🛜");

    early::SystemInit::default().init()?;


    info!("⌛ Starting tasks ⌛");
    let shell_task = task::ShellTask::new()?;
    let mut tasks: Vec<Box<dyn task::Task>> = vec![Box::new(shell_task)];

    let config = match load_config(config_file) {
        Ok(v) => Some(v),
        Err(e) => {
            error!("Failed to parse configuration file: {:?}", e);
            None
        }
    };

    if let Some(config) = config {
        let network_task = network::NetworkTask::new(config.network)?;
        tasks.push(Box::new(network_task));
    }

    loop {
        let mut poll_fds = tasks
            .iter()
            .filter_map(|task| task.poll_fd())
            .map(|(fd, flags)| {
                let mut pollfd = nc::pollfd_t::default();
                pollfd.fd = fd.get();
                pollfd.events = flags;
                pollfd
            })
            .collect::<Vec<_>>();

        unsafe { nc::poll(&mut poll_fds, 120000) }.unwrap();
        let mut new_tasks = vec![];
        for mut task in tasks {
            if !task.is_alive()? {
                info!("Task {:?} died, checking restart strategy", task);
                match task.restart_strategy() {
                    task::RestartStrategy::Never => {
                        info!("Task {:?} isn't configured to restart.", task);
                    }
                    task::RestartStrategy::Reboot => {
                        error!("Task {:?} requires a device reboot. Rebooting.", task);
                        return Err(error::Error::TaskDied);
                    }
                    task::RestartStrategy::RestartProcess => {
                        info!("Task {:?} is configured fo restart, restarting it", task);
                        match task.restart() {
                            Ok(_) => new_tasks.push(task),
                            Err(e) => {
                                error!(
                                    "Failed to restart task {:?}, giving up on it: {:?}",
                                    task, e
                                );
                            }
                        }
                    }
                }
            } else {
                new_tasks.push(task);
            }
        }

        tasks = new_tasks;

        if tasks.len() > 0 {
            debug!("There are still {} children alive, continuing", tasks.len());
            continue;
        }
        debug!("all children must have died 😿");

        return Err(error::Error::AllChildrenDied);
    }
}
