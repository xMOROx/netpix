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

## Analyzing WebRTC calls with Chrome event logs

WebRTC encrypts RTCP (SRTCP), so a packet capture shows only RTCP headers. Chrome can record a WebRTC event log with RTCP in plain form, and Netpix reads it with `-l`.

1. Record an event log in Chrome, before or during the call. Either:
   - open `chrome://webrtc-internals`, expand **Create diagnostic packet recordings**, check **Enable diagnostic packet and event recording** and choose a base file name, or
   - start Chrome with `--webrtc-event-logging=<existing directory>` (useful for automation).

   Chrome writes one file per peer connection, for example `event_log_20261112_1252_7_1.log`.

2. Start Netpix with the log files. A file that Chrome is still writing is read live as the call goes on.

```shell
netpix run -l ~/Downloads/event_log_20261112_1252_7_1.log
```

3. Open http://127.0.0.1:3550, pick the log in the source list and use the **RTCP Packets** and **RTCP Streams** views.
