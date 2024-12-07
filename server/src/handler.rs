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

#[cfg(test)]
mod tests {
    use super::*;
    use axum::{
        response::Response,
        body::to_bytes,
    };
    use http::StatusCode;
    use serde_json::Value;

    #[tokio::test]
    async fn test_health_check_response() {
        let response: Response = health_check_handler().await.into_response();
        assert_eq!(response.status(), StatusCode::OK);

        let body = to_bytes(response.into_body(), 1024).await.unwrap();
        let json: Value = serde_json::from_slice(&body).unwrap();
        assert_eq!(json["status"], "success");
        assert_eq!(json["message"], "Healthy 🙇‍♂️");
    }
}