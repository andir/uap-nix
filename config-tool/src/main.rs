use clap::Parser;
use config::load_config;

#[derive(Parser)]
pub enum Command {
    GenerateSchema,
    VerifyConfig { file_name: String },
}

fn main() {
    let args = Command::parse();
    match args {
        Command::GenerateSchema => {
            let schema = schemars::schema_for!(config::Configuration);
            println!("{}", serde_json::to_string_pretty(&schema).unwrap());
        }
        Command::VerifyConfig { file_name } => {
	    load_config(&file_name).unwrap();
	}
    }
}
