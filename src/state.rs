use crate::models::{IceServer, ServerConfig, ServerMessage};
use std::{collections::HashMap, sync::Arc};
use tokio::sync::{mpsc, RwLock};

#[derive(Clone)]
pub struct AppState {
    rooms: Arc<RwLock<HashMap<String, Room>>>,
    config: ServerConfig,
}

#[derive(Default)]
struct Room {
    peers: HashMap<String, mpsc::UnboundedSender<ServerMessage>>,
}

impl AppState {
    pub fn new(ice_server_args: Vec<String>) -> Self {
        Self {
            rooms: Arc::new(RwLock::new(HashMap::new())),
            config: ServerConfig {
                ice_servers: parse_ice_servers(ice_server_args),
            },
        }
    }

    pub fn config(&self) -> ServerConfig {
        self.config.clone()
    }

    pub async fn join(
        &self,
        room_name: &str,
        peer_id: &str,
        tx: mpsc::UnboundedSender<ServerMessage>,
    ) -> Vec<String> {
        let mut rooms = self.rooms.write().await;
        let room = rooms.entry(room_name.to_string()).or_default();
        let peers = room
            .peers
            .keys()
            .filter(|id| id.as_str() != peer_id)
            .cloned()
            .collect::<Vec<_>>();

        room.peers.insert(peer_id.to_string(), tx);
        for (id, peer_tx) in &room.peers {
            if id != peer_id {
                let _ = peer_tx.send(ServerMessage::PeerJoined {
                    peer_id: peer_id.to_string(),
                });
            }
        }

        peers
    }

    pub async fn leave(&self, room_name: &str, peer_id: &str) {
        let mut rooms = self.rooms.write().await;
        let Some(room) = rooms.get_mut(room_name) else {
            return;
        };

        room.peers.remove(peer_id);
        for peer_tx in room.peers.values() {
            let _ = peer_tx.send(ServerMessage::PeerLeft {
                peer_id: peer_id.to_string(),
            });
        }

        if room.peers.is_empty() {
            rooms.remove(room_name);
        }
    }

    pub async fn forward_signal(
        &self,
        room_name: &str,
        from: &str,
        to: &str,
        data: serde_json::Value,
    ) {
        let rooms = self.rooms.read().await;
        let Some(room) = rooms.get(room_name) else {
            return;
        };
        let Some(peer_tx) = room.peers.get(to) else {
            return;
        };

        let _ = peer_tx.send(ServerMessage::Signal {
            from: from.to_string(),
            data,
        });
    }
}

fn parse_ice_servers(args: Vec<String>) -> Vec<IceServer> {
    let args = if args.is_empty() {
        vec!["stun:stun.l.google.com:19302".to_string()]
    } else {
        args
    };

    args.into_iter()
        .filter_map(|arg| {
            let mut parts = arg.splitn(3, ',').map(str::trim);
            let urls = parts.next()?.to_string();
            if urls.is_empty() {
                return None;
            }

            let username = parts
                .next()
                .filter(|value| !value.is_empty())
                .map(ToOwned::to_owned);
            let credential = parts
                .next()
                .filter(|value| !value.is_empty())
                .map(ToOwned::to_owned);

            Some(IceServer {
                urls,
                username,
                credential,
            })
        })
        .collect()
}
