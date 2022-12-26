mod task;
mod util;
mod early;
mod error;

use clap::Parser;
use util::{getpid, reboot, sleep_secs, FD};
use util::ProcessStatus;
use log::{debug, error, info, warn};


#[derive(Parser, Clone, Copy)]
pub enum Command {
    Init,
}

#[derive(Parser)]
pub struct Arguments {
    #[clap(subcommand)]
    command: Command,
}

fn init_logging() {
    simplelog::CombinedLogger::init(vec![simplelog::TermLogger::new(
        log::LevelFilter::Info,
        simplelog::Config::default(),
        simplelog::TerminalMode::Mixed,
        simplelog::ColorChoice::Auto,
    )])
    .expect("Must be able to initialize the logging");
}

fn main() {
    let pid = getpid();
    init_logging();

    let args = Arguments::parse();
    match args.command {
        Command::Init => {
            if let Err(e) = run() {
                error!("System failed: {:?}", e);
            }

            if pid == 1 {
                error!("No more work to watch out for. Rebooting in 15s");
                sleep_secs(15);
                reboot();
            }
        }
        #[cfg(feature = "schema-generator")]
        Command::GenerateSchema => {
            let schema = schemars::schema_for!(Configuration);
            println!("{}", serde_json::to_string_pretty(&schema).unwrap());
        }
    };
}

fn run() -> Result<(), error::Error> {
    info!("🛜 System init started 🛜");

    early::SystemInit::default().init()?;

    // do this after /proc is mounted
    let self_exectuable = match std::env::current_exe().map(|x| x.to_str().map(|x| x.to_string())) {
        Ok(Some(e)) => e,
        Ok(None) => {
            error!("The executable path isn't a valid string :(");
            "/invalid/path".to_string()
        }
        Err(e) => {
            error!("Failed to determine my own executable path :(: {:?}", e);
            "/invalid/path/".to_string()
        }
    };


    info!("⌛ Starting tasks ⌛");
    let shell_task = task::ShellTask::new()?;
    let mut tasks: Vec<Box<dyn task::Task>> = vec![Box::new(shell_task)];

    info!("⌛ Parsing configuration ⌛");

    let network_task = task::SubprocessTask::new(
        "/bin/netconf",
        &["netconf"],
        task::RestartStrategy::Never,
        "network",
    )?;

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
            match task.is_alive()? {
                ProcessStatus::Alive => {
                    new_tasks.push(task);
                }
                ProcessStatus::Dead { status } => {
                    warn!(
                        "Task {:?} died with status {}, checking restart strategy",
                        task, status
                    );
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
                }
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
