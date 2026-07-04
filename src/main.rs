mod cli;
mod html;
mod ice;
mod models;
mod routes;
mod state;
mod turn_relay;

use clap::{CommandFactory, Parser};
use cli::Args;
use state::AppState;
use std::net::{IpAddr, Ipv4Addr, SocketAddr};
use turn_relay::TurnConfig;

#[tokio::main]
async fn main() -> std::io::Result<()> {
    let args = Args::parse();

    let bind = SocketAddr::new(IpAddr::V4(Ipv4Addr::UNSPECIFIED), args.port);
    let turn_config = match TurnConfig::from_sources(
        args.turn_port,
        args.turn_public_ip.as_deref(),
        args.turn_username.as_deref(),
        args.turn_credential.as_deref(),
    ) {
        Ok(config) => config,
        Err(err) if err.kind() == std::io::ErrorKind::InvalidInput => {
            eprintln!("error: {err}\n");
            let _ = Args::command().print_help();
            eprintln!();
            return Err(err);
        }
        Err(err) => return Err(err),
    };
    if args.debug {
        println!("debug logging enabled; clipboard text and file bytes are not logged");
    }
    let _turn_server = turn_relay::start(turn_config.clone(), args.debug).await?;
    let state = AppState::new(turn_config, args.debug);
    let app = routes::router(state);

    let listener = tokio::net::TcpListener::bind(bind).await?;
    println!("p2p_clip_bridge_server listening on http://{bind}");
    println!(
        "open http://127.0.0.1:{}/your-password locally, or use nginx/https in production",
        args.port
    );
    axum::serve(listener, app).await
}
