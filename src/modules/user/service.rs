use crate::core::database::DbPool;
use crate::modules::user::repo;
use actix_web::{web, Error, Responder, Result};

#[actix_web::get("")]
pub async fn get_users(pool: web::Data<DbPool>) -> Result<impl Responder, Error> {
    let mut conn = pool.get().expect("Could not get db connection from pool");
    let users = repo::select_users(&mut conn);
    Ok(web::Json(users))
}
