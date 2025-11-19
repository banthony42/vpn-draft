use clap::Args;
use clap::Parser;
use clap::Subcommand;

#[derive(Args)]
pub struct ClientCommand {
    /// VPN client local tunnel IPv4 address
    #[clap(long)]
    pub tun_addr: String,

    /// VPN remote server IPv4 address.
    /// (default remote port 4242 will be use automatically)
    #[clap(long)]
    pub remote: String,
}

#[derive(Args)]
pub struct ServerCommand {
    /// VPN server local tunnel IPv4 address
    #[clap(long)]
    pub tun_addr: String,
}

#[derive(Subcommand)]
pub enum Commands {
    /// Run VPN client instance
    Client(ClientCommand),

    /// Run VPN server instance
    Server(ServerCommand),
}

#[derive(Parser)]
pub struct VPNArgs {
    #[clap(subcommand)]
    pub command: Commands,
}
