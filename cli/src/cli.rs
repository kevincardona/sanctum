use clap::{Parser, Subcommand};

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
pub struct Args {
    #[command(subcommand)]
    pub command: Commands,
    #[arg(short = 'c', long, value_name = "FILE", default_value = "~/.config/sanctum/config.toml")]
    pub config: String,
}

#[derive(Subcommand, Debug)]
pub enum Commands {
    #[command(name = "version", alias = "v")]
    Version,
    #[command(name = "server", aliases = ["s", "servers"])]
    Server {
        #[command(subcommand)]
        command: ServerCommands,
    },
    #[command(name = "use")]
    Use { server: String },
    HTTP { url: String },
    TCP { host: String, port: u16 },
}

#[derive(Subcommand, Debug)]
pub enum ServerCommands {
    #[command(name = "add", alias = "new")]
    Add {
        name: String,
        host: String,
        port: u16,
    },
    #[command(name = "remove", alias = "rm")]
    Remove {
        name: String,
    },
    #[command(name = "use", alias = "set")]
    Use {
        name: String,
    },
    List,
    // SetToken {
    //     token: String,
    // }
}
