#[macro_use]
extern crate actix_web;

use std::{env, io};

use actix_cors::Cors;
use actix_web::{middleware, App, HttpServer};
mod proof;

#[tokio::main(flavor = "multi_thread", worker_threads = 20)]
async fn main() -> io::Result<()> {
    env::set_var("RUST_LOG", "actix_web=debug,actix_server=info");
    env_logger::init();

    let result = HttpServer::new(|| {
        let cors = Cors::permissive();

        App::new()
            .wrap(cors)
            .wrap(middleware::Logger::default())
            .service(proof::post)
    })
    .bind("0.0.0.0:9090")?
    .run()
    .await;

    println!("{}", result.is_ok());
    return result;
}
