use reqwest::header::HeaderMap;
use reqwest::Client;
use std::collections::HashMap;
use std::thread;
use std::time::Duration;
use tokio::task;

pub struct Instance;
/**
 * service regist and beat to nacos
 */
impl Instance {
    pub fn regist_task() {
        task::spawn(async {
            loop {
                Instance::regist().await;
                thread::sleep(Duration::from_secs(5));
            }
        });
    }

    pub fn beat_task() {
        task::spawn(async {
            loop {
                Instance::beat().await;
                thread::sleep(Duration::from_secs(5));
            }
        });
    }

    pub fn delete_service_task() {}

    // 像nacos 注册当前主机接口
    async fn regist() {
        println!("regis start");
        let client = Client::new();
        let mut header = HeaderMap::new();
        header.insert("Content-Type", "application/json".parse().unwrap());

        let mut param = HashMap::new();
        param.insert("ip", "192.168.207.56");
        param.insert("port", "8080");
        param.insert("serviceName", "rust_nacos_test");

        let url = "http://10.130.136.101:8848/nacos/v1/ns/instance?ip=192.168.207.56&port=8080&serviceName=test_rust_nacos";
        let resp = match client.post(url).send().await {
            Ok(resp) => resp,
            Err(e) => panic!("{:?}", e),
        };

        if resp.status() != 200 {
            println!("service regist error");
        }

        println!("service regist seccussful");
    }

    async fn beat() {
        println!("beat start");
        let client = Client::new();
        let mut header = HeaderMap::new();
        header.insert("Content-Type", "application/json".parse().unwrap());

        let mut param = HashMap::new();
        param.insert("ip", "192.168.207.56");
        param.insert("port", "8080");
        param.insert("serviceName", "rust_nacos_test");

        let url = "http://10.130.136.101:8848/nacos/v1/ns/instance?ip=192.168.207.56&port=8080&serviceName=test_rust_nacos";
        let resp = match client.post(url).send().await {
            Ok(resp) => resp,
            Err(e) => panic!("{:?}", e),
        };

        if resp.status() != 200 {
            println!("service regist error");
        }

        println!("service regist seccussful");
    }

    async fn delete_service() {
        println!("regis start");
        let client = Client::new();
        let mut header = HeaderMap::new();
        header.insert("Content-Type", "application/json".parse().unwrap());

        let mut param = HashMap::new();
        param.insert("ip", "192.168.207.56");
        param.insert("port", "8080");
        param.insert("serviceName", "rust_nacos_test");

        let url = "http://10.130.136.101:8848/nacos/v1/ns/instance?ip=192.168.207.56&port=8080&serviceName=test_rust_nacos";
        let resp = match client.post(url).send().await {
            Ok(resp) => resp,
            Err(e) => panic!("{:?}", e),
        };

        if resp.status() != 200 {
            println!("service regist error");
        }

        println!("service regist seccussful");
    }
}
