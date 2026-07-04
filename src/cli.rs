use clap::Parser;
use std::net::SocketAddr;

#[derive(Parser, Debug)]
#[command(
    author,
    version,
    about = "Small password-in-URL clipboard and file relay server"
)]
pub struct Args {
    #[arg(long, default_value = "0.0.0.0:7259")]
    pub bind: SocketAddr,

    #[arg(
        long = "ice-server",
        value_name = "URL[,USERNAME,CREDENTIAL]",
        help = "Repeatable WebRTC ICE server. Example: --ice-server stun:stun.l.google.com:19302 --ice-server turns:turn.example.com:5349,user,pass"
    )]
    pub ice_server: Vec<String>,
}
