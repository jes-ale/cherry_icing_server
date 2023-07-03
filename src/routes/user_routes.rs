use crate::handlers::user_handlers::{create_user, list_users};
use crate::routes::router::build_route;
use axum::{
    routing::{get, post},
    Router,
};
pub fn get_user_routes() -> Router {
    build_route("/user/list", get(list_users))
}
