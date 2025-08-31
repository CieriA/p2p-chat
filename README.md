# P2P Chat
A simple, non-encrypted peer-to-peer chat written in Rust.

## Features
- `open` a new room
- `join` an existing room using a ticket
- communicate with all peers in the room.

## Requirements
- **Rust** (stable) — install via [rustup](https://rustup.rs)

## Building the project
Clone the repository and build it in release mode:
```bash
git clone https://github.com/CieriA/p2p-chat
cd p2p-chat
cargo build --release
```

## Running the project
To open a new room:
```bash
cargo run --release open
```
To join a room:
```bash
cargo run --release join <TICKET>
```

## Docs
To build the documentation locally:
```bash
cargo doc --open
```

## License
This project is licensed under the ISC License. For more info, see the [LICENSE](LICENSE) file.

