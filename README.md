# Netpix

![crates.io](https://img.shields.io/crates/v/netpix)

RTP/MPEG-TS streams analysis and visualization tool.

_Work in progress..._

## Installation

1. Install Rust
```shell
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

Supports Linux and MacOS.

2. Netpix depends on `libpcap`, make sure to install it:

```shell
# installed on MacOS by default

# for Ubuntu
sudo apt install libpcap-dev

# if error appears due to lack of linter `cc` 
sudo apt install build-essential

# for Arch
sudo pacman -S libpcap
```

3. Install netpix using the [Rust toolchain](https://www.rust-lang.org/tools/install):

```shell
cargo install --locked netpix
```

4. Run Netpix:

```shell
netpix --help
```

## Performance Tuning

Netpix provides several configuration options to optimize performance and resource usage for your specific use case:

### Connection and Memory Limits

- `--max-clients` (default: 100): Maximum number of concurrent WebSocket connections. Adjust based on your server capacity.
- `--max-client-queue-size` (default: 1000): Maximum number of messages that can be queued per client. When this limit is reached, older messages are dropped to prevent memory exhaustion.
- `--buffer-size` (default: 32,768): Number of packets the server can store in memory before discarding old packets.

### Message Delivery

- `--message-interval` (default: 5ms): Interval between sending messages to clients. Lower values provide more real-time updates but increase CPU usage. Higher values reduce CPU usage but may cause lag in the UI.
- `--message-batch-size` (default: 10): Number of messages to send per client per interval. Higher values improve throughput for high-traffic scenarios but may increase latency for individual messages.

### Packet Management

- `--maximum-package-age` (default: 300s): Maximum age of packets before they are automatically removed from the buffer.

### Example: High-Performance Configuration

For a high-traffic environment with many concurrent users:

```shell
netpix run -i eth0 \
  --max-clients 200 \
  --max-client-queue-size 2000 \
  --buffer-size 65536 \
  --message-interval 10 \
  --message-batch-size 20
```

### Example: Low-Latency Configuration

For real-time analysis with minimal delay:

```shell
netpix run -i eth0 \
  --max-clients 50 \
  --message-interval 2 \
  --message-batch-size 5
```

### Example: Low-Resource Configuration

For environments with limited memory:

```shell
netpix run -i eth0 \
  --max-clients 50 \
  --max-client-queue-size 500 \
  --buffer-size 16384 \
  --message-interval 20 \
  --message-batch-size 15
```

### Monitoring

Watch for these warning messages in logs:
- `"Client queue full, dropping message"` - Consider increasing `--max-client-queue-size` or `--message-interval`
- `"Maximum client limit reached"` - Increase `--max-clients` if you need to support more concurrent users
- `"Packet buffer full, discarding oldest packet"` - Increase `--buffer-size` to retain more packet history

### Performance Recommendations

**Throughput vs Latency Trade-off:**
- For maximum throughput: Increase `--message-batch-size` and `--message-interval`
- For minimum latency: Decrease `--message-interval` and `--message-batch-size`

**Memory Management:**
- The total memory usage is approximately: `buffer-size * avg_packet_size + (max-clients * max-client-queue-size * avg_message_size)`
- Typical packet size: 1-2 KB
- Typical message size: 1-2 KB

**CPU Usage:**
- Lower `--message-interval` increases CPU usage but provides more real-time updates
- Higher `--message-batch-size` reduces the overhead of acquiring locks per message

