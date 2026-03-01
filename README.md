# Replayer
<img width="200px" src="https://github.com/user-attachments/assets/aee18132-9a95-43ef-9ac6-2103816e4d01" />
<hr/>
A decentralized retro game marketplace and emulation platform built on Solana. Developers publish encrypted CHIP-8 games on-chain, players purchase them as NFTs, and play directly in the browser.

## How It Works

1. **Publish** — Developers register, upload a game ROM and cover image. The backend encrypts the game data with AES-GCM, the cover image goes to IPFS via Bundlr, and the encrypted ROM is stored on-chain in chunks.
2. **Buy** — Players browse available games, purchase with SOL. An NFT is minted to their wallet proving ownership, and revenue is split between the developer and the platform treasury.
3. **Play** — Players sign an authentication message, the backend verifies NFT ownership and returns the decryption key. The game is fetched from-chain, decrypted client-side, and loaded into a CHIP-8 emulator running in the browser via WebAssembly.

## Tech Stack

- **Frontend**: [Leptos](https://leptos.dev/) (Rust full-stack framework) compiled to WebAssembly
- **Server**: Axum with server-side rendering
- **Blockchain**: Solana (devnet) — game storage, payments, NFT minting
- **Storage**: Bundlr for game cover images
- **Encryption**: AES-GCM for game data protection
- **Wallet**: Phantom browser extension
- **UI Components**: Thaw
- **Styling**: SCSS
- **Client Generator**: Codama Rust
## Screens

<img width="1000"  alt="image" src="https://github.com/user-attachments/assets/3473855e-2ada-4b4c-a2ce-0b6710ffad39" />
<hr/>
<img width="1000"  alt="image" src="https://github.com/user-attachments/assets/878b3328-55ce-4e4b-8e07-2192c360df8b" />
<hr/>
<img width="2278" height="1161" alt="image" src="https://github.com/user-attachments/assets/f989d07b-d8c6-4913-af77-4d633b399ede" />
<hr/>
<img width="1000"  alt="image" src="https://github.com/user-attachments/assets/624a7aa5-2854-4fbc-a69d-d35c68f2e44e" />
<hr/>
<img width="2278" height="1161" alt="image" src="https://github.com/user-attachments/assets/2f3f7c52-6159-4235-9cfd-e2af48e6065b" />
<hr/>
## Project Structure

```
src/
├── app/           # App shell, routing, context providers
├── components/    # Reusable UI (game cards, upload form, nav, etc.)
├── config/        # Runtime configuration (TOML)
├── generated/     # Solana program types (accounts, instructions)
├── models/        # Data models (encryption)
├── pages/         # Page components (home, buy, play, publish, admin)
├── server/        # Server-side logic (API client, transaction builders, queries)
├── utils/         # Constants, deserializers
├── vm/            # CHIP-8 emulator (core + WASM bindings)
└── wallet/        # Phantom wallet integration (signing, transactions)
```

## Prerequisites

- [Rust](https://rustup.rs/) (edition 2021)
- [cargo-leptos](https://github.com/leptos-rs/cargo-leptos): `cargo install cargo-leptos`
- `wasm32-unknown-unknown` target: `rustup target add wasm32-unknown-unknown`
- A running instance of the Replayer backend API (default: `http://127.0.0.1:3003`)
- [Phantom wallet](https://phantom.app/) browser extension


## Configuration

Edit `config/config.toml`:

```toml
[app]
backend_url = "http://127.0.0.1:3003"
backend_signer = "<backend-public-key>"

[solana]
rpc_url = "<devnet-rpc-url>"
program_id = "<program-id>"
bundlr_url = "https://devnet.bundlr.network/"
bundlr_keypair = [...]
```

## Running

```bash
cargo leptos watch
```

The app starts at `http://127.0.0.1:3005` with hot-reload on port 3001.

## Building for Production

```bash
cargo leptos build --release
```

Output is in `target/site/`. The WASM bundle uses the `wasm-release` profile with aggressive optimizations (LTO, `opt-level = 'z'`).

## Related Repositories

This is part of the **Replayer** project. Check out the other repos:

- [replayer-be](https://github.com/vlady-kotsev/replayer-be) — Backend API
- [replayer-program](https://github.com/vlady-kotsev/replayer-program) — Solana program

