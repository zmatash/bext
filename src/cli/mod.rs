use clap::{Parser, Subcommand};
use colored::Colorize;

mod build_cmd;
mod clean_cmd;
mod init_cmd;
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
    Link {
        #[arg(short, long, help = "Replace existing paths if they exist")]
        replace: bool,
    },
    Unlink,
    Clean,
    Build,
    Init,
}

pub fn run() {
    let args = Args::parse();
    let result = match args.command {
        Commands::Link { replace } => match link_cmd::run_link_command(replace) {
            Ok(result) => {
                for path in &result.linked {
                    println!("{} {}", "Linked:".green(), path.display());
                }
                for path in &result.skipped {
                    println!("{} {}", "Skipped:".yellow(), path.display());
                }
                Ok(())
            }
            Err(e) => Err(e.to_string()),
        },
        Commands::Unlink => match unlink_cmd::run_unlink_command() {
            Ok(result) => {
                for path in &result.removed {
                    println!("{} {}", "Removed:".green(), path.display());
                }
                for path in &result.not_found {
                    println!("{} {}", "Not found:".yellow(), path.display());
                }
                Ok(())
            }
            Err(e) => Err(e.to_string()),
        },
        Commands::Clean => clean_cmd::run_clean_command().map_err(|e| e.to_string()),
        Commands::Build => build_cmd::run_build_command().map_err(|e| e.to_string()),
        Commands::Init => init_cmd::run_init_command().map_err(|e| e.to_string()),
    };

    if let Err(e) = result {
        eprintln!("{}", e.red());
        std::process::exit(1);
    }
}
