use crate::server_regist::read_config::Config;
use local_ipaddress;
use reqwest::header::HeaderMap;
use reqwest::{Client, Url};
use serde_derive::Serialize;
use tokio::task;
use tokio::time;

#[derive(Serialize)]
struct Beat {
    ip: String,
    port: String,
    scheduled: bool,
    serviceName: String,
    weight: String,
}

pub struct RegistNacosTask {
    pub config: Config,
}

/**
 * service regist and beat to nacos
 */
impl RegistNacosTask {
    pub fn new() -> Self {
        RegistNacosTask {
            config: Config::new(),
        }
    }

    pub async fn task(&self) {
        self.regist_task().await;
        self.beat_task();
    }

    async fn regist_task(&self) {
        let config = self.config.clone();
        match RegistNacosTask::regist(config).await {
            Ok(_) => (),
            Err(e) => panic!("regist service error {:?}", e),
        };
    }

    fn beat_task(&self) {
        let start = time::Instant::now();
        let mut interval = time::interval_at(start, time::Duration::from_secs(10));
        let config = self.config.clone();
        task::spawn(async move {
            loop {
                interval.tick().await;
                match RegistNacosTask::beat(config.clone()).await {
                    Ok(_) => (),
                    Err(e) => panic!("beat error {:?}", e),
                };
            }
        });
    }

    async fn regist(config: Config) -> Result<(), reqwest::Error> {
        println!("regis start");
        let client = Client::new();
        let mut header = HeaderMap::new();
        header.insert("Content-Type", "application/json".parse().unwrap());

        let url_str = format!(
            "http://{}:{}/nacos/v1/ns/instance",
            config.nacos_server, config.nacos_port
        );
        let mut url = Url::parse(url_str.as_str()).unwrap();
        url.query_pairs_mut()
            .append_pair("ip", local_ipaddress::get().unwrap().as_str())
            .append_pair("port", config.port.as_str())
            .append_pair("serviceName", config.name.as_str());

        client.post(url).send().await?;
        Ok(())
    }

    async fn beat(config: Config) -> Result<(), reqwest::Error> {
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

        let url_str = format!(
            "http://{}:{}/nacos/v1/ns/instance/beat",
            config.nacos_server, config.nacos_port
        );
        let mut url = Url::parse(url_str.as_str()).unwrap();
        let beat_str = serde_json::to_string(&beat).unwrap();
        url.query_pairs_mut()
            .append_pair("serviceName", con.name.as_str())
            .append_pair("beat", beat_str.as_str());

        client.put(url).headers(header).send().await;
        Ok(())
    }
}
