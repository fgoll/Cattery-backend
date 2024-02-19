use std::{env, primitive};

use serde::Serialize;
use sqlx::{prelude::FromRow, Pool, Sqlite, SqlitePool};
use time::PrimitiveDateTime;

pub async fn establish_connection() -> Pool<Sqlite> {
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");

    let pool = SqlitePool::connect(&database_url)
        .await
        .expect("can't connect to databse");

    pool
}

#[derive(FromRow)]
pub struct User {
    pub id: i32,
    pub openid: String,
    pub session_key: String,
    pub created_at: PrimitiveDateTime,
    pub updated_at: PrimitiveDateTime,
}

#[derive(FromRow)]
pub struct AdminUser {
    pub id: i32,
    pub phone: String,
    pub password: String,
    pub created_at: PrimitiveDateTime,
    pub updated_at: PrimitiveDateTime,
}

#[derive(FromRow, Debug, Serialize)]
pub struct Shop {
    pub id: i32,
    pub app_id: String,
    pub name: String,
    pub content: String,
    pub created_at: PrimitiveDateTime,
    pub updated_at: PrimitiveDateTime,
}