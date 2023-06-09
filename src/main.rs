use actix_web::{App, HttpServer};
use std::{env, net::Ipv4Addr};

mod worktime;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let port_key = "FUNCTIONS_CUSTOMHANDLER_PORT";
    let port = match env::var(port_key) {
        Ok(val) => val.parse().expect("Custom Handler port is not a number!"),
        Err(_) => 3000,
    };

    HttpServer::new(|| App::new().service(worktime::services::get_worktime))
        .bind((Ipv4Addr::LOCALHOST, port))?
        .run()
        .await
}
