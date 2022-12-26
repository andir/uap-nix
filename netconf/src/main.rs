use clap::Parser;

#[derive(Parser)]
pub struct Arguments {
    #[clap(short, long, default_value = "/config.json")]
    config_file: String,
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
    init_logging();

    let args = Arguments::parse();
    let config = config::load_config(&args.config_file).unwrap();
    netconf::network::main(config.network);
}
