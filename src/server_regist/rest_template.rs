use crate::server_regist::read_config::Config;
use mut_static::MutStatic;
use reqwest::{Client, Url};
use serde_json::value::Value;
use std::collections::HashMap;
use tokio::task;
use tokio::time;

#[derive(Clone, Debug)]
pub struct InstanceStruct {
    pub ip: String,
    pub port: usize,
    pub dom: String,
    pub value: usize,
}

lazy_static! {
    static ref NACOS_INSTANCE_LIST: MutStatic<HashMap<String, InstanceStruct>> = MutStatic::new();
}

pub struct NacosService;

impl NacosService {
    pub fn new() -> Self {
        let map: HashMap<String, InstanceStruct> = HashMap::new();
        NACOS_INSTANCE_LIST.set(map).unwrap();
        NacosService
    }

    pub async fn search_instance_list(config: Config) {
        println!("search instance list start");
        let client = Client::default();
        let url_str = format!(
            "http://{}:{}/nacos/v1/ns/instance",
            config.nacos_server, config.nacos_port
        );
        let mut url = Url::parse(url_str.as_str()).unwrap();

        url.query_pairs_mut()
            .append_pair("serviceName", config.name.as_str());

        let resp = match client.get(url).send().await {
            Ok(resp) => resp,
            Err(e) => panic!("{:?}", e),
        };

        if resp.status() != 200 {
            panic!("search instance list error");
        }

        let aa = match resp.json::<Value>().await {
            Ok(value) => value,
            Err(e) => panic!("get resp err {:?}", e),
        };

        println!("{:?}", aa.get("dom"));
        println!("{:?}", aa.get("hosts"));
        let instace_map = InstanceStruct {
            ip: "dd".to_string(),
            port: 8080,
            dom: "ss".to_string(),
            value: 10,
        };

        let mut mut_handle = NACOS_INSTANCE_LIST.write().unwrap();
        mut_handle.insert("dd".to_string(), instace_map.clone());
        println!("insert globle value");
    }

    pub fn get_Value() {
        let value = NACOS_INSTANCE_LIST.read().unwrap();
        println!("{:?}", &value.get("dd"));
    }

    pub fn update_instance_list(config: Config) {
        let start = time::Instant::now();
        let mut interval = time::interval_at(start, time::Duration::from_secs(10));
        task::spawn(async move {
            loop {
                let cfg = config.clone();
                task::spawn(async move {
                    let c = cfg.clone();
                    NacosService::search_instance_list(cfg);
                });
                interval.tick().await;
            }
        });
    }
}
