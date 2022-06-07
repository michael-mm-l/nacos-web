use local_ipaddress;
use serde_derive::Deserialize;
use std::fs::OpenOptions;
use std::io::prelude::*;
use toml;

#[derive(Deserialize, Clone)]
pub struct Config {
    pub port: String,
    pub name: String,
    pub nacos_server: String,
    pub nacos_port: String,
    pub nacos_namespace: Option<String>
}

impl Config {
    pub fn new() -> Self {
        let mut file = OpenOptions::new()
            .read(true)
            .open("Config.toml")
            .expect("open file error");
        let mut str_val = String::new();
        match file.read_to_string(&mut str_val) {
            Ok(s) => s,
            Err(e) => panic!("Error Reading file: {}", e),
        };

        let config: Config = toml::from_str(&str_val).unwrap();

        config
    }
}

