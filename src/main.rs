#[macro_use]
extern crate lazy_static;
extern crate mut_static;

mod server_regist;
use actix_web::{get, App, HttpServer, Responder};
use server_regist::{read_config::Config, regist_nacos::RegistNacosTask, rest_template};

#[get("/")]
async fn index() -> impl Responder {
    let config = Config::new();
    rest_template::NacosService::update_instance_list(config);
    format!("{:?}", "dd")
}

#[get("/test")]
async fn index11() -> impl Responder {
    rest_template::NacosService::get_Value();
    format!("dsdsdsd")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    rest_template::NacosService::new();
    RegistNacosTask::regist_task().await;
    RegistNacosTask::beat_task();

    HttpServer::new(move || App::new().service(index).service(index11))
        .bind("127.0.0.1:9980")?
        .run()
        .await
}
