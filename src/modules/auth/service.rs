use crate::{
    core::database::DbPool,
    modules::{
        auth::util::{self, generate_access_token},
        user::repo,
    },
};
use actix_web::{cookie::Cookie, error, web, Error, HttpRequest, HttpResponse, Responder, Result};
use once_cell::sync::Lazy;
use pwhash::bcrypt;
use regex::Regex;
use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
struct SignUpRequestBody {
    email: String,
    password: String,
}

static EMAIL_REGEX: Lazy<Regex> = Lazy::new(|| {
    Regex::new(r"^[\w\.-]+@([\w-]+\.)+[\w-]{2,4}$").expect("Unable to compile email regex")
});

#[actix_web::post("/signup")]
pub async fn sign_up(
    pool: web::Data<DbPool>,
    body: web::Json<SignUpRequestBody>,
) -> Result<impl Responder, Error> {
    if !EMAIL_REGEX.is_match(&body.email) {
        return Err(error::ErrorBadRequest("Provided email is invalid"));
    }

    let mut conn = pool.get().expect("Could not get db conneciton from pool");
    if !repo::check_if_email_dne(&mut conn, body.email.clone()) {
        return Err(error::ErrorBadRequest("User with email already exists"));
    }
    let hashed_password =
        bcrypt::hash(body.password.clone()).expect("Error hashing plain password");
    let new_user = repo::insert_user(&mut conn, body.email.clone(), hashed_password);

    let access_token = util::generate_access_token(new_user.id);
    if let Err(_) = access_token {
        return Err(error::ErrorUnauthorized(
            "Unable to generate access token for user",
        ));
    }
    let refresh_token = util::generate_refresh_token(new_user.id);
    if let Err(_) = refresh_token {
        return Err(error::ErrorUnauthorized(
            "Unable to generate refresh token for user",
        ));
    }

    let access_cookie = Cookie::build("access_token", access_token.unwrap().clone())
        .path("/")
        .http_only(true)
        .secure(true)
        .same_site(actix_web::cookie::SameSite::Strict)
        .finish();

    let refresh_cookie = Cookie::build("refresh_token", refresh_token.unwrap().clone())
        .path("/")
        .http_only(true)
        .secure(true)
        .same_site(actix_web::cookie::SameSite::Strict)
        .finish();

    Ok(HttpResponse::Ok()
        .cookie(access_cookie)
        .cookie(refresh_cookie)
        .json(new_user))
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
    let user_id_password_option = repo::retrieve_user_id_password(&mut conn, body.email.clone());
    if let None = user_id_password_option {
        return Err(error::ErrorUnauthorized("User with email does not exist"));
    }
    let user_id_password = user_id_password_option.unwrap();
    let verified = bcrypt::verify(body.password.clone(), &user_id_password.hashed_password);
    if !verified {
        return Err(error::ErrorUnauthorized("Invalid password"));
    }

    let access_token = util::generate_access_token(user_id_password.id);
    if let Err(_) = access_token {
        return Err(error::ErrorUnauthorized(
            "Unable to generate access token for user",
        ));
    }
    let refresh_token = util::generate_refresh_token(user_id_password.id);
    if let Err(_) = refresh_token {
        return Err(error::ErrorUnauthorized(
            "Unable to generate refresh token for user",
        ));
    }

    let access_cookie = Cookie::build("access_token", access_token.unwrap().clone())
        .path("/")
        .http_only(true)
        .secure(true)
        .same_site(actix_web::cookie::SameSite::Strict)
        .finish();

    let refresh_cookie = Cookie::build("refresh_token", refresh_token.unwrap().clone())
        .path("/")
        .http_only(true)
        .secure(true)
        .same_site(actix_web::cookie::SameSite::Strict)
        .finish();

    let response = LoginResponseBody {
        message: String::from("User successfully logged in"),
    };

    Ok(HttpResponse::Ok()
        .cookie(access_cookie)
        .cookie(refresh_cookie)
        .json(response))
}

#[derive(Serialize)]
struct ValidateResponseBody {
    message: String,
}

#[actix_web::post("/validate")]
pub async fn validate(request: HttpRequest) -> Result<impl Responder, Error> {
    let access_cookie_option = request.cookie("access_token");
    if let None = access_cookie_option {
        return Err(error::ErrorUnauthorized("Access token is not provided"));
    }
    let access_token = access_cookie_option.unwrap().value().to_string();
    if let None = util::validate_access_token(access_token) {
        return Err(error::ErrorUnauthorized("Provided access token is invalid"));
    }

    let response = ValidateResponseBody {
        message: String::from("Access token is valid"),
    };
    Ok(web::Json(response))
}

#[derive(Serialize)]
struct RefreshResponseBody {
    message: String,
}

#[actix_web::post("/refresh")]
pub async fn refresh(request: HttpRequest) -> Result<impl Responder, Error> {
    let refresh_cookie_option = request.cookie("refresh_token");
    if let None = refresh_cookie_option {
        return Err(error::ErrorUnauthorized("Refresh token is not provided"));
    }
    let refresh_token = refresh_cookie_option.unwrap().value().to_string();
    let token_data_option = util::validate_refresh_token(refresh_token);
    if let None = token_data_option {
        return Err(error::ErrorUnauthorized(
            "Provided refresh token is invalid",
        ));
    }
    let token_data = token_data_option.unwrap();

    let access_token = generate_access_token(token_data.claims.sub);
    let access_cookie = Cookie::build("access_token", access_token.unwrap().clone())
        .path("/")
        .http_only(true)
        .secure(true)
        .same_site(actix_web::cookie::SameSite::Strict)
        .finish();
    let response = RefreshResponseBody {
        message: String::from("Generated a new access token"),
    };
    Ok(HttpResponse::Ok().cookie(access_cookie).json(response))
}
