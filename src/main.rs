use anyhow::Result;
use clap::{Parser, Subcommand};

mod commands;

#[derive(Parser)]
#[command(name = "ms-tool", about = "Access microservice contract files")]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Print the overview file for a service
    Overview {
        /// Service directory name under .microservice/
        service: String,
    },
    /// Print a domain contract file for a service
    Domain {
        /// Domain file stem (e.g. device-bind)
        domain: String,
        /// Service directory name under .microservice/
        #[arg(long)]
        service: String,
    },
    /// Extract a path entry from the service's openapi.yaml
    Api {
        /// API path as it appears in openapi.yaml (e.g. /api/device-bind/...)
        path: String,
        /// Service directory name under .microservice/
        #[arg(long)]
        service: String,
    },
}

fn main() {
    let cli = Cli::parse();
    let result: Result<()> = match cli.command {
        Commands::Overview { service } => commands::overview::run(&service),
        Commands::Domain { domain, service } => commands::domain::run(&domain, &service),
        Commands::Api { path, service } => commands::api::run(&path, &service),
    };
    if let Err(e) = result {
        eprintln!("{e}");
        std::process::exit(1);
    }
}
