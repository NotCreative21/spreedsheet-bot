use std::fs::{File, read_to_string};
use std::io::{Write, Result};
use serde::Deserialize;

#[derive(Deserialize, Debug, Clone)]
pub struct Config {
    pub token: String,
    pub prefix: String,
    pub access: Vec<u64>,
    pub db_path: String,
}

impl Config {
    fn new() -> Result<()> {
       let mut config = File::create("./config.toml")?;

       config.write_all(b"# optimal bot configuration file
token = ''
prefix = ''
access = []
db_path = ''
")?;

       Ok(())
    }
    pub fn load() -> Result<Config> {
        let config = match read_to_string("./config.toml") {
            Ok(v) => v,
            Err(_) => {
                println!("failed to load config, does it exist? creating new one now");
                Config::new()?;
                println!("created new config file at './config.toml', please fill it out");
                std::process::exit(1);
            }
        };

        let config = toml::from_str(&config)?;

        Ok(config)
    }
}
