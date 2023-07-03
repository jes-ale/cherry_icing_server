use crate::handlers::internal_error;
use crate::models::user_models::{PayloadUser, User};
use crate::models::{DatabaseConnection, Pool};
use crate::schema::user;
use axum::{extract::State, http::StatusCode, response::Json};
use diesel::prelude::*;
use diesel_async::RunQueryDsl;

pub async fn create_user(
    State(pool): State<Pool>,
    Json(payload_user): Json<PayloadUser>,
) -> Result<Json<User>, (StatusCode, String)> {
    let mut conn = pool.get().await.map_err(internal_error)?;

    let res = diesel::insert_into(user::table)
        .values(payload_user)
        .returning(User::as_returning())
        .get_result(&mut conn)
        .await
        .map_err(internal_error)?;
    Ok(Json(res))
}

pub async fn list_users(
    DatabaseConnection(mut conn): DatabaseConnection,
) -> Result<Json<Vec<User>>, (StatusCode, String)> {
    let res = user::table
        .select(User::as_select())
        .load(&mut conn)
        .await
        .map_err(internal_error)?;
    Ok(Json(res))
}
