mod cli;
mod html;
mod models;
mod routes;
mod state;

use clap::Parser;
use cli::Args;
use state::AppState;

#[tokio::main]
async fn main() -> std::io::Result<()> {
    let args = Args::parse();

    let bind = args.bind;
    let state = AppState::new(args.ice_server);
    let app = routes::router(state);

    let listener = tokio::net::TcpListener::bind(bind).await?;
    println!("clip_bridge listening on http://{bind}");
    println!("open http://{bind}/your-password behind nginx/https in production");
    axum::serve(listener, app).await
}
