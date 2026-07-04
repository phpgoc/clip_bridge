use crate::models::IceServer;

/// Public STUN servers used before the built-in TURN relay.
///
/// Browsers may contact multiple ICE servers while gathering candidates. Slow or
/// unreachable STUN servers can delay candidate gathering on some networks, so
/// downstream builders can tune this list for their region.
pub const PUBLIC_STUN_SERVERS: &[&str] = &[
    "stun:stun.miwifi.com:3478",
    "stun:stun.chat.bilibili.com:3478",
    "stun:stun.l.google.com:19302",
];

pub fn public_stun_servers() -> Vec<IceServer> {
    PUBLIC_STUN_SERVERS
        .iter()
        .map(|urls| IceServer {
            urls: (*urls).to_string(),
            username: None,
            credential: None,
        })
        .collect()
}
