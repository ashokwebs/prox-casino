<p align="center">
  <pre align="center">
    ┏━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━┓
    ┃                                               ┃
    ┃     ██████╗ ██████╗  ██████╗ ██╗  ██╗         ┃
    ┃     ██╔══██╗██╔══██╗██╔═══██╗╚██╗██╔╝         ┃
    ┃     ██████╔╝██████╔╝██║   ██║ ╚███╔╝          ┃
    ┃     ██╔═══╝ ██╔══██╗██║   ██║ ██╔██╗          ┃
    ┃     ██║     ██║  ██║╚██████╔╝██╔╝ ██╗         ┃
    ┃     ╚═╝     ╚═╝  ╚═╝ ╚═════╝╚═╝  ╚═╝         ┃
    ┃              C A S I N O                       ┃
    ┃                                               ┃
    ┗━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━┛
  </pre>
</p>

<p align="center">
  <strong>A terminal-native casino experience built entirely in Rust.</strong><br>
  Blackjack · Slots · Persistent Stats · Offline-First
</p>

<p align="center">
  <img src="https://img.shields.io/badge/rust-1.75%2B-orange?logo=rust" alt="Rust">
  <img src="https://img.shields.io/badge/terminal-TUI-blueviolet?logo=windowsterminal" alt="TUI">
  <img src="https://img.shields.io/badge/license-MIT-green" alt="License">
  <img src="https://img.shields.io/badge/status-offline--first-crimson" alt="Status">
</p>

---

## Overview

**PROX CASINO** is a fully offline, terminal-based casino simulator. No servers, no accounts, no real money — just pure gameplay in your terminal. Built with [ratatui](https://github.com/ratatui-org/ratatui) for a rich, responsive TUI experience.

### Why?

- 🎰 **Play anywhere** — works offline, no internet needed
- 💾 **Your data, your machine** — saves locally via atomic JSON writes
- ⚡ **Instant startup** — sub-100ms launch, native Rust performance
- 🎨 **Beautiful TUI** — hand-crafted ASCII cards, animated slot reels, themed UI

---

## Games

### ♠ Blackjack

Full-featured blackjack with professional card rendering:

- **Hit, Stand, Double Down, Split** — all core mechanics
- **Dealer AI** with soft-17 rules
- **Hand-crafted ASCII cards** with suit symbols and rank corners
- **Adjustable bets** with configurable increments
- **Detailed stats** — win/loss streaks, bust count, biggest wins

### 🎰 Slots

Multiple slot machine types with weighted reels:

- **3-reel and 5-reel** machines
- **Elite VIP** high-risk/high-reward slots
- **7 symbol types** — Cherry, Lemon, Bell, Seven, Diamond, Wild, Scatter
- **Progressive jackpots** — Mini, Mega, and Ultra tiers
- **Auto-spin** mode (10 spins)
- **Massive ASCII art reels** with dynamic pulsing animations

### 🌐 Online Multiplayer (Coming Soon)

We are currently building the **PROX CASINO Online Engine**. Soon, you'll be able to:
- **Play with friends** in real-time private tables
- **Global leaderboards** for biggest payouts and longest streaks
- **Live jackpots** shared across the entire community

Stay tuned for the v1.0 release!

---

## Quick Start

### Prerequisites

- [Rust](https://rustup.rs/) 1.75 or later

### Run from source

```bash
git clone https://github.com/ashokwebs/prox-casino.git
cd prox-casino
cargo run --release
```

### Install globally

```bash
cargo build --release
cp target/release/prox-casino ~/.local/bin/casino
```

Then just type `casino` from anywhere.

### Debug mode

```bash
RUST_LOG=info cargo run
```

---

## Controls

### Navigation

| Key | Action |
|-----|--------|
| `1` | Dashboard |
| `2` | Blackjack |
| `3` | Slots |
| `4` | Online *(placeholder)* |
| `q` | Quit & save |

### Blackjack

| Key | Action |
|-----|--------|
| `←` `→` | Adjust bet |
| `↑` `↓` | ±10K chips |
| `Space` | Deal |
| `H` | Hit |
| `S` | Stand |
| `D` | Double down |
| `P` | Split |
| `R` | Rules |

### Slots

| Key | Action |
|-----|--------|
| `←` `→` | Adjust bet |
| `Space` | Spin |
| `A` | Auto-spin (10) |
| `M` | Change machine |

---

## Architecture

```
src/
├── app/           # Application state machine & event handling
├── core/          # Config constants (tick rate, starting chips)
├── games/
│   ├── blackjack/ # Blackjack engine, hand logic, dealer AI
│   └── slots/     # Slots engine, reel weighting, jackpots
├── models/        # Player, stats, mode data structures
├── network/       # Online client stub (future)
├── security/      # Security architecture notes (future)
├── services/      # Save service abstraction
├── storage/       # Local JSON + SQLite backends
├── ui/
│   ├── animations/# Spinner & reel animation frames
│   ├── components/# Header, footer, modal, notifications
│   ├── screens/   # Dashboard, blackjack, slots, online
│   └── theme.rs   # Color palette & style system
└── utils/         # Chip formatting, RNG policy, error types
```

### Tech Stack

| Component | Library |
|-----------|---------|
| UI Framework | [ratatui](https://github.com/ratatui-org/ratatui) 0.27 |
| Terminal Backend | [crossterm](https://github.com/crossterm-rs/crossterm) 0.27 |
| Async Runtime | [tokio](https://tokio.rs/) 1.38 |
| Serialization | [serde](https://serde.rs/) + serde_json |
| Database | [rusqlite](https://github.com/rusqlite/rusqlite) 0.31 *(future)* |
| Error Handling | [anyhow](https://github.com/dtolnay/anyhow) + [thiserror](https://github.com/dtolnay/thiserror) |

### Design Principles

- **Offline-first** — all game logic runs locally; online mode is scaffolded for future use
- **Atomic saves** — writes to `.tmp` then renames, preventing corruption
- **Server-authoritative ready** — security architecture designed so online mode can be added without client trust

---

## Save Data

Your progress is saved automatically to:

```
~/.local/share/prox-casino/offline_save.json
```

Tracked stats include:
- Chips balance & lifetime earnings
- Games played, win/loss/push counts
- Streaks (win, loss, push)
- Bust count, double-down count
- Slots: spins, biggest win, jackpot hits, symbol appearances

---

## Configuration

Key constants in `src/core/config.rs`:

| Constant | Default | Description |
|----------|---------|-------------|
| `START_CHIPS` | 2,000,000 | Initial chip balance |
| `DAILY_BONUS` | 50,000 | Daily login bonus |
| `TICK_RATE_MS` | 120 | Game loop tick (ms) |
| `SLOTS_SPIN_FRAMES` | 35 | Slot reel animation length |
| `BJ_DEALER_TICK_DELAY` | 8 | Dealer draw animation speed |

---

## Building

```bash
# Development
cargo check
cargo clippy
cargo test

# Release (optimized, stripped, LTO)
cargo build --release
```

The release profile enables LTO, single codegen unit, and binary stripping for a compact ~1.7MB binary.

---

## License

[MIT](LICENSE)

---

<p align="center">
  <sub>Built with ♠ ♥ ♦ ♣ and Rust</sub>
</p>
