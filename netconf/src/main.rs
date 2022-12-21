use serde::Deserialize;


mod neighbours;

mod early;
mod epoll;
mod error;
mod network;
mod task;
mod util;

use util::{reboot, sleep_secs, AutoCloseFD, FD};

use log::{debug, error, info};

fn init_logging() {
    simplelog::CombinedLogger::init(vec![simplelog::TermLogger::new(
        log::LevelFilter::Debug,
        simplelog::Config::default(),
        simplelog::TerminalMode::Mixed,
        simplelog::ColorChoice::Auto,
    )])
    .expect("Must be able to initialize the logging");
}

fn main() {
    let pid = util::getpid();
    init_logging();
    if let Err(e) = run() {
        error!("System failed: {:?}", e);
    }

    if pid == 1 {
        error!("No more work to watch out for. Rebooting in 15s");
        sleep_secs(15);
        reboot();
    }
}

#[derive(Debug, Deserialize)]
struct Configuration {
    network: network::NetworkConfiguration,
}

fn run() -> Result<(), error::Error> {
    info!("🛜 System init started 🛜");

    early::SystemInit::default().init()?;

    info!("⌛ Parsing configuration ⌛");
    if !util::stat("/config.yaml").is_ok() {
	error!("Missing configuration file at /config.yaml. Can't proceed.");
	return Ok(());
    }

    let fh = std::fs::File::open("/config.yaml").expect("Failed to read config.yaml");
    let data = std::io::read_to_string(fh).unwrap();
    let config : Configuration = serde_yaml::from_str(&data).unwrap();
    

    info!("⌛ Starting tasks ⌛");
    let shell_task = task::ShellTask::new()?;
    let network_task = network::NetworkTask::new(config.network)?;
    let mut tasks: Vec<Box<dyn task::Task>> = vec![Box::new(shell_task), Box::new(network_task)];

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
				error!("Failed to restart task {:?}, giving up on it: {:?}", task, e);
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
