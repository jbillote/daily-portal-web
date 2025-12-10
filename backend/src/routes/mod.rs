mod uma_fans;

use axum::{routing::get, Router};

pub fn create_router() -> Router {
    Router::new().route("/api/uma/fans", get(uma_fans::uma_fans_list_handler))
}
