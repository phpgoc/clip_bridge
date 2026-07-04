use clap::Parser;

#[derive(Parser, Debug)]
#[command(
    author,
    version,
    about = "Small password-in-URL clipboard and file relay server"
)]
pub struct Args {
    #[arg(short = 'p', long, default_value_t = 7259)]
    pub port: u16,

    #[arg(short = 't', long = "turn-port", default_value_t = 3478)]
    pub turn_port: u16,

    #[arg(short = 'i', long = "ip", value_name = "PUBLIC_IP")]
    pub turn_public_ip: Option<String>,

    #[arg(short = 'u', long = "username", value_name = "USERNAME")]
    pub turn_username: Option<String>,

    #[arg(short = 'c', long = "credential", value_name = "PASSWORD")]
    pub turn_credential: Option<String>,
}
