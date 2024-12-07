use axum::response::IntoResponse;
use axum::Json;

pub async fn health_check_handler() -> impl IntoResponse {
    const MESSAGE: &str = "Healthy 🙇‍♂️";

    let json_response = serde_json::json!({
        "status": "success",
        "message": MESSAGE
    });

    Json(json_response)
}
