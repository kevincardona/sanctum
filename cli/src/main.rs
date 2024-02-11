mod cli;
mod config;

use clap::Parser;
use cli::{Args, Commands, ServerCommands};
use config::Config;

fn main() {
    let args = Args::parse();
    let mut config = Config::from_file(&args.config).expect("Failed to read configuration file");

    match args.command {
        Commands::Version => println!(env!("CARGO_PKG_VERSION")),
        Commands::Server { command } => match command {
            ServerCommands::Add { name, host, port } => handle_result(config.add_server(&name, &host, port, &args.config)),
            ServerCommands::Remove { name } => handle_result(config.remove_server(&name, &args.config)),
            ServerCommands::Use { name } => handle_result(config.set_current_server(&name, &args.config)),
            ServerCommands::List => config.list_servers(),
        },
        Commands::Use { server } => handle_result(config.set_current_server(&server, &args.config)),
        Commands::HTTP { url } => println!("HTTP command not implemented yet"),
        Commands::TCP { host, port } => println!("TCP command not implemented yet"),
    }
}

fn handle_result(result: Result<(), Box<dyn std::error::Error>>) {
    match result {
        Ok(_) => {}
        Err(e) => eprintln!("Error: {}", e),
    }
}
