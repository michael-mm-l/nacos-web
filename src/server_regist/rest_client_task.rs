use crate::server_regist::read_config::Config;
use mut_static::MutStatic;
use reqwest::{Client, Url};
use serde_derive::{Deserialize, Serialize};
use serde_json::value::Value;
use std::collections::HashMap;
use tokio::{task, time};

#[derive(Clone, Debug, Deserialize, Serialize)]
struct NacosResp {
    pub name: String,
    pub groupName: String,
    pub clusters: Option<String>,
    pub cacheMillis: usize,
    pub hosts: Vec<NacosHosts>,
    pub lastRefTime: usize,
    pub checksum: Option<String>,
    pub allIPs: bool,
    pub reachProtectionThreshold: bool,
    pub valid: bool,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
struct NacosHosts {
    pub instanceId: String,
    pub ip: String,
    pub port: usize,
    pub weight: f32,
    pub healthy: bool,
    pub enabled: bool,
    pub ephemeral: bool,
    pub clusterName: String,
    pub serviceName: String,
    pub metadata: Value,
    pub instanceHeartBeatInterval: usize,
    pub instanceIdGenerator: String,
    pub ipDeleteTimeout: usize,
    pub instanceHeartBeatTimeOut: usize,
}

lazy_static! {
    static ref NACOS_INSTANCE_LIST: MutStatic<HashMap<String, NacosResp>> = MutStatic::new();
}

pub struct NacosService {
    pub config: Config,
}

impl NacosService {
    pub fn new() -> Self {
        let map: HashMap<String, NacosResp> = HashMap::new();
        NACOS_INSTANCE_LIST.set(map).unwrap();
        NacosService {
            config: Config::new(),
        }
    }

    async fn search_instance_list(config: Config) -> Result<(), reqwest::Error> {
        println!("search instance list start");
        let client = Client::default();
        let url_str = format!(
            "http://{}:{}/nacos/v1/ns/instance/list",
            config.nacos_server, config.nacos_port
        );
        let mut url = Url::parse(url_str.as_str()).unwrap();

        url.query_pairs_mut()
            .append_pair("serviceName", config.name.as_str());

        let resp = client.get(url).send().await?.json::<NacosResp>().await?;

        let res = resp.clone();
        let mut mut_handle = NACOS_INSTANCE_LIST.write().unwrap();

        mut_handle.insert(res.name, resp.clone());
        println!("insert globle value");
        Ok(())
    }

    pub fn update_instance_list(&self) {
        let start = time::Instant::now();
        let mut interval = time::interval_at(start, time::Duration::from_secs(10));
        let config = self.config.clone();
        task::spawn(async move {
            loop {
                interval.tick().await;
                let cfg = config.clone();
                task::spawn(async move {
                    match NacosService::search_instance_list(cfg).await {
                        Ok(_) => (),
                        Err(e) => panic!("{:?}", e),
                    };
                });
            }
        });
    }
}

// pub fn find_service_host(name: String) -> NacosHosts {
//     let list = NACOS_INSTANCE_LIST.read().unwrap();
//     {
//         let hosts = list.get(name.as_str()).unwrap();

//     }

// }
