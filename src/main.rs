#[macro_use]
extern crate log;

use actix_web::{App, HttpResponse, HttpServer, Error};
use anyhow::Result;
use dotenv::dotenv;
use listenfd::ListenFd;
use sqlx::PgPool;
use std::env;
use actix_web::middleware::Logger;

// import todo module (routes and model)
mod paths;
mod mahasiswa;
mod multi;
mod client;

// default / handler
// async fn index() -> impl Responder {
//     HttpResponse::Ok().body(r#"
//         Welcome to Actix-web with SQLx Todos example.
//         Available routes:
//         GET /todos -> list of all todos
//         POST /todo -> create new todo, example: { "description": "learn actix and sqlx", "done": false }
//         GET /todo/{id} -> show one todo with requested id
//         PUT /todo/{id} -> update todo with requested id, example: { "description": "learn actix and sqlx", "done": true }
//         DELETE /todo/{id} -> delete todo with requested id
//     "#
//     )
// }

#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct WebServiceResponse {
    pub status: String,
    pub info: String,
}

// Utils
pub fn ws_response(status: &str, info: &str) -> core::result::Result<HttpResponse, Error> {
    let data = WebServiceResponse{
        status: status.into(), info: info.into()
    };
    Ok(HttpResponse::Ok().json(data))
}

#[actix_web::main]
async fn main() -> Result<()> {
    dotenv().ok();
    env_logger::init();

    // this will enable us to keep application running during recompile: systemfd --no-pid -s http::5000 -- cargo watch -x run
    let mut listenfd = ListenFd::from_env();

    let database_url =
        env::var("DATABASE_URL").expect("DATABASE_URL is not set in .env file");
    let db_pool = PgPool::connect(&database_url).await?;

    let mut server = HttpServer::new(move || {
        App::new()
            .data(db_pool.clone()) // pass database pool to application so we can access it inside handlers
            .wrap(Logger::new("%a %r %s %{User-Agent}i"))
            // .route("/", web::get().to(index))
            .configure(paths::init) // init todo routes
    });

    server = match listenfd.take_tcp_listener(0)? {
        Some(listener) => server.listen(listener)?,
        None => {
            let host = env::var("HOST").expect("HOST is not set in .env file");
            let port = env::var("PORT").expect("PORT is not set in .env file");
            server.bind(format!("{}:{}", host, port))?
        }
    };

    info!("Starting server");
    server.run().await?;

    Ok(())
}