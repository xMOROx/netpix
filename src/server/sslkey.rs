use std::{collections::HashMap, path::PathBuf, sync::Arc};

use log::{debug, info, warn};
use tokio::{
    fs::File,
    io::{AsyncBufReadExt, AsyncSeekExt, BufReader},
    sync::RwLock,
    time::{sleep, Duration},
};

use tokio::io::SeekFrom;

pub type SslKeyStore = Arc<RwLock<HashMap<String, String>>>;

pub fn new_store() -> SslKeyStore {
    Arc::new(RwLock::new(HashMap::new()))
}

/// Starts a background task that tails the provided path (Firefox/Chrome SSLKEYLOGFILE format)
/// and populates a shared in-memory store with new secrets.
///
/// Supported line formats (NSS key log):
/// CLIENT_RANDOM <client_random> <master_secret>
/// CLIENT_EARLY_TRAFFIC_SECRET <client_random> <secret>
/// CLIENT_HANDSHAKE_TRAFFIC_SECRET <client_random> <secret>
/// SERVER_HANDSHAKE_TRAFFIC_SECRET <client_random> <secret>
/// CLIENT_TRAFFIC_SECRET_0 <client_random> <secret>
/// SERVER_TRAFFIC_SECRET_0 <client_random> <secret>
/// EXPORTER_SECRET <client_random> <secret>
/// For simplicity we treat all as LABEL + ID + SECRET and store under key: "LABEL_ID".
pub fn start_ssl_key_watcher(path_opt: Option<String>) -> SslKeyStore {
    let store = new_store();
    if let Some(path_str) = path_opt {
        let path = PathBuf::from(path_str.clone());
        let store_cl = store.clone();
        tokio::spawn(async move {
            info!("SSL key watcher: starting for {:?}", path);
            let mut pos: u64 = 0;
            loop {
                match File::open(&path).await {
                    Ok(mut file) => {
                        match file.metadata().await {
                            Ok(meta) => {
                                let len = meta.len();
                                if pos > len {
                                    debug!(
                                        "SSL key watcher: file truncated or rotated (pos {} > len {}), resetting",
                                        pos, len
                                    );
                                    pos = 0;
                                }
                            }
                            Err(e) => warn!("SSL key watcher: metadata error: {:?}", e),
                        }

                        if let Err(e) = file.seek(SeekFrom::Start(pos)).await {
                            warn!("SSL key watcher: seek error: {:?}", e);
                            sleep(Duration::from_secs(1)).await;
                            continue;
                        }
                        let mut reader = BufReader::new(file);
                        let mut line = String::new();
                        loop {
                            match reader.read_line(&mut line).await {
                                Ok(0) => break,
                                Ok(n) => {
                                    pos += n as u64;
                                    if let Err(e) = process_line(line.trim_end(), &store_cl).await {
                                        debug!("SSL key watcher: skipping line due to error: {}", e);
                                    }
                                    line.clear();
                                }
                                Err(e) => {
                                    warn!("SSL key watcher: read_line error: {:?}", e);
                                    break;
                                }
                            }
                        }
                    }
                    Err(e) => debug!("SSL key watcher: cannot open file {:?}: {:?}", path, e),
                }
                sleep(Duration::from_secs(1)).await;
            }
        });
    } else {
        info!("SSLKEYLOGFILE not set; SSL key watcher not started");
    }
    store
}

async fn process_line(line: &str, store: &SslKeyStore) -> Result<(), &'static str> {
    if line.is_empty() || line.starts_with('#') {
        return Ok(());
    }
    let parts: Vec<&str> = line.split_whitespace().collect();
    if parts.len() < 3 {
        return Err("unsupported line format");
    }
    let label = parts[0];
    let id = parts[1];
    let secret = parts[2];
    let key = format!("{}_{}", label, id);
    {
        let mut map = store.write().await;
        if map.contains_key(&key) {
            return Ok(());
        }
        map.insert(key.clone(), secret.to_string());
    }
    info!("SSL key watcher: added key {}", key);
    Ok(())
}

pub async fn get_secret(store: &SslKeyStore, label: &str, id: &str) -> Option<String> {
    let map = store.read().await;
    map.get(&format!("{}_{}", label, id)).cloned()
}
