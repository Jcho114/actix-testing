mod core;
mod modules;
use actix_web::{
    middleware::from_fn, middleware::Logger, web, App, HttpResponse, HttpServer, Responder,
};
use core::config::CONFIG;
use core::database::create_sqlite_pool;
use modules::auth::{middleware::validate_middleware, service as auth_service};
use modules::user::service as user_service;

#[actix_web::get("/")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hello long compiles!")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let pool = create_sqlite_pool();

    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));

    let environment = CONFIG.environment.clone().unwrap_or_default();
    let hostname = if environment == "docker" {
        "0.0.0.0"
    } else {
        "127.0.0.1"
    };

    HttpServer::new(move || {
        App::new()
            .wrap(Logger::default())
            .app_data(web::Data::new(pool.clone()))
            .service(hello)
            .service(
                web::scope("/users")
                    .service(user_service::get_users)
                    .wrap(from_fn(validate_middleware)),
            )
            .service(
                web::scope("/auth")
                    .service(auth_service::sign_up)
                    .service(auth_service::login)
                    .service(auth_service::validate)
                    .service(auth_service::refresh),
            )
    })
    .bind((hostname, 8080))?
    .run()
    .await
}
