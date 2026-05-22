# PROX CASINO

**Version 0.2.0** — A premium cross-platform terminal casino simulator.

PROX CASINO is a production-grade, cyberpunk-themed social casino built in Rust with `ratatui` + `crossterm`. It uses virtual chips only and is designed for competitive/social gameplay patterns.

## Disclaimer

This project does **not** implement real-money gambling, payments, or cash-out flows.

## Features

- **Polished TUI** with cyberpunk aesthetic, rounded borders, colored suits, and Unicode symbols
- **Two isolated modes**:
  - Offline mode (fully local, modifiable save data, no anti-cheat)
  - Online mode (placeholder architecture for future server-authoritative gameplay)
- **Offline economy**:
  - Start chips: `2,000,000`
  - Daily bonus: `50,000` chips
- **Games**:
  - **Blackjack** — hit, stand, double down, dealer AI, streak tracking, blackjack payouts (3:2)
  - **Slots** — animated spinning reels, weighted rarity, payout multipliers, jackpot flashing
- **Save system** — atomic JSON saves with corruption-safe writes under user data directory
- **Statistics** — per-game win/loss/push tracking, streak tracking, biggest win, total bet
- **Animation framework** — slot reel spinner, card dealing animation counter
- **Theme system** — centralized `ProxTheme` for consistent colors and styles
- **Future-ready architecture**:
  - Networking stubs (TCP/WebSocket protocol types)
  - Security architecture documentation
  - SQLite storage scaffold
  - Plugin/theming system foundations

## Tech Stack

- Rust stable (edition 2021)
- `ratatui` + `crossterm` — terminal UI framework
- `tokio` — async runtime
- `serde` / `serde_json` — serialization
- `chrono` — date/time for daily bonuses
- `rand` — RNG for card shuffling and slot outcomes
- `rusqlite` — SQLite scaffolding
- `tracing` / `tracing-subscriber` — structured logging
- `uuid` — future session/player IDs
- `anyhow` / `thiserror` — error handling

## Project Structure

```
src/
  app/          # App state machine, input handling, flow control
  core/         # Configuration constants
  games/        # Modular game engines (blackjack, slots)
  models/       # Domain models (player, stats, mode, game)
  network/      # Online client/protocol placeholders
  security/     # Future security architecture documentation
  services/     # Save service layer
  storage/      # Local JSON persistence, SQLite stub
  ui/
    screens/    # Screen renderers (dashboard, blackjack, slots, online)
    components/ # Reusable UI components (header, footer, notifications, modal)
    animations/ # Animation helpers
    theme.rs    # Centralized theming system
  utils/        # Shared error types and RNG policy helpers
```

## Controls

| Key | Action |
|-----|--------|
| `q` | Quit |
| `Tab` | Cycle views |
| `1` | Switch to Offline mode |
| `2` | Switch to Online mode |
| `d` | Claim offline daily bonus |

### Blackjack

| Key | Action |
|-----|--------|
| `←/→` | Adjust bet |
| `n` | Deal new round |
| `h` | Hit |
| `s` | Stand |
| `d` | Double down |

### Slots

| Key | Action |
|-----|--------|
| `←/→` | Adjust bet |
| `Space` | Spin |

## Build & Run

```bash
cargo build --release
cargo run --release
```

Debug build with logging:

```bash
RUST_LOG=info cargo run
```

## Cross-Platform

Targets Windows, macOS, and Linux. Behavior depends on terminal Unicode/color support.

## Architecture Principles

- **Offline mode**: Client-authoritative. Users may edit save files freely.
- **Online mode**: Server-authoritative planned. Client must never be trusted.
- **Modular games**: Each game is an independent engine with its own state and rendering.
- **Future expansion**: Networking, security, theming, and plugin systems are scaffolded.

## License

MIT — PROX CASINO Contributors
