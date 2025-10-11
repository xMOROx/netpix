use crate::server;
use crate::server::config::Config;
use crate::sniffer::{Error, Sniffer};
use std::collections::HashMap;
use std::net::{IpAddr, Ipv4Addr, SocketAddr};

const DEFAULT_PORT: u16 = 3550;
const DEFAULT_IP: IpAddr = IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1));
const DEFAULT_PROMISC: bool = false;
const DEFAULT_PACKET_BUFFER_SIZE: usize = 32_768;
const DEFAULT_MAXIMUM_PACKAGE_AGE: u64 = 300;
const DEFAULT_CLIENT_MESSAGE_INTERVAL_MS: u64 = 5; // ~ 300 messages per second

#[derive(Debug, clap::Args)]
pub struct Run {
    /// Pcap files to capture the packets from
    #[arg(short, long, num_args = 1..)]
    files: Vec<String>,
    /// Network interfaces to capture the packets from
    #[arg(short, long, num_args = 1..)]
    interfaces: Vec<String>,
    /// rtc_event_log files to capture packets from
    #[arg(short = 'l', long = "log-files", num_args = 1..)]
    log_files: Vec<String>,
    /// Capture filter string in Wireshark/tcpdump syntax, applies to all sources
    #[arg(short, long, default_value_t = String::new())]
    capture: String,
    /// IP address used by the application
    #[arg(short, long, default_value_t = DEFAULT_IP)]
    address: IpAddr,
    /// Port used by the application
    #[arg(short = 'p', long, default_value_t = DEFAULT_PORT)]
    port: u16,
    /// Enable promiscuous mode
    #[arg(short = 'P', long, default_value_t = DEFAULT_PROMISC)]
    promisc: bool,
    /// Set packet buffer size -> number of packets that server is able to store without
    /// discharging it
    #[arg(short, long, default_value_t = DEFAULT_PACKET_BUFFER_SIZE)]
    buffer_size: usize,
    /// Interval in milliseconds between client messages
    #[arg(short='m', long, default_value_t = DEFAULT_CLIENT_MESSAGE_INTERVAL_MS)]
    message_interval: u64,
    /// Maximum age of a package in seconds before it is considered outdated
    #[arg(short='M', long, default_value_t = DEFAULT_MAXIMUM_PACKAGE_AGE)]
    maximum_package_age: u64,
}

impl Run {
    pub async fn run(self) {
        if self.interfaces.is_empty() && !self.files.is_empty() && self.promisc {
            println!("Error: promiscuous mode cannot be used with file captures only");
            return;
        }

        let live_filter = self.create_capture_filter();

        let mut file_sniffers = get_sniffers(self.files, Sniffer::from_file);
        let mut interface_sniffers = get_sniffers(self.interfaces, |dev| {
            Sniffer::from_device(dev, self.promisc)
        });
        let log_sniffers = get_sniffers(self.log_files, Sniffer::from_logs);

        let file_res = apply_filters(&mut file_sniffers, &self.capture);
        let interface_res = apply_filters(&mut interface_sniffers, &live_filter);

        if file_res.is_err() || interface_res.is_err() {
            println!("Error: provided capture filter is invalid");
            return;
        }

        let sniffers: HashMap<_, _> = file_sniffers
            .into_iter()
            .chain(interface_sniffers)
            .chain(log_sniffers)
            .collect();

        if sniffers.is_empty() {
            // TODO: use some pretty printing (colors, bold font etc.)
            println!("Error: no valid sources were passed");
            return;
        }

        let address = SocketAddr::new(self.address, self.port);
        let config = Config::builder()
            .client_message_interval_ms(self.message_interval)
            .max_packets_age(self.maximum_package_age)
            .packet_buffer_size(self.buffer_size)
            .addr(address)
            .build();

        server::run(sniffers, config).await;
    }

    fn create_capture_filter(&self) -> String {
        // to filter out RTPeeker own WebSocket/HTTP messages
        let own_filter = if self.address.is_unspecified() {
            format!("not port {}", self.port)
        } else {
            format!("not (host {} and port {})", self.address, self.port)
        };

        if self.capture.is_empty() {
            own_filter
        } else {
            format!("({}) and ({})", own_filter, self.capture)
        }
    }
}

fn get_sniffers<F>(mut sources: Vec<String>, get_sniffer: F) -> HashMap<String, Sniffer>
where
    F: Fn(&str) -> Result<Sniffer, Error>,
{
    sources.sort_unstable();
    sources.dedup();
    sources
        .into_iter()
        .filter_map(|source| match get_sniffer(&source) {
            Ok(sniffer) => Some((source, sniffer)),
            Err(err) => {
                println!(
                    "Failed to capture packets from source {}, reason: {:?}",
                    source, err
                );
                None
            }
        })
        .collect()
}

fn apply_filters(sniffers: &mut HashMap<String, Sniffer>, filter: &str) -> Result<(), Error> {
    for (_, sniffer) in sniffers.iter_mut() {
        if let err @ Err(_) = sniffer.apply_filter(filter) {
            return err;
        }
    }

    Ok(())
}
