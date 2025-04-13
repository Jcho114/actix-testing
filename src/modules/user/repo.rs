use crate::modules::user::model::{NewUser, User};
use chrono::Utc;
use diesel::prelude::SqliteConnection;
use diesel::query_dsl::methods::{OrderDsl, SelectDsl};
use diesel::{ExpressionMethods, RunQueryDsl};

pub fn insert_user(conn: &mut SqliteConnection, user_name: String) -> User {
    use crate::modules::user::schema::users::dsl::*;

    let now = Utc::now().naive_utc();
    let new_user = NewUser {
        name: user_name,
        created_at: now,
        updated_at: now,
    };

    diesel::insert_into(users)
        .values(&new_user)
        .execute(conn)
        .expect("Error inserting new user");

    users
        .order(id.desc())
        .first(conn)
        .expect("Error retrieveing user info")
}

pub fn select_users(conn: &mut SqliteConnection) -> Vec<User> {
    use crate::modules::user::schema::users::dsl::*;

    users
        .select((id, name, created_at, updated_at, deleted_at))
        .load::<User>(conn)
        .expect("Error retrieving users")
}
