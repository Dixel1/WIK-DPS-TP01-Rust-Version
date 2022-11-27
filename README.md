# RUST API 

## Installation guide

```bash 
mkdir -p ~/Rust_API && cd ~/Rust_API
```

```bash
git clone https://github.com/rthidfrev/WIK-DPS-TP01-Rust-Version.git
```

```bash
cd Simple-api-in-rust
```

```bash
cargo build --release
```

The api listen on port `8080` by default, you can change it by creating a new env variable named `PING_LISTEN_PORT` and set it to the port you want :

```
export PING_LISTEN_PORT=8081
```

## Usage

```bash
./target/release/cour_dev_ops
```

