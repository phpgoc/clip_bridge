use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ServerConfig {
    pub ice_servers: Vec<IceServer>,
}

#[derive(Clone, Serialize)]
pub struct IceServer {
    pub urls: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub username: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub credential: Option<String>,
}

#[derive(Debug, Deserialize)]
#[serde(tag = "kind", rename_all = "snake_case")]
pub enum ClientMessage {
    Join { room: String, peer_id: String },
    Signal { to: String, data: Value },
}

#[derive(Clone, Serialize)]
#[serde(tag = "kind", rename_all = "snake_case")]
pub enum ServerMessage {
    Peers { peers: Vec<String> },
    PeerJoined { peer_id: String },
    PeerLeft { peer_id: String },
    Signal { from: String, data: Value },
    Error { message: String },
}
