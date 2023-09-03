// Copyright (c) 2022-2023 Yuki Kishimoto
// Distributed under the MIT software license

mod config;
mod logger;
mod tcp;

use self::config::{Args, Parser};
use self::logger::Logger;
use self::tcp::TcpReverseProxy;
use arti_client::{TorClient, TorClientConfig};

type Result<T, E = Box<dyn std::error::Error>> = std::result::Result<T, E>;

#[tokio::main]
async fn main() -> Result<()> {
    Logger::init();

    let args: Args = Args::parse();

    let mut reverse_proxy = TcpReverseProxy::new(args.local_addr, args.forward_addr);

    #[cfg(feature = "tor")]
    if args.use_tor {
        let config = TorClientConfig::default();
        let tor_client = TorClient::create_bootstrapped(config).await?;
        reverse_proxy = reverse_proxy.tor(tor_client);
    }

    reverse_proxy.socks5_proxy(args.socks5_proxy).run().await
}
