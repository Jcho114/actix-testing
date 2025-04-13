use crate::{core::database::DbPool, modules::user::repo};
use actix_web::{error, web, Error, Responder, Result};
use pwhash::bcrypt;
use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
struct SignUpRequestBody {
    email: String,
    password: String,
}

#[actix_web::post("/signup")]
pub async fn sign_up(
    pool: web::Data<DbPool>,
    body: web::Json<SignUpRequestBody>,
) -> Result<impl Responder, Error> {
    let mut conn = pool.get().expect("Could not get db conneciton from pool");
    if let false = repo::check_if_email_dne(&mut conn, body.email.clone()) {
        return Err(error::ErrorBadRequest("User with email already exists"));
    }
    let hashed_password =
        bcrypt::hash(body.password.clone()).expect("Error hashing plain password");
    let new_user = repo::insert_user(&mut conn, body.email.clone(), hashed_password);
    Ok(web::Json(new_user))
}

#[derive(Deserialize)]
struct LoginRequestBody {
    email: String,
    password: String,
}

#[derive(Serialize)]
struct LoginResponseBody {
    message: String,
}

#[actix_web::post("/login")]
pub async fn login(
    pool: web::Data<DbPool>,
    body: web::Json<LoginRequestBody>,
) -> Result<impl Responder, Error> {
    let mut conn = pool.get().expect("Could not get db connection from pool");
    let hash = repo::retrieve_user_hashed_password(&mut conn, body.email.clone());
    if let None = hash {
        return Err(error::ErrorUnauthorized("User with email does not exist"));
    }
    let hashed_password = hash.unwrap();
    let verified = bcrypt::verify(body.password.clone(), &hashed_password);
    if verified {
        let response = LoginResponseBody {
            message: String::from("User successfully logged in"),
        };
        return Ok(web::Json(response));
    } else {
        return Err(error::ErrorUnauthorized("Invalid password"));
    }
}
