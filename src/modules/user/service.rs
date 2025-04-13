use crate::modules::user::repo;
use crate::DbPool;
use actix_web::{web, Error, Responder, Result};
use serde::Deserialize;

#[derive(Deserialize)]
struct InsertUserRequestBody {
    name: String,
}

#[actix_web::post("")]
pub async fn insert_user(
    pool: web::Data<DbPool>,
    user: web::Json<InsertUserRequestBody>,
) -> Result<impl Responder, Error> {
    let mut conn = pool.get().expect("Could not get db conneciton from pool");
    let new_user = repo::insert_user(&mut conn, user.name.clone());
    Ok(web::Json(new_user))
}

#[actix_web::get("")]
pub async fn get_users(pool: web::Data<DbPool>) -> Result<impl Responder, Error> {
    let mut conn = pool.get().expect("Could not get db connection from pool");
    let users = repo::select_users(&mut conn);
    Ok(web::Json(users))
}
