use crate::models::IceServer;
use std::{
    collections::HashMap,
    env,
    io::{Error as IoError, ErrorKind, Result as IoResult},
    net::{IpAddr, SocketAddr},
    sync::Arc,
    time::Duration,
};
use tokio::net::UdpSocket;
use turn::{
    auth::{generate_auth_key, AuthHandler},
    relay::relay_static::RelayAddressGeneratorStatic,
    server::{
        config::{ConnConfig, ServerConfig},
        Server,
    },
    Error as TurnError,
};
use util::vnet::net::Net;

const TURN_REALM: &str = "p2p_clip_bridge_server";

#[derive(Clone, Debug)]
pub struct TurnConfig {
    bind: SocketAddr,
    public_ip: IpAddr,
    username: String,
    credential: String,
}

impl TurnConfig {
    pub fn from_sources(
        turn_port: u16,
        public_ip: Option<&str>,
        username: Option<&str>,
        credential: Option<&str>,
    ) -> IoResult<Self> {
        let bind = SocketAddr::new(IpAddr::V4(std::net::Ipv4Addr::UNSPECIFIED), turn_port);
        let public_ip = required_value(public_ip, "P2P_CLIP_BRIDGE_SERVER_TURN_PUBLIC_IP")?
            .parse::<IpAddr>()
            .map_err(|err| invalid_input(format!("invalid TURN public IP: {err}")))?;
        let username = required_value(username, "P2P_CLIP_BRIDGE_SERVER_TURN_USERNAME")?;
        let credential = required_value(credential, "P2P_CLIP_BRIDGE_SERVER_TURN_CREDENTIAL")?;

        Ok(Self {
            bind,
            public_ip,
            username,
            credential,
        })
    }

    pub fn ice_server(&self, request_host: &str) -> IceServer {
        IceServer {
            urls: format!("turn:{}:{}", turn_host(request_host), self.bind.port()),
            username: Some(self.username.clone()),
            credential: Some(self.credential.clone()),
        }
    }

    pub fn stun_server(&self, request_host: &str) -> IceServer {
        IceServer {
            urls: format!("stun:{}:{}", turn_host(request_host), self.bind.port()),
            username: None,
            credential: None,
        }
    }
}

pub async fn start(config: TurnConfig, debug: bool) -> IoResult<Server> {
    let conn = Arc::new(UdpSocket::bind(config.bind).await?);
    let local_addr = conn.local_addr()?;
    let mut credentials = HashMap::new();
    credentials.insert(
        config.username.clone(),
        generate_auth_key(&config.username, TURN_REALM, &config.credential),
    );

    let server = Server::new(ServerConfig {
        conn_configs: vec![ConnConfig {
            conn,
            relay_addr_generator: Box::new(RelayAddressGeneratorStatic {
                relay_address: config.public_ip,
                address: config.bind.ip().to_string(),
                net: Arc::new(Net::new(None)),
            }),
        }],
        realm: TURN_REALM.to_string(),
        auth_handler: Arc::new(StaticAuthHandler { credentials, debug }),
        channel_bind_timeout: Duration::from_secs(0),
        alloc_close_notify: None,
    })
    .await
    .map_err(turn_error)?;

    println!(
        "p2p_clip_bridge_server built-in TURN listening on udp://{} as turn:<current-host>:{}",
        local_addr,
        local_addr.port()
    );

    Ok(server)
}

struct StaticAuthHandler {
    credentials: HashMap<String, Vec<u8>>,
    debug: bool,
}

impl AuthHandler for StaticAuthHandler {
    fn auth_handle(
        &self,
        username: &str,
        _realm: &str,
        src_addr: SocketAddr,
    ) -> Result<Vec<u8>, TurnError> {
        let result = self
            .credentials
            .get(username)
            .cloned()
            .ok_or(TurnError::ErrFakeErr);
        if self.debug {
            println!(
                "debug turn auth username={} src={} accepted={}",
                username,
                src_addr,
                result.is_ok()
            );
        }
        result
    }
}

fn turn_host(request_host: &str) -> String {
    let host = request_host.trim();
    if let Some(end) = host.strip_prefix('[').and_then(|value| value.find(']')) {
        return format!("[{}]", &host[1..=end]);
    }

    host.split(':').next().unwrap_or(host).to_string()
}

fn required_value(cli_value: Option<&str>, env_name: &str) -> IoResult<String> {
    cli_value
        .map(ToOwned::to_owned)
        .or_else(|| env::var(env_name).ok())
        .and_then(trim_owned)
        .ok_or_else(|| invalid_input(format!("{env_name} is required")))
}

fn trim_owned(value: String) -> Option<String> {
    let trimmed = value.trim();
    (!trimmed.is_empty()).then(|| trimmed.to_string())
}

fn invalid_input(message: String) -> IoError {
    IoError::new(ErrorKind::InvalidInput, message)
}

fn turn_error(err: TurnError) -> IoError {
    IoError::other(err)
}

#[cfg(test)]
mod tests {
    use super::{turn_host, TurnConfig};

    #[test]
    fn derives_turn_host_from_http_host() {
        assert_eq!(turn_host("clip.example.com"), "clip.example.com");
        assert_eq!(turn_host("clip.example.com:7259"), "clip.example.com");
        assert_eq!(turn_host("127.0.0.1:7259"), "127.0.0.1");
        assert_eq!(turn_host("[::1]:7259"), "[::1]");
    }

    #[test]
    fn builds_turn_config_from_cli_values() {
        let config =
            TurnConfig::from_sources(3478, Some("127.0.0.1"), Some("user"), Some("pass")).unwrap();

        let ice_server = config.ice_server("127.0.0.1:7259");
        assert_eq!(ice_server.urls, "turn:127.0.0.1:3478");
        assert_eq!(ice_server.username.as_deref(), Some("user"));
        assert_eq!(ice_server.credential.as_deref(), Some("pass"));

        let stun_server = config.stun_server("127.0.0.1:7259");
        assert_eq!(stun_server.urls, "stun:127.0.0.1:3478");
        assert_eq!(stun_server.username, None);
        assert_eq!(stun_server.credential, None);
    }
}
