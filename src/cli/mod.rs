use clap::{Parser, Subcommand};

mod link_cmd;
mod unlink_cmd;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
pub struct Args {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand, Debug, Clone)]
pub enum Commands {
    Link,
    Unlink,
    Clean,
    Build,
}

pub fn run() {
    let args = Args::parse();
    let result = match args.command {
        Commands::Link => link_cmd::run_link_command().map_err(|e| e.to_string()),
        Commands::Unlink => unlink_cmd::run_unlink_command().map_err(|e| e.to_string()),
        Commands::Clean => todo!(),
        Commands::Build => todo!(),
    };
    if let Err(e) = result {
        eprintln!("Error: {e}");
        std::process::exit(1);
    }
}
