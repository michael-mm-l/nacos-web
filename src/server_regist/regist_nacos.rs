use reqwest::header::HeaderMap;
use reqwest::{Client, Url};
use std::collections::HashMap;
use std::thread;
use std::time::Duration;
use tokio::task;
use local_ipaddress;
use serde_derive::Serialize;
use crate::server_regist::read_config::Config;

#[derive(Serialize)]
struct Beat {
    ip: String,
    port: String,
    scheduled: bool,
    serviceName: String,
    weight: String,
}

pub struct Instance;

/**
 * service regist and beat to nacos
 */
impl Instance {
    pub async fn regist_task() {
        let config = Config::new();
        Instance::regist(config).await;
    }

    pub fn beat_task() {
        let config = Config::new();
        task::spawn(async move {
            loop {
                thread::sleep(Duration::from_secs(5));
                Instance::beat(config.clone()).await;
            }
        });
    }


    // 像nacos 注册当前主机接口
    async fn regist(config: Config) {
        println!("regis start");
        let client = Client::new();
        let mut header = HeaderMap::new();
        header.insert("Content-Type", "application/json".parse().unwrap());

        let url_str = format!("http://{}:{}/nacos/v1/ns/instance",
                              config.nacos_server, config.nacos_port
        );
        let mut url = Url::parse(url_str.as_str()).unwrap();
        url.query_pairs_mut().append_pair("ip", local_ipaddress::get().unwrap().as_str())
            .append_pair("port", config.port.as_str())
            .append_pair("serviceName", config.name.as_str());

        let resp = match client.post(url).send().await {
            Ok(resp) => resp,
            Err(e) => panic!("{:?}", e),
        };

        if resp.status() != 200 {
            println!("service regist error");
        }
        println!("service regist seccussful");
    }

    async fn beat(config: Config) {
        println!("beat start");
        let client = Client::new();
        let mut header = HeaderMap::new();
        header.insert("Content-Type", "application/json".parse().unwrap());

        let con = config.clone();
        let beat = Beat {
            ip: local_ipaddress::get().unwrap(),
            port: config.port,
            scheduled: true,
            serviceName: config.name,
            weight: "1".to_string(),
        };

        let url_str = format!("http://{}:{}/nacos/v1/ns/instance/beat",
                              config.nacos_server, config.nacos_port);
        let mut url = Url::parse(url_str.as_str()).unwrap();
        let beat_str = serde_json::to_string(&beat).unwrap();
        url.query_pairs_mut().append_pair("serviceName", con.name.as_str())
            .append_pair("beat",beat_str.as_str());

        let resp = match client.put(url).headers(header).send().await {
            Ok(resp) => resp,
            Err(e) => panic!("{:?}", e),
        };

        if resp.status() != 200 {
            println!("service beat error {:?}", resp);
        }
        println!("service beat seccussful");
    }
}
