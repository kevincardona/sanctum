use clap::{Parser, Subcommand};

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
pub struct Args {
    #[command(subcommand)]
    pub command: Commands,
    #[arg(short, long)]
    pub verbose: bool,
    #[arg(short = 'c', long, value_name = "FILE", default_value = "~/.config/nglue/config.toml")]
    pub config: String,
}

#[derive(Subcommand, Debug)]
pub enum Commands {
    List,
    AddServer {
        name: String,
        host: String,
        port: u16,
    },
}


