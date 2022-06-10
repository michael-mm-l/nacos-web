use crate::server_regist::rest_client_task;
use reqwest::{Client, Request, Response};
use reqwest_middleware::{ClientBuilder, ClientWithMiddleware, Middleware, Next, Result};
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

pub async fn run() -> ClientWithMiddleware {
    let reqwest_client = Client::builder().build().unwrap();
    ClientBuilder::new(reqwest_client)
        .with(LoggingMiddleware)
        .build()
}
