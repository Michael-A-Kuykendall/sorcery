use std::sync::Arc;

use axum::{extract::State, http::StatusCode, response::IntoResponse, Json};
use serde_json::json;

use crate::server::AppState;

pub async fn messages(State(_state): State<Arc<AppState>>) -> impl IntoResponse {
	(
		StatusCode::NOT_IMPLEMENTED,
		Json(json!({
			"error": {
				"message": "Anthropic compatibility is not implemented in this rehydration",
				"type": "not_implemented"
			}
		})),
	)
}
