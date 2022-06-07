use reqwest::{Client, Request, Response};
use reqwest_middleware::{ClientBuilder, Middleware, Next, Result};
use task_local_extensions::Extensions;

struct LoggingMiddleware;

#[async_trait::async_trait]
impl Middleware for LoggingMiddleware {
    async fn handle(
        &self,
        req: Request,
        extensions: &mut Extensions,
        next: Next<'_>,
    ) -> Result<Response> {
        println!("Request started {:?}  {:?}", req, extensions);
        let res = next.run(req, extensions).await;
        println!("Result: {:?}", res);
        res
    }
}

pub async fn run() {
    let reqwest_client = Client::builder().build().unwrap();
    let client = ClientBuilder::new(reqwest_client)
        .with(LoggingMiddleware)
        .build();
    let resp = client
        .get(
            "http://10.130.136.101:8848/nacos/v1/ns/instance/list?serviceName=saascloud-scms-admin",
        )
        .send()
        .await
        .unwrap();
    println!("TrueLayer page HTML: {}", resp.text().await.unwrap());
}
