#[macro_use]
extern crate lazy_static;
extern crate mut_static;

mod server_regist;
use actix_web::{get, App, HttpServer, Responder};
use server_regist::rest_template;
use server_regist::regist_nacos::Instance;


#[get("/")]
async fn index() -> impl Responder {
    rest_template::NacosService::foo();
    format!("{:?}", "dd")
}

#[get("/test")]
async fn index11() -> impl Responder {
    rest_template::NacosService::getValue();
    format!("dsdsdsd")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    rest_template::NacosService::new();
    Instance::regist_task().await;
    Instance::beat_task();

    HttpServer::new(move || App::new().service(index).service(index11))
        .bind("127.0.0.1:9980")?
        .run()
        .await
}
