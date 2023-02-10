mod early;
mod error;
mod task;
mod util;

use clap::Parser;
use log::{debug, error, info, warn};
use util::ProcessStatus;
use util::{getpid, reboot, sleep_secs, FD};

#[derive(Parser, Clone)]
pub enum Command {
    Init(InitArguments),
}

#[derive(Parser, Clone)]
pub struct InitArguments {
    #[clap(short, long, default_value = "/config.json")]
    config_file: String,
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
        Command::Init(args) => {
            let config_file = config::load_config(&args.config_file).unwrap();
            if let Err(e) = run(config_file) {
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

fn run(config: config::Configuration) -> Result<(), error::Error> {
    info!("🛜 System init started 🛜");

    early::SystemInit::default().init()?;

    info!("Creating new tokio executor");
    let runtime = tokio::runtime::Runtime::new()?;
    runtime.block_on(run_tasks(config))
}

async fn run_tasks(config: config::Configuration) -> Result<(), error::Error> {
    info!("⌛ Starting tasks ⌛");
    let shell_task = task::ShellTask::new()?;
    let mut tasks: Vec<Box<dyn task::Task>> = vec![Box::new(shell_task)];

    info!("⌛ Parsing configuration ⌛");

    let network_task = task::SubprocessTask::new(
        "/bin/netconf",
        &[],
        task::RestartStrategy::Never,
        "network",
        true,
    )?;

    tasks.push(Box::new(network_task));

    if !config.services.is_empty() {
        info!("Starting runtime define services");
        for (name, cfg) in config.services.iter() {
            info!("Starting services {}", name);
            let restart_strategy = cfg.restart_strategy.into();
            let args = cfg.args.iter().map(|s| s.as_ref()).collect::<Vec<&str>>();
            let service =
                task::SubprocessTask::new(cfg.file.clone(), &args, restart_strategy, name, false)?;
            tasks.push(Box::new(service));
        }
    }

    let mut set = tokio::task::JoinSet::new();
    for task in tasks {
        set.spawn(async move {
            let mut task = task;
            let mut res = ProcessStatus::Dead { status: -1 };
            loop {
                match task.wait().await {
                    Err(e) => {
                        error!("Failed to wait for task {:?}: {:?}", task, e);
                        break;
                    }
                    Ok(ProcessStatus::Alive) => {
                        continue;
                    }
                    Ok(ProcessStatus::Dead { status }) => {
                        res = ProcessStatus::Dead { status };
                        warn!(
                            "Task {:?} died with status {}, checking restart strategy",
                            task, status
                        );
                        match task.restart_strategy() {
                            task::RestartStrategy::Never => {
                                info!("Task {:?} isn't configured to restart.", task);
                                return None;
                            }
                            task::RestartStrategy::Reboot => {
                                error!("Task {:?} requires a device reboot. Rebooting.", task);
                                break;
                            }
                            task::RestartStrategy::RestartProcess => {
                                info!("Task {:?} is configured fo restart, restarting it", task);
                                match task.restart() {
                                    Ok(_) => {},
                                    Err(e) => {
                                        error!(
                                            "Failed to restart task {:?}, giving up on it: {:?}",
                                            task, e
                                        );
                                        break;
                                    }
                                }
                            }
                        }
                    }
                }
            }

            Some(res)
        });
    }

    loop {
        tokio::select! {
            // wait for any of them to finish
            value = set.join_next() => {
                match value {
                    Some(Ok(Some(status))) => {
                        error!("Some task died. This means we've to reboot. Exit status: {:?}", status);
                        return Err(error::Error::AllChildrenDied);
                    }
                    Some(Ok(None)) => {
                        info!("A process died but doesn't require a restart.");
                    }
                    e => {
                        error!("Failed to join task futures: {:?}", e);
                        return Err(error::Error::AllChildrenDied);
                    }
                }
        }
        };
    }

    Ok(())

    //loop {
    //    let mut new_tasks = vec![];

    //    {
    //        let mut set = tokio::task::JoinSet::new();

    //        for task in tasks.iter_mut().map(|task| task.wait()) {
    //            set.spawn(async move { task.await });
    //        }

    //        // wait for any of the processes to die
    //        set.join_next().await;
    //    }
    //
    //    for mut task in tasks {
    //        match task.is_alive().await? {
    //            ProcessStatus::Alive => {
    //                new_tasks.push(task);
    //            }
    //            ProcessStatus::Dead { status } => {
    //                warn!(
    //                    "Task {:?} died with status {}, checking restart strategy",
    //                    task, status
    //                );
    //                match task.restart_strategy() {
    //                    task::RestartStrategy::Never => {
    //                        info!("Task {:?} isn't configured to restart.", task);
    //                    }
    //                    task::RestartStrategy::Reboot => {
    //                        error!("Task {:?} requires a device reboot. Rebooting.", task);
    //                        return Err(error::Error::TaskDied);
    //                    }
    //                    task::RestartStrategy::RestartProcess => {
    //                        info!("Task {:?} is configured fo restart, restarting it", task);
    //                        match task.restart() {
    //                            Ok(_) => new_tasks.push(task),
    //                            Err(e) => {
    //                                error!(
    //                                    "Failed to restart task {:?}, giving up on it: {:?}",
    //                                    task, e
    //                                );
    //                            }
    //                        }
    //                    }
    //                }
    //            }
    //        }
    //    }

    //    tasks = new_tasks;

    //    if tasks.len() > 0 {
    //        debug!("There are still {} children alive, continuing", tasks.len());
    //        continue;
    //    }
    //    debug!("all children must have died 😿");

    //    return Err(error::Error::AllChildrenDied);
    //}
}
