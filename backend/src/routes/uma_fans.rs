use axum::{http::StatusCode, response::IntoResponse, Json};

// TODO: Proper implementation
pub async fn uma_fans_list_handler() -> impl IntoResponse {
    const MESSAGE: &str = "GET /api/uma/fans";

    let json_response = serde_json::json!({
        "message": MESSAGE
    });

    Json(json_response)
}

// TODO: Proper implementation
pub async fn uma_fans_update_handler() -> impl IntoResponse {
    const MESSAGE: &str = "POST /api/uma/fans";

    let json_response = serde_json::json!({
        "message": MESSAGE
    });

    Json(json_response)
}
