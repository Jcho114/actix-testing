mod modules;
use crate::modules::user::service as user_service;
use actix_web::{middleware::Logger, web, App, HttpResponse, HttpServer, Responder};
use diesel::prelude::*;
use diesel::r2d2::{ConnectionManager, Pool};

#[actix_web::get("/")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hello long compiles!")
}

pub type DbPool = Pool<ConnectionManager<SqliteConnection>>;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let manager = ConnectionManager::<SqliteConnection>::new("data.db");
    let pool = Pool::builder()
        .test_on_check_out(true)
        .build(manager)
        .expect("Could not build connection pool");

    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));

    HttpServer::new(move || {
        App::new()
            .wrap(Logger::default())
            .app_data(web::Data::new(pool.clone()))
            .service(hello)
            .service(
                web::scope("/users")
                    .service(user_service::insert_user)
                    .service(user_service::get_users),
            )
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
