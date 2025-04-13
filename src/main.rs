mod core;
mod modules;
use actix_web::{middleware::Logger, web, App, HttpResponse, HttpServer, Responder};
use core::database::create_sqlite_pool;
use modules::auth::service as auth_service;
use modules::user::service as user_service;

#[actix_web::get("/")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hello long compiles!")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let pool = create_sqlite_pool();

    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));

    HttpServer::new(move || {
        App::new()
            .wrap(Logger::default())
            .app_data(web::Data::new(pool.clone()))
            .service(hello)
            .service(web::scope("/users").service(user_service::get_users))
            .service(
                web::scope("/auth")
                    .service(auth_service::sign_up)
                    .service(auth_service::login),
            )
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
