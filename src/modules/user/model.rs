use crate::modules::user::schema;
use chrono::NaiveDateTime;
use diesel::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Queryable, Selectable, Serialize, Deserialize)]
#[diesel(table_name = schema::users)]
pub struct User {
    pub id: i32,
    pub email: String,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
    pub deleted_at: Option<NaiveDateTime>,
}

#[derive(Insertable, Selectable, Serialize)]
#[diesel(table_name = schema::users)]
pub struct NewUser {
    pub email: String,
    pub hashed_password: String,
}
