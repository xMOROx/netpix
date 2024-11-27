# Netpix

![crates.io](https://img.shields.io/crates/v/netpix)

RTP/MPEG-TS streams analysis and visualization tool.

_Work in progress..._

## Installation

Supports Linux and MacOS.

1. Netpix depends on `libpcap`, make sure to install it:

```shell
# installed on MacOS by default

# for Ubuntu
sudo apt install libpcap-dev

# if error appears due to lack of linter `cc` 
sudo apt install build-essential

# for Arch
sudo pacman -S libpcap
```

2. Install netpix using the [Rust toolchain](https://www.rust-lang.org/tools/install):

```shell
cargo install --locked netpix
```

3. Run Netpix:

```shell
netpix --help
```
