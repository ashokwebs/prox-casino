# prox casino

a terminal casino thing in rust. blackjack + slots, no real money involved.

## what you can do

- play blackjack (hit, stand, double, split, basic dealer ai)
- play slots (a few machine types, weighted reels, jackpots)
- track stats across sessions
- play offline (your save file is just json, feel free to edit it)
- online mode is there as a placeholder, does nothing yet

## keys

**global:**
- `1` - dashboard
- `2` - blackjack
- `3` - slots
- `4` - online (placeholder)
- `q` - quit

**blackjack:**
- `←` `→` - adjust bet
- `space` - deal
- `h` - hit
- `s` - stand
- `d` - double
- `p` - split
- `r` - rules

**slots:**
- `←` `→` - adjust bet
- `space` - spin
- `a` - auto 10 spins
- `m` - change machine

## run it

```
cargo run --release
```

debug with logging:

```
RUST_LOG=info cargo run
```

## project layout

```
src/
  app/          main app logic, key handling
  core/         config constants
  games/        blackjack and slots
  models/       player, stats, etc
  network/      online placeholder
  security/     docs for future security
  services/     save/load
  storage/      json persistence, sqlite stub
  ui/
    screens/    actual screen rendering
    components/ reusable ui pieces
    animations/ spinner helpers
    theme.rs    colors and styles
  utils/        errors, formatting
```

## license

mit
