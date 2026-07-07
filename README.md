# Ethernaut Writeup

This repository collects my Ethernaut challenge solutions and reproduction notes.

## Structure

- `Ethernaut_1-25/`: Rust-based solutions for the earlier Ethernaut levels, built with `alloy` and `tokio`.
- `Ethernaut_26-32/`: Foundry-based reproductions and exploit scripts for the later levels.

## Usage

```sh
git clone --recurse-submodules https://github.com/LameCrow/Ethernaut-Writeup.git
cd Ethernaut-Writeup
```

Run the Rust project:

```sh
cd Ethernaut_1-25
cargo run
```

Run the Foundry project:

```sh
cd Ethernaut_26-32
forge build
forge test
```

Some scripts target Sepolia deployments and may require RPC/private key environment variables before execution.
