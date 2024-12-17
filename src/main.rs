#![allow(dead_code)]
use clap::{Parser, Subcommand};

mod cmd;
mod server;
mod sniffer;

#[tokio::main]
async fn main() {
    pretty_env_logger::init();

    let cli = NetpixArgs::parse();
    cli.run().await;
}

#[derive(Debug, Parser)]
#[clap(version, about)]
struct NetpixArgs {
    #[clap(subcommand)]
    pub(crate) action: NetpixSubcommands,
}

impl NetpixArgs {
    pub async fn run(self) {
        match self.action {
            NetpixSubcommands::Run(inner) => inner.run().await,
            NetpixSubcommands::List(inner) => inner.run().await,
        }
    }
}

#[derive(Debug, Subcommand)]
enum NetpixSubcommands {
    /// Run the app. E.g "run -f rtp.pcap webex.pcap -i etn0 wireless". Obtain help with "run --help"
    Run(cmd::run::Run),

    /// List network interfaces
    List(cmd::list::List),
}
