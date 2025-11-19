use clap::Parser;
use std::error::Error;
use std::net::SocketAddr;
use std::str::FromStr;
use tokio::net::UdpSocket;
use tun::AsyncDevice;
use tun::Error as TunError;

mod args;

use crate::args::Commands;
use crate::args::VPNArgs;

const VPN_IFACE_NAME: &str = "vpn-draft";
const VPN_TUN_MTU: u16 = 1500;
const VPN_SERVER_ENDPOINT: &str = "0.0.0.0:4242";
const UDP_MAXIMUM_PACKET_SIZE: u32 = 65536;

/// Create tun interface (AsyncDevice)
///
/// Default address range: addr/24
fn create_tun_interface(tun_name: &str, addr: &str) -> Result<AsyncDevice, TunError> {
    let mut config = tun::Configuration::default();
    config
        .tun_name(tun_name)
        .address(addr)
        .netmask((255, 255, 255, 0))
        .mtu(VPN_TUN_MTU)
        .up();

    #[cfg(target_os = "linux")]
    config.platform_config(|config| {
        // requiring root privilege to acquire complete functions
        config.ensure_root_privileges(true);
    });

    tun::create_as_async(&config)
}

async fn run_vpn(tun_addr: &str, remote_addr: Option<&str>) -> Result<(), Box<dyn Error>> {
    let mut tun_buf = [0; VPN_TUN_MTU as usize];
    let mut sock_buf = [0; UDP_MAXIMUM_PACKET_SIZE as usize];

    let tun = create_tun_interface(VPN_IFACE_NAME, tun_addr)?;
    let remote = UdpSocket::bind(VPN_SERVER_ENDPOINT).await?;

    let (mode, src_addr) = match remote_addr {
        Some(addr) => ("client", SocketAddr::from_str(addr)?),
        None => {
            let (_, src_addr) = remote.recv_from(&mut sock_buf).await?;
            ("server", src_addr)
        }
    };

    println!("Begin ...");

    loop {
        tokio::select! {
            _ = tun.recv(&mut tun_buf) => {
                println!("{mode}: tun.recv, transmitting to remote ...");
                remote.send_to(&tun_buf, src_addr).await?;
            },
            _ = remote.recv(&mut sock_buf) => {
                println!("{mode}: sock.recv, transmitting to local tun interface ...");
                tun.send(&sock_buf).await?;
            }
        }
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let args = VPNArgs::parse();
    println!("vpn-draft started ...");
    match args.command {
        Commands::Client(client) => run_vpn(&client.tun_addr, Some(&client.remote)).await,
        Commands::Server(server) => run_vpn(&server.tun_addr, None).await,
    }
}
