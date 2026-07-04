use crate::{
    html::{APP_HTML, ROOT_HTML},
    models::{ClientMessage, ServerConfig, ServerMessage},
    state::AppState,
};
use axum::{
    extract::{
        ws::{Message, WebSocket, WebSocketUpgrade},
        State,
    },
    http::HeaderMap,
    response::Html,
    routing::get,
    Json, Router,
};
use futures_util::{SinkExt, StreamExt};
use tokio::sync::mpsc;

pub fn router(state: AppState) -> Router {
    Router::new()
        .route("/", get(root))
        .route("/clip_bridge_server/config", get(server_config))
        .route("/clip_bridge_server", get(server_ws))
        .fallback(get(app_page))
        .with_state(state)
}

async fn root() -> Html<&'static str> {
    Html(ROOT_HTML)
}

async fn app_page() -> Html<&'static str> {
    Html(APP_HTML)
}

async fn server_config(State(state): State<AppState>, headers: HeaderMap) -> Json<ServerConfig> {
    let host = headers
        .get(axum::http::header::HOST)
        .and_then(|value| value.to_str().ok())
        .unwrap_or("localhost");
    if state.debug() {
        println!("debug http config host={host}");
    }
    Json(state.config(host))
}

async fn server_ws(
    State(state): State<AppState>,
    ws: WebSocketUpgrade,
) -> impl axum::response::IntoResponse {
    ws.on_upgrade(move |socket| signaling_peer(socket, state))
}

async fn signaling_peer(socket: WebSocket, state: AppState) {
    let (mut sender, mut receiver) = socket.split();
    let (tx, mut rx) = mpsc::unbounded_channel::<ServerMessage>();
    let mut joined_room: Option<String> = None;
    let mut peer_id: Option<String> = None;

    loop {
        tokio::select! {
            from_client = receiver.next() => {
                let Some(Ok(Message::Text(raw))) = from_client else {
                    break;
                };

                let Ok(message) = serde_json::from_str::<ClientMessage>(&raw) else {
                    let _ = tx.send(ServerMessage::Error {
                        message: "bad signaling message".to_string(),
                    });
                    continue;
                };

                match message {
                    ClientMessage::Join { room, peer_id: incoming_peer_id } => {
                        if joined_room.is_some() {
                            continue;
                        }
                        let peers = state.join(&room, &incoming_peer_id, tx.clone()).await;
                        joined_room = Some(room);
                        peer_id = Some(incoming_peer_id);
                        let _ = tx.send(ServerMessage::Peers { peers });
                    }
                    ClientMessage::Signal { to, data } => {
                        let (Some(room), Some(from)) = (&joined_room, &peer_id) else {
                            continue;
                        };
                        if state.debug() {
                            println!(
                                "debug ws signal room={} from={} to={} type={}",
                                room,
                                from,
                                to,
                                signal_type(&data)
                            );
                        }
                        state.forward_signal(room, from, &to, data).await;
                    }
                }
            }
            from_room = rx.recv() => {
                let Some(message) = from_room else {
                    break;
                };
                let body = serde_json::to_string(&message).unwrap_or_else(|_| "{}".to_string());
                if sender.send(Message::Text(body.into())).await.is_err() {
                    break;
                }
            }
        }
    }

    if let (Some(room), Some(peer)) = (joined_room, peer_id) {
        state.leave(&room, &peer).await;
    }
}

fn signal_type(data: &serde_json::Value) -> &str {
    data.get("type")
        .and_then(|value| value.as_str())
        .unwrap_or("unknown")
}
