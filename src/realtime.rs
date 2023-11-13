use axum::{
    extract::{
        ws::{Message, WebSocket},
        State, WebSocketUpgrade,
    },
    response::IntoResponse,
    routing::get,
    Router,
};

use crate::AppState;

pub fn router(state: AppState) -> Router {
    Router::new()
        .route("/", get(realtime_get))
        .with_state(state)
}

#[axum::debug_handler]
async fn realtime_get(
    ws: WebSocketUpgrade,
    State(state): State<AppState>,
) -> impl IntoResponse {
    ws.on_upgrade(|ws: WebSocket| async {
        realtime_cpu_stream(state, ws).await
    })
}

async fn realtime_cpu_stream(app_state: AppState, mut ws: WebSocket) {
    let mut rx = app_state.tx.subscribe();

    while let Ok(msg) = rx.recv().await {
        let payload = serde_json::to_string(&msg)
            .expect("failed to serialize socket payload");
        let _ = ws.send(Message::Text(payload)).await;
    }
}
