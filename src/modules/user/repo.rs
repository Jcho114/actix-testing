use crate::modules::user::model::{NewUser, User, UserIdPassword};
use crate::modules::user::schema::users::dsl::*;
use diesel::prelude::SqliteConnection;
use diesel::query_dsl::methods::{FilterDsl, OrderDsl, SelectDsl};
use diesel::result::Error::NotFound;
use diesel::{ExpressionMethods, RunQueryDsl};

pub fn check_if_email_dne(conn: &mut SqliteConnection, user_email: String) -> bool {
    let user = users
        .select(id)
        .filter(email.eq(user_email))
        .first::<i32>(conn);
    if let Err(NotFound) = user {
        return true;
    } else {
        return false;
    }
}

pub fn insert_user(conn: &mut SqliteConnection, user_email: String, hash: String) -> User {
    let new_user = NewUser {
        email: user_email,
        hashed_password: hash,
    };

    diesel::insert_into(users)
        .values(&new_user)
        .execute(conn)
        .expect("Error inserting new user");

    users
        .select((id, email, created_at, updated_at, deleted_at))
        .order(id.desc())
        .first(conn)
        .expect("Error retrieveing user info")
}

pub fn select_users(conn: &mut SqliteConnection) -> Vec<User> {
    users
        .select((id, email, created_at, updated_at, deleted_at))
        .load::<User>(conn)
        .expect("Error retrieving users")
}

pub fn retrieve_user_id_password(
    conn: &mut SqliteConnection,
    user_email: String,
) -> Option<UserIdPassword> {
    let user = users
        .select((id, hashed_password))
        .filter(email.eq(user_email))
        .first(conn);
    match user {
        Ok(id_password) => Some(id_password),
        Err(_) => None,
    }
}
