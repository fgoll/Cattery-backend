use axum::{extract::State, Json};
use jsonwebtoken::{encode, EncodingKey, Header};
use serde::{Deserialize, Serialize};
use sqlx::{Pool, Sqlite};

use crate::db::{AdminUser, User};

use super::{
    jwt::{AuthError, Claims},
    ApiError,
};

#[derive(Deserialize)]
pub struct LoginPayload {
    code: String,
}

pub struct AdminRegisterPayload {
    phone: String,
    password: String,
}

#[derive(Serialize)]
pub struct AuthBody {
    access_token: String,
    token_type: String,
}

impl AuthBody {
    fn new(access_token: String) -> Self {
        Self {
            access_token,
            token_type: "Bearar".to_string(),
        }
    }
}

pub async fn login(
    State(pool): State<Pool<Sqlite>>,
    Json(payload): Json<LoginPayload>,
) -> Result<Json<AuthBody>, ApiError> {
    let wx_user = wx_login(payload.code).await?;

    let user = sqlx::query_as::<_, User>("select * from users where openid = ?")
        .bind(&wx_user.openid)
        .fetch_one(&pool)
        .await;

    let user = match user {
        Ok(user) => user,
        Err(sqlx::Error::RowNotFound) => {
            sqlx::query("insert into users(openid, session_key) values (?, ?)")
                .bind(&wx_user.openid)
                .bind(&wx_user.session_key)
                .execute(&pool)
                .await?;

            sqlx::query_as::<_, User>("select * from users where openid = ?")
                .bind(&wx_user.openid)
                .fetch_one(&pool)
                .await?
        }
        Err(e) => {
            let err = Err(ApiError::from(e));

            return err;
        }
    };

    let claims = Claims::new(user.id.to_string());

    let token = encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(b"secret"),
    )
    .map_err(|_| AuthError::TokenCreation)?;

    Ok(Json(AuthBody::new(token)))
}

pub async fn admin_register(
    State(pool): State<Pool<Sqlite>>,
    Json(payload): Json<AdminRegisterPayload>,
) -> Result<Json<()>, ApiError> {
    let user = sqlx::query_as::<_, AdminUser>(
        "select * from admin_users where phone = ? and password = ?",
    )
    .bind(&payload.phone)
    .bind(&payload.password)
    .fetch_one(&pool)
    .await;

    let _ = match user {
        Ok(_) => Err(ApiError::Auth(AuthError::TokenCreation)),
        Err(sqlx::Error::RowNotFound) => {
            sqlx::query("insert into admin_users(phone, password) values (?, ?)")
                .bind(&payload.phone)
                .bind(&payload.password)
                .execute(&pool)
                .await?;

            Ok(Json({}))
        }
        Err(e) => {
            let err = Err(ApiError::from(e));

            return err;
        }
    };

    Ok(Json({}))
}

#[derive(Serialize, Default)]
struct WxUser {
    pub openid: String,
    pub session_key: String,
}

async fn wx_login(code: String) -> Result<WxUser, ApiError> {
    Ok(WxUser::default())
}
