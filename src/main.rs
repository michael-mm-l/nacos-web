#[macro_use]
extern crate lazy_static;
extern crate mut_static;

mod server_regist;
use actix_web::{get, App, HttpServer, Responder};
use server_regist::{regist_nacos::RegistNacosTask, rest_client_task::NacosService};

#[get("/")]
async fn index() -> impl Responder {
    format!("{:?}", "dd")
}

#[get("/test")]
async fn index11() -> impl Responder {
    format!("dsdsdsd")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let service = NacosService::new();
    let regist_nacos_task = RegistNacosTask::new();
    regist_nacos_task.task().await;
    service.update_instance_list();

    HttpServer::new(move || App::new().service(index).service(index11))
        .bind("127.0.0.1:9980")?
        .run()
        .await
}
