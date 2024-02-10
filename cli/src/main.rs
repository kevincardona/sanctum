mod cli;
mod config;

use clap::Parser;
use cli::{Args, Commands};
use config::Config;

fn main() {
    let args = Args::parse();
    let config = Config::from_file(&args.config).expect("Failed to read configuration file");

    match args.command {
        Commands::List => {
            for server in config.servers {
                println!("{}", server.name);
            }
        }
        Commands::AddServer { name, host, port } => {
            let server = config::Server {
                name: name.clone(),
                host: host.clone(),
                port: port.clone()
            };
            config.add_server(server, &args.config);
        }
    }
}
