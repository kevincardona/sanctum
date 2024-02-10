use serde::{Serialize, Deserialize};
use std::fs;
use toml;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Config {
    pub servers: Vec<Server>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Server {
    pub name: String,
    pub host: String,
    pub port: u16
}

impl Config {
    pub fn from_file(path: &str) -> Result<Self, Box<dyn std::error::Error>> {
        let config_path: String = shellexpand::tilde(path).to_string();
        println!("Using config file: {}", config_path);
        let contents = fs::read_to_string(config_path)?;
        let config: Self = toml::from_str(&contents)?;
        Ok(config)
    }

    // public method to add server to the [[servers]] array in the config.toml file
    pub fn add_server(&self, server: Server, config_path: &str) {
        let config: &mut Config = &mut self.clone();
        config.servers.push(server);
        let toml = toml::to_string(&config).unwrap();
        fs::write(config_path, toml).expect("Failed to write to file");
    }
}

