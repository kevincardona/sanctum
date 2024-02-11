use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;
use toml;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Config {
    pub servers: Vec<Server>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Server {
    pub name: String,
    pub host: String,
    pub port: u16,
    pub current: bool,
}

impl Config {
    pub fn from_file(path: &str) -> Result<Self, Box<dyn std::error::Error>> {
        let config_path: String = shellexpand::tilde(path).to_string();
        println!("Using config file: {}", config_path);
        if let Ok(contents) = fs::read_to_string(&config_path) {
            let config: Self = toml::from_str(&contents)?;
            Ok(config)
        } else {
            Self::create_config_dir(&config_path)?;
            Ok(Self { servers: Vec::new() })
        }
    }

    pub fn add_server(&mut self, name: &str, host: &str, port: u16, config_path: &str) -> Result<(), Box<dyn std::error::Error>> {
        let server: Server = self.verify_and_build_server(&name, &host, port)?;
        self.servers.push(server);
        let toml = toml::to_string(&self)?;
        let expanded_path: String = shellexpand::tilde(config_path).to_string();
        Self::create_config_dir(&expanded_path)?;
        fs::write(expanded_path, toml)?;
        println!("Server '{}' added.", name);
        Ok(())
    }

    pub fn remove_server(&mut self, name: &str, config_path: &str) -> Result<(), Box<dyn std::error::Error>> {
        self.servers.retain(|s| s.name != name);
        let toml = toml::to_string(&self)?;
        let expanded_path: String = shellexpand::tilde(config_path).to_string();
        Self::create_config_dir(&expanded_path)?;
        fs::write(expanded_path, toml)?;
        println!("Server '{}' removed.", name);
        Ok(())
    }

    pub fn set_current_server(&mut self, name: &str, config_path: &str) -> Result<(), Box<dyn std::error::Error>> {
        if !self.server_exists(name) {
            return Err(format!("Server with name '{}' doesn't exist.", name).into());
        }
        for s in &mut self.servers {
            s.current = s.name == name;
        }
        let toml = toml::to_string(&self)?;
        let expanded_path: String = shellexpand::tilde(config_path).to_string();
        Self::create_config_dir(&expanded_path)?;
        fs::write(expanded_path, toml)?;
        println!("Now using server: '{}'", name);
        Ok(())
    }

    pub fn list_servers(&self) {
        for server in &self.servers {
            print!("{}", server.name);
            if server.current {
                print!(" (in use)");
            }
            println!();
        }
    }

    pub fn get_current_server(&self) -> Result<Option<&Server>, Box<dyn std::error::Error>> {
        let server = self.servers.iter().find(|s| s.current);
        if let Some(server) = server {
            Ok(Some(server))
        } else {
            Err("No server is set.".into())
        }
    }

    fn verify_and_build_server(&self, name: &str, host: &str, port: u16) -> Result<Server, Box<dyn std::error::Error>> {
        if self.server_exists(name) {
            return Err(format!("Server with name '{}' already exists.", name).into());
        }
        let server = Server {
            name: name.to_string(),
            host: host.to_string(),
            port,
            current: false,
        };
        Ok(server)
    }

    fn server_exists(&self, name: &str) -> bool {
        self.servers.iter().any(|s| s.name == name)
    }

    fn create_config_dir(config_path: &str) -> Result<(), Box<dyn std::error::Error>> {
        let path = PathBuf::from(config_path);
        if let Some(parent) = path.parent() {
            if !parent.exists() {
                fs::create_dir_all(parent)?;
            }
        }
        Ok(())
    }
}

