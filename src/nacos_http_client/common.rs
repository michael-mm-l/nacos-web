use serde::Serialize;
#[derive(Debug, Clone, Deserialize)]
pub struct NacosConfig {
    pub name: String,
    pub ip: String,
    pub port: String,
    pub namespaceId: String
}

#[derive(Debug, Clone, Deserialize)]
pub struct Nacos {
    pub nacos_config: NacosConfig
}


use std::fs::File;
use std::io::prelude::*;
use std::io::Error;
impl Nacos {
   pub fn read_config() -> Nacos {
        let file_path = "Nacos.toml";
        let mut file = match File::open(file_path) {
            Ok(f) => f,
            Err(e) => panic!("no such f.ile {} exception:{}", file_path, e)
        };
        let mut buff = String::new();
        match file.read_to_string(&mut buff) {
            Ok(s) => s,
            Err(e) => panic!("read error {}", e)
        };
        let nacos: Nacos = toml::from_str(&mut buff).unwrap();
        nacos
    }
}

